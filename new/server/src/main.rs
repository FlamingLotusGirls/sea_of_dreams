mod artnet_output_socket;
mod effects;
mod mapping;
mod model;
mod poofer_bus_port;
mod preview;

use iced::{
    ContentFit, Element, Length, Settings, Size, Subscription, Task, Theme, application, time,
    window,
};
use model::create_elders;
use poofer_bus_port::PooferBusPort;
use std::time::{Duration, Instant};

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
    poofer_output_enabled_once: bool,
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
                poofer_output_enabled_once: false,
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
                    if self.poofer_output_enabled || self.poofer_output_enabled_once {
                        poofer_port.output(&mut self.preview.0);
                        self.poofer_output_enabled_once = false;
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
                if self.poofer_output_enabled {
                    self.turn_poofers_off();
                    self.poofer_output_enabled_once = true;
                    self.poofer_output_enabled = false;
                } else {
                    self.poofer_output_enabled = true;
                }
                Task::none()
            }
            Message::SelectEffect(i) => {
                self.turn_poofers_off();
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
        container(row![
            container(column(self.all_effects.iter().enumerate().map(
                |(i, effect)| {
                    (button(text(effect.name()))
                        .style(if i == self.current_effect {
                            button::primary
                        } else {
                            button::secondary
                        })
                        .on_press(Message::SelectEffect(i)))
                    .into()
                }
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
            column![
                checkbox("Output LEDs", self.artnet_output_enabled)
                    .on_toggle(|_| { Message::ArtnetOutputCheckboxPressed }),
                checkbox("Output Poofers", self.poofer_output_enabled)
                    .on_toggle(|_| { Message::PooferOutputCheckboxPressed }),
                column(self.available_serial_ports.iter().map(|port_name| {
                    button(text(port_name))
                        .on_press(Message::SelectSerialPort(port_name.clone()))
                        .into()
                })),
            ],
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(10)).map(Message::Tick)
    }

    fn turn_poofers_off(&mut self) {
        for elder in &mut self.preview.0 {
            elder.poofer_wide.poof(false);
            elder.poofer_narrow.poof(false);
        }
    }
}
