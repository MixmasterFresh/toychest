#![windows_subsystem = "windows"]
use iced::{
    alignment, button, container, executor, Alignment, Application, Button, Canvas, Checkbox,
    Column, Command, Container, Element, Length, Row, Settings, Subscription, Text,
};
use iced_native::{window, Event};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod draggable;
mod message;
mod pattern;
mod update;
use crate::message::Message;

pub fn main() -> iced::Result {
    Toychest::run(Settings {
        exit_on_close_request: false,
        window: iced::window::Settings {
            decorations: false,
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Default)]
struct Toychest {
    last: Vec<iced_native::Event>,
    enabled: bool,
    exit: button::State,
    should_exit: bool,
    running_update: bool,
}

struct Theme {}

impl button::StyleSheet for Theme {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(iced_native::Background::Color(
                iced_native::Color::from_rgb(0.5f32, 0f32, 0f32),
            )),
            shadow_offset: iced_native::Vector::new(2f32, 2f32),
            border_radius: 4f32,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        button::Style {
            shadow_offset: active.shadow_offset + iced_native::Vector::new(1f32, 1f32),
            ..active
        }
    }
}

impl container::StyleSheet for Theme {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(iced_native::Background::Color(
                iced_native::Color::from_rgb8(0xfb, 0xc6, 0x51),
            )),
            ..container::Style::default()
        }
    }
}

impl std::default::Default for Theme {
    fn default() -> Self {
        Self {}
    }
}

impl Application for Toychest {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Toychest, Command<Message>) {
        (
            Toychest::default(),
            Command::perform(update::attempt_update(), update::map_result_type),
        )
    }

    fn title(&self) -> String {
        String::from("Toychest")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NativeEvent(event) if self.enabled => {
                self.last.push(event);

                if self.last.len() > 5 {
                    let _ = self.last.remove(0);
                }
                Command::none()
            }
            Message::NativeEvent(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    self.should_exit = true;
                }
                Command::none()
            }
            Message::Toggled(enabled) => {
                self.enabled = enabled;
                Command::none()
            }
            Message::Exit => {
                self.should_exit = true;
                Command::none()
            }
            Message::UpdateEvent(updateMessage) => update::update(updateMessage),
            Message::Ignored => Command::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::NativeEvent)
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn view(&mut self) -> Element<Message> {
        let events = self
            .last
            .iter()
            .fold(Column::new().spacing(10), |column, event| {
                column.push(Text::new(format!("{:?}", event)).size(20))
            });

        let toggle = Checkbox::new(self.enabled, "Listen to runtime events", Message::Toggled);

        let exit = Button::new(
            &mut self.exit,
            Text::new("Exit")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .style(Theme::default())
        .width(Length::Units(100))
        .height(Length::Units(40))
        .padding(10)
        .on_press(Message::Exit);

        let bar = draggable::DragBar::new(
            Canvas::new(pattern::Pattern::default())
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
        );

        let content = Column::new()
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .spacing(20)
            .push(events)
            .push(toggle);

        let navbar = Row::new()
            .push(
                Row::new()
                    .padding(10)
                    .push(Text::new("Toychest").size(40))
                    .align_items(Alignment::Start),
            )
            .push(
                Row::new()
                    .width(Length::Fill)
                    .padding(10)
                    .spacing(10)
                    .align_items(Alignment::End)
                    .push(bar)
                    .push(exit),
            )
            .max_height(70);

        let real_content = Column::new().push(navbar).push(content).width(Length::Fill);

        Container::new(real_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(Theme::default())
            //.center_x()
            //.center_y()
            .into()
    }
}
