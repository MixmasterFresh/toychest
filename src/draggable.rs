use std::hash::Hash;

use iced_native::{renderer, Element, Event, Widget};

pub struct DragBar<'a, Message, Renderer> {
    canvas: Element<'a, Message, Renderer>,
    _dragging: bool,
    _last_moved: iced::Point,
}

// enum DraggableMessage {
//     Drag(iced::Vector),
// }

impl<'a, Message, Renderer> DragBar<'a, Message, Renderer> {
    pub fn new(element: Element<'a, Message, Renderer>) -> Self {
        DragBar {
            canvas: element,
            _dragging: false,
            _last_moved: iced::Point::default(),
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
        _layout: iced_native::Layout<'_>,
        _cursor_position: iced::Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_native::Clipboard,
        _shell: &mut iced_native::Shell<'_, Message>,
    ) -> iced_native::event::Status {
        match event {
            // TODO: make the dragging work
            Event::Mouse(_mouse_event) => iced_native::event::Status::Ignored,
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

impl<'a, Message, Renderer> From<DragBar<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + renderer::Renderer,
    Message: 'a,
{
    fn from(dragbar: DragBar<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(dragbar)
    }
}
