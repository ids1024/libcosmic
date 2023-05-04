// TODO Support positioning similar to GdkPopup, xdg_popup

use iced::futures::channel::mpsc::UnboundedSender;
use iced::widget::Container;

use iced_native::alignment;
use iced_native::event::{self, Event};
use iced_native::layout;
use iced_native::mouse;
use iced_native::overlay;
use iced_native::renderer;
use iced_native::widget::{Operation, Tree};
use iced_native::{
    Clipboard, Element, Layout, Length, Padding, Point, Rectangle, Shell, Size, Widget,
};
use std::{cell::RefCell, fmt::Debug, hash::Hash};

pub use iced_style::container::{Appearance, StyleSheet};

pub struct Popover<'a, Message, Renderer> {
    pub content: Element<'a, Message, Renderer>,
    pub popup: RefCell<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Popover<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content), Tree::new(&*self.popup.borrow())]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.content, &self.popup.borrow()])
    }

    fn width(&self) -> Length {
        self.content.as_widget().width()
    }

    fn height(&self) -> Length {
        self.content.as_widget().height()
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        self.content.as_widget().layout(renderer, limits)
    }

    fn operate(&self, tree: &mut Tree, layout: Layout<'_>, operation: &mut dyn Operation<Message>) {
        self.content
            .as_widget()
            .operate(&mut tree.children[0], layout, operation)
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        renderer_style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            renderer_style,
            layout,
            cursor_position,
            viewport,
        )
    }

    fn overlay<'b>(
        &'b self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        let bounds = layout.bounds();
        let position = Point::new(bounds.x, bounds.y);

        // XXX needed to use RefCell to get &mut for popup element
        Some(overlay::Element::new(
            position,
            Box::new(Overlay {
                tree: &mut tree.children[1],
                content: &self.popup,
            }),
        ))
    }
}

impl<'a, Message, Renderer> From<Popover<'a, Message, Renderer>> for Element<'a, Message, Renderer> 
where
    Message: 'static,
    Renderer: iced_native::Renderer + 'static,
    Renderer::Theme: StyleSheet,
{
    fn from(popover: Popover<'a, Message, Renderer>) -> Self {
        Self::new(popover)
    }
}

struct Overlay<'a, 'b, Message, Renderer> {
    tree: &'a mut Tree,
    content: &'a RefCell<Element<'b, Message, Renderer>>,
}

impl<'a, 'b, Message, Renderer> overlay::Overlay<Message, Renderer>
    for Overlay<'a, 'b, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    fn layout(&self, renderer: &Renderer, bounds: Size, position: Point) -> layout::Node {
        // TODO handle position
        let limits = layout::Limits::new(bounds, bounds);
        self.content.borrow().as_widget().layout(renderer, &limits)
    }

    fn operate(&mut self, layout: Layout<'_>, operation: &mut dyn Operation<Message>) {
        self.content
            .borrow()
            .as_widget()
            .operate(self.tree, layout, operation)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        self.content.borrow_mut().as_widget_mut().on_event(
            self.tree,
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.borrow().as_widget().mouse_interaction(
            self.tree,
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
    ) {
        let bounds = layout.bounds();
        self.content.borrow().as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            &bounds,
        )
    }
}
