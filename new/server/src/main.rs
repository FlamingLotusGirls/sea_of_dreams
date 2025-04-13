mod effects;
mod output_artnet;
mod preview;

use iced::{
    ContentFit, Element, Length, Settings, Size, Subscription, Task, Theme, application, time,
    window,
};
use serialport::{self, DataBits, SerialPort, SerialPortType, StopBits};
use std::{
    io::Write as _,
    time::{Duration, Instant},
};

use effects::{Effect, get_effect};
use output_artnet::OutputArtnetSocket;

fn main() -> iced::Result {
    match serialport::available_ports() {
        Err(e) => {
            eprintln!("Error listing serial ports:");
            eprintln!("{e:?}");
            ::std::process::exit(1);
        }
        Ok(mut available_ports) => {
            available_ports.sort_by_key(|key| key.port_name.clone());

            for port in &available_ports {
                println!("{}", port.port_name);

                match &port.port_type {
                    SerialPortType::UsbPort(info) => {
                        println!("        Type: USB");
                        println!("        VID: {:04x}", info.vid);
                        println!("        PID: {:04x}", info.pid);
                        println!(
                            "        Serial Number: {}",
                            info.serial_number.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "        Manufacturer: {}",
                            info.manufacturer.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "        Product: {}",
                            info.product.as_ref().map_or("", String::as_str)
                        );
                    }
                    SerialPortType::BluetoothPort => {
                        println!("        Type: Bluetooth");
                    }
                    SerialPortType::PciPort => {
                        println!("        Type: PCI");
                    }
                    SerialPortType::Unknown => {
                        println!("        Type: Unknown");
                    }
                }
            }

            if let Some(port) = available_ports.get(0) {
                let port = serialport::new(&port.port_name, 19200)
                    .stop_bits(StopBits::One)
                    .data_bits(DataBits::Eight)
                    .open()
                    .unwrap_or_else(|e| {
                        eprintln!("Failed to open port {}. Error: {}", port.port_name, e);
                        ::std::process::exit(1);
                    });

                return run(port);
            } else {
                eprintln!("Error getting first serial port.");
                ::std::process::exit(1);
            }
        }
    }
}

// Since we only support one art-net universe (512B), 170 is the maximum number of total pixels for now
const ELDER_COUNT: usize = 9;
const FRAME_OUTPUT_PERIOD: usize = 2;

fn run(mut port: Box<dyn SerialPort>) -> iced::Result {
    let command = "!0121.";
    match port.write(command.as_bytes()) {
        Ok(_) => {
            print!("{}", command);
            std::io::stdout().flush().unwrap();
        }
        Err(e) => eprintln!("{e:?}"),
    }
    println!();

    application("Haven Server", App::update, App::view)
        .theme(|_| Theme::Dark)
        .settings(Settings {
            antialiasing: true,
            ..Default::default()
        })
        .window(window::Settings {
            position: window::Position::SpecificWith(|window_size, monitor_dimens| {
                (0., (monitor_dimens.height / 2.) - (window_size.height / 2.)).into()
            }),
            ..Default::default()
        })
        .subscription(App::subscription)
        .run_with(App::new)
}

struct App {
    main_window_size: Size,
    start: Instant,
    preview: preview::Preview,
    current_effect: usize,
    output_socket: OutputArtnetSocket,
    all_effects: Vec<Box<dyn Effect>>,
    output_enabled: bool,
    output_frame_count: usize,
}

#[allow(clippy::enum_variant_names)]
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Message {
    SetWindowSize(Size),
    Tick(Instant),
    PressOutput,
    SelectEffect(usize),
}
impl App {
    fn new() -> (Self, Task<Message>) {
        (
            App {
                main_window_size: Size::new(0., 0.),
                start: Instant::now(),
                preview: preview::Preview::new(create_elders()),
                current_effect: 0,
                output_socket: OutputArtnetSocket::new(),
                all_effects: {
                    let mut effects: Vec<Box<dyn Effect>> = vec![];
                    let mut i = 0;
                    while let Some(effect) = get_effect(i) {
                        effects.push(effect);
                        i += 1;
                    }
                    effects
                },
                output_enabled: true,
                output_frame_count: 0,
            },
            window::get_latest()
                .and_then(window::get_size)
                .map(Message::SetWindowSize),
        )
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetWindowSize(size) => {
                self.main_window_size = size;
                Task::none()
            }
            Message::Tick(now) => {
                // Clear all pixels
                for pixel in self.preview.0.iter_mut() {
                    pixel.crane_light.r = 0.;
                    pixel.crane_light.g = 0.;
                    pixel.crane_light.b = 0.;
                }
                self.all_effects[self.current_effect].render(&mut self.preview.0, now - self.start);

                if self.output_enabled {
                    if self.output_frame_count == 0 {
                        self.output_socket.output(&self.preview.0);
                    }
                    self.output_frame_count = (self.output_frame_count + 1) % FRAME_OUTPUT_PERIOD;
                }

                self.preview.request_redraw();

                Task::none()
            }
            Message::PressOutput => {
                self.output_enabled = !self.output_enabled;
                Task::none()
            }
            Message::SelectEffect(i) => {
                self.current_effect = i;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        use iced::widget::{column, *};
        container(column![
            container(row(self.all_effects.iter().enumerate().map(
                |(i, effect)| (button(text(effect.name()))
                    .style(if i == self.current_effect {
                        button::primary
                    } else {
                        button::secondary
                    })
                    .on_press(Message::SelectEffect(i)))
                .into()
            ))),
            container(responsive(move |bounds| {
                let Size { width, height } = ContentFit::Contain.fit(Size::new(1., 1.), bounds);
                center(
                    canvas(&self.preview)
                        .width(Length::Fixed(width))
                        .height(Length::Fixed(height)),
                )
                .into()
            }))
            .width(Length::Fill)
            .height(Length::Fill),
            checkbox("Output", self.output_enabled).on_toggle(|_| { Message::PressOutput }),
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(10)).map(Message::Tick)
    }
}

#[derive(Clone, Debug)]
pub struct Elder {
    pub crane_light: Pixel,
    pub poofer: Poofer,
}

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Clone, Debug)]
pub struct Poofer {
    pub x: f32,
    pub y: f32,
    pub on: bool,
    /// Multiple solenoids / relays which always will poof together
    pub relays: Vec<RelayAddress>,
}

#[derive(Clone, Debug)]
pub struct RelayAddress {
    pub board_address: u8,
    pub poofer_address: u8,
}

/**
 * We use -1 to 1 for both X and Y axes.
 */
fn create_elders() -> Vec<Elder> {
    let mut elders = Vec::with_capacity(ELDER_COUNT);

    let starting_theta = -std::f32::consts::FRAC_PI_2;
    let crane_light_radius: f32 = 0.5;
    let poofer_radius: f32 = 0.6;

    let elder_count = ELDER_COUNT as f32;
    for i in 0..ELDER_COUNT {
        let i_u8 = i as u8;
        let elder_theta = starting_theta + std::f32::consts::TAU * (i as f32) / elder_count;
        elders.push(Elder {
            crane_light: Pixel {
                x: elder_theta.cos() * crane_light_radius,
                y: elder_theta.sin() * crane_light_radius,
                r: 0.,
                g: 0.,
                b: 0.,
            },
            poofer: Poofer {
                x: elder_theta.cos() * poofer_radius,
                y: elder_theta.sin() * poofer_radius,
                on: false,
                // We will start out poofers in pairs for each elder
                relays: vec![
                    RelayAddress {
                        board_address: 1 + i_u8 / 3,
                        poofer_address: 2 * (i_u8 % 3) + 1,
                    },
                    RelayAddress {
                        board_address: 1 + i_u8 / 3,
                        poofer_address: 2 * (i_u8 % 3) + 2,
                    },
                ],
            },
        });
    }

    elders
}
