use iced::{
    canvas::{self},
    Point,
};

use crate::Message;

#[derive(Default)]
pub struct Pattern {
    cache: canvas::Cache,
}

impl canvas::Program<Message> for Pattern {
    fn draw(&self, bounds: iced::Rectangle, _cursor: canvas::Cursor) -> Vec<canvas::Geometry> {
        let content = self.cache.draw(bounds.size(), |frame| {
            for x in (0..bounds.width as i64).step_by(10) {
                for y in (0..bounds.height as i64).step_by(10) {
                    frame.fill_rectangle(
                        Point::new(x as f32, y as f32),
                        iced::Size {
                            width: 2.0,
                            height: 2.0,
                        },
                        iced_native::Color::from_rgb8(0x31, 0x2f, 0x28),
                    );

                    frame.fill_rectangle(
                        Point::new(x as f32 + 5.0, y as f32 + 5.0),
                        iced::Size {
                            width: 2.0,
                            height: 2.0,
                        },
                        iced_native::Color::from_rgb8(0x31, 0x2f, 0x28),
                    );
                }
            }
        });
        vec![content]
    }

    fn update(
        &mut self,
        _event: canvas::Event,
        _bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        (canvas::event::Status::Ignored, None)
    }

    fn mouse_interaction(
        &self,
        _bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> iced_native::mouse::Interaction {
        iced_native::mouse::Interaction::default()
    }
}
