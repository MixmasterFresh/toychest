use std::hash::Hash;

use iced_native::{renderer, Element, Event, Widget};

pub struct DragBar<'a, Message, Renderer> {
    canvas: Element<'a, Message, Renderer>,
    dragging: bool,
    last_moved: iced::Point,
}

// enum DraggableMessage {
//     Drag(iced::Vector),
// }

impl<'a, Message, Renderer> DragBar<'a, Message, Renderer> {
    pub fn new(element: Element<'a, Message, Renderer>) -> Self {
        DragBar {
            canvas: element,
            dragging: false,
            last_moved: iced::Point::default(),
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for DragBar<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn width(&self) -> iced::Length {
        iced::Length::Fill
    }

    fn height(&self) -> iced::Length {
        iced::Length::Units(40)
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        self.canvas.layout(renderer, limits)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        style: &renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) {
        self.canvas
            .draw(renderer, style, layout, cursor_position, viewport);
    }

    fn mouse_interaction(
        &self,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        _viewport: &iced::Rectangle,
    ) -> iced_native::mouse::Interaction {
        if layout.bounds().contains(cursor_position) {
            iced_native::mouse::Interaction::Grab
        } else {
            iced_native::mouse::Interaction::Idle
        }
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        0.hash(state)
    }

    fn on_event(
        &mut self,
        event: iced_native::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_native::Clipboard,
        _shell: &mut iced_native::Shell<'_, Message>,
    ) -> iced_native::event::Status {
        match event {
            // TODO: make the dragging work
            Event::Mouse(mouse_event) => iced_native::event::Status::Ignored,
            _ => iced_native::event::Status::Ignored,
        }
    }

    fn overlay(
        &mut self,
        _layout: iced_native::Layout<'_>,
    ) -> Option<iced_native::overlay::Element<'_, Message, Renderer>> {
        None
    }
}

impl<'a, Message, Renderer> Into<Element<'a, Message, Renderer>> for DragBar<'a, Message, Renderer>
where
    Renderer: 'a + renderer::Renderer,
    Message: 'a,
{
    fn into(self) -> Element<'a, Message, Renderer> {
        Element::new(self)
    }
}
