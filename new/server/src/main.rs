mod artnet_output_socket;
mod effects;
mod poofer_bus_port;
mod preview;

use iced::{
    ContentFit, Element, Length, Settings, Size, Subscription, Task, Theme, application, time,
    window,
};
use poofer_bus_port::PooferBusPort;
use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    time::{Duration, Instant},
};

use artnet_output_socket::ArtnetOutputSocket;
use effects::{Effect, get_effects};

const ARTNET_FRAME_OUTPUT_PERIOD: usize = 2;

fn main() -> iced::Result {
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
    effect_start: Instant,
    preview: preview::Preview,
    current_effect: usize,
    artnet_socket: ArtnetOutputSocket,
    all_effects: Vec<Box<dyn Effect>>,
    artnet_output_enabled: bool,
    poofer_output_enabled: bool,
    artnet_output_frame_count: usize,
    available_serial_ports: Vec<String>,
    poofer_port: Option<PooferBusPort>,
}

#[allow(clippy::enum_variant_names)]
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Message {
    SetWindowSize(Size),
    Tick(Instant),
    ArtnetOutputCheckboxPressed,
    PooferOutputCheckboxPressed,
    SelectEffect(usize),
    SelectSerialPort(String),
}
impl App {
    fn new() -> (Self, Task<Message>) {
        (
            App {
                main_window_size: Size::new(0., 0.),
                start: Instant::now(),
                effect_start: Instant::now(),
                preview: preview::Preview::new(create_elders()),
                current_effect: 0,
                artnet_socket: ArtnetOutputSocket::new(),
                all_effects: get_effects(),
                artnet_output_enabled: true,
                poofer_output_enabled: false,
                artnet_output_frame_count: 0,
                available_serial_ports: PooferBusPort::available_ports(),
                poofer_port: None,
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
                self.all_effects[self.current_effect].render(
                    &mut self.preview.0,
                    now - self.start,
                    now - self.effect_start,
                );

                if self.artnet_output_enabled {
                    if self.artnet_output_frame_count == 0 {
                        self.artnet_socket.output(&self.preview.0);
                    }
                    self.artnet_output_frame_count =
                        (self.artnet_output_frame_count + 1) % ARTNET_FRAME_OUTPUT_PERIOD;
                }

                if let Some(poofer_port) = &mut self.poofer_port {
                    if self.poofer_output_enabled {
                        poofer_port.output(&mut self.preview.0);
                    }
                }

                self.preview.request_redraw();

                Task::none()
            }
            Message::ArtnetOutputCheckboxPressed => {
                self.artnet_output_enabled = !self.artnet_output_enabled;
                Task::none()
            }
            Message::PooferOutputCheckboxPressed => {
                self.poofer_output_enabled = !self.poofer_output_enabled;
                Task::none()
            }
            Message::SelectEffect(i) => {
                self.current_effect = i;
                self.effect_start = Instant::now();
                Task::none()
            }
            Message::SelectSerialPort(port_name) => {
                self.poofer_port = Some(PooferBusPort::new(&port_name));
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
            row![
                column(self.available_serial_ports.iter().map(|port_name| {
                    button(text(port_name))
                        .on_press(Message::SelectSerialPort(port_name.clone()))
                        .into()
                })),
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
            ],
            checkbox("Output LEDs", self.artnet_output_enabled)
                .on_toggle(|_| { Message::ArtnetOutputCheckboxPressed }),
            checkbox("Output Poofers", self.poofer_output_enabled)
                .on_toggle(|_| { Message::PooferOutputCheckboxPressed }),
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
    pub artnet_target_addr: SocketAddr,
    pub crane_light: Pixel,
    pub poofer_both: Poofer,
    pub poofer_wide: Poofer,
    pub poofer_narrow: Poofer,
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
    pub needs_to_send_command: bool,
    /// Multiple solenoids / relays which always will poof together
    pub relays: Vec<RelayAddress>,
}
impl Poofer {
    pub fn poof(&mut self, new_value_of_on: bool) {
        match (self.on, new_value_of_on) {
            (true, false) => {
                self.on = false;
                self.needs_to_send_command = true;
            }
            (false, true) => {
                self.on = true;
                self.needs_to_send_command = true;
            }
            _ => {}
        }
    }
}

/**
 * Board address is based on dip switches. LSB is dip switch 1, MSB is dip switch 5. Putting the
 * switch in the direction of the arrow means 0; against the opposite direction of the arrow means
 * 1.
 */
#[derive(Clone, Debug)]
pub struct RelayAddress {
    pub board_address: u8,
    pub poofer_address: u8,
}

pub struct ElderDefinition {
    pub artnet_target_ip_last_octet: u8,
    pub relay_wide: RelayAddress,
    pub relay_narrow: RelayAddress,
}

fn get_elder_defs() -> [ElderDefinition; 9] {
    [
        ElderDefinition {
            artnet_target_ip_last_octet: 91,
            relay_wide: RelayAddress {
                board_address: 1,
                poofer_address: 1,
            },
            relay_narrow: RelayAddress {
                board_address: 1,
                poofer_address: 2,
            },
        },
        ElderDefinition {
            artnet_target_ip_last_octet: 92,
            relay_wide: RelayAddress {
                board_address: 1,
                poofer_address: 3,
            },
            relay_narrow: RelayAddress {
                board_address: 1,
                poofer_address: 4,
            },
        },
        ElderDefinition {
            artnet_target_ip_last_octet: 93,
            relay_wide: RelayAddress {
                board_address: 1,
                poofer_address: 5,
            },
            relay_narrow: RelayAddress {
                board_address: 1,
                poofer_address: 6,
            },
        },
        ElderDefinition {
            artnet_target_ip_last_octet: 94,
            relay_wide: RelayAddress {
                board_address: 2,
                poofer_address: 1,
            },
            relay_narrow: RelayAddress {
                board_address: 2,
                poofer_address: 2,
            },
        },
        ElderDefinition {
            artnet_target_ip_last_octet: 95,
            relay_wide: RelayAddress {
                board_address: 2,
                poofer_address: 3,
            },
            relay_narrow: RelayAddress {
                board_address: 2,
                poofer_address: 4,
            },
        },
        ElderDefinition {
            artnet_target_ip_last_octet: 96,
            relay_wide: RelayAddress {
                board_address: 2,
                poofer_address: 5,
            },
            relay_narrow: RelayAddress {
                board_address: 2,
                poofer_address: 6,
            },
        },
        ElderDefinition {
            artnet_target_ip_last_octet: 97,
            relay_wide: RelayAddress {
                board_address: 3,
                poofer_address: 1,
            },
            relay_narrow: RelayAddress {
                board_address: 3,
                poofer_address: 2,
            },
        },
        ElderDefinition {
            artnet_target_ip_last_octet: 98,
            relay_wide: RelayAddress {
                board_address: 3,
                poofer_address: 3,
            },
            relay_narrow: RelayAddress {
                board_address: 3,
                poofer_address: 4,
            },
        },
        ElderDefinition {
            artnet_target_ip_last_octet: 99,
            relay_wide: RelayAddress {
                board_address: 3,
                poofer_address: 5,
            },
            relay_narrow: RelayAddress {
                board_address: 3,
                poofer_address: 6,
            },
        },
    ]
}

/**
 * We use -1 to 1 for both X and Y axes.
 */
fn create_elders() -> Vec<Elder> {
    let starting_theta = -std::f32::consts::FRAC_PI_2;
    let crane_light_radius: f32 = 0.5;
    let poofer_radius: f32 = 0.6;

    let elder_defs = get_elder_defs();
    let elder_count = elder_defs.len() as f32;
    elder_defs
        .into_iter()
        .enumerate()
        .map(
            |(
                i,
                ElderDefinition {
                    artnet_target_ip_last_octet,
                    relay_wide,
                    relay_narrow,
                },
            )| {
                let elder_theta = starting_theta + std::f32::consts::TAU * (i as f32) / elder_count;
                Elder {
                    artnet_target_addr: SocketAddrV4::new(
                        Ipv4Addr::new(169, 254, 9, artnet_target_ip_last_octet),
                        6454,
                    )
                    .into(),
                    crane_light: Pixel {
                        x: elder_theta.cos() * crane_light_radius,
                        y: elder_theta.sin() * crane_light_radius,
                        r: 0.,
                        g: 0.,
                        b: 0.,
                    },
                    poofer_both: Poofer {
                        x: elder_theta.cos() * poofer_radius,
                        y: elder_theta.sin() * poofer_radius,
                        on: false,
                        needs_to_send_command: false,
                        relays: vec![relay_wide.clone(), relay_narrow.clone()],
                    },
                    poofer_wide: Poofer {
                        x: elder_theta.cos() * poofer_radius,
                        y: elder_theta.sin() * poofer_radius,
                        on: false,
                        needs_to_send_command: false,
                        relays: vec![relay_wide],
                    },
                    poofer_narrow: Poofer {
                        x: elder_theta.cos() * poofer_radius,
                        y: elder_theta.sin() * poofer_radius,
                        on: false,
                        needs_to_send_command: false,
                        relays: vec![relay_narrow],
                    },
                }
            },
        )
        .collect::<Vec<Elder>>()
}
