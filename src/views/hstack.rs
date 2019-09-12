use crate::views::view::*;
use crate::widgets::widget::Widget;

pub struct HStack<T> {
    data: ViewData<T>,
}

impl<T> HStack<T> {
    pub fn new(components: Vec<WidgetOrView<T>>) -> Self {
        HStack {
            data:
                ViewData {
                    component_map: std::collections::HashMap::new(),
                    components: components,
                    view_width: 0,
                    view_height: 0,
                    fixed_size: false,
                    padding:
                        Padding {
                            left: 10,
                            right: 10,
                            top: 10,
                            bottom: 10,
                        },
                    alignment: Alignment::Center,
                }
        }
    }
}

impl<T> View<T> for HStack<T> {
    fn init(&mut self) {

    }

    fn widgets_mut(&mut self) -> Vec<&mut Box<dyn Widget<T>>> {
        let mut widgets = Vec::new();

        for component in &mut self.data.components {
            match component {
                WidgetOrView::Widget(widget) => {
                    widgets.push(widget);
                }
                _ => {}
            }
        }

        widgets
    }

    fn widgets(&self) -> Vec<&Box<dyn Widget<T>>> {
        let mut widgets = Vec::new();

        for component in &self.data.components {
            match component {
                WidgetOrView::Widget(widget) => {
                    widgets.push(widget);
                }
                _ => {}
            }
        }

        widgets
    }

    fn fixed_width(mut self, width: u32) -> Self {
        self.data.view_width = width;
        self
    }

    fn fixed_height(mut self, height: u32) -> Self {
        self.data.view_height = height;
        self
    }

    fn fixed_size(mut self, width: u32, height: u32) -> Self {
        self.data.view_width = width;
        self.data.view_height = height;
        self
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        // Translate all components by the same amound
        for component in &mut self.data.components {
            match component {
                WidgetOrView::Widget(widget) => {
                    widget.translate(dx, dy);
                }
                WidgetOrView::View(view) => {
                    view.translate(dx, dy);
                }
            }
        }
    }

    fn align(&mut self, alignment: Alignment) {
        let draw_width = self.draw_width();

        match alignment {
            Alignment::Center => {
                // Distance from HStack's center to the view center
                let x_offset = (self.draw_width() / 2) - (self.data.view_width / 2);

                for component in &mut self.data.components {
                    match component {
                        WidgetOrView::Widget(widget) => {
                            widget.translate(x_offset as i32 + widget.rect().x(), 0);
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                // TODO: Implement the rest
            }
        }
    }

    fn draw_width(&self) -> u32 {
        let mut width = 0u32;

        for component in &self.data.components {
            match component {
                WidgetOrView::Widget(widget) => {
                    width += widget.draw_width();
                }
                _ => {}
            }
        }

        width + self.data.padding.left + self.data.padding.right
    }

    fn draw_height(&self) -> u32 {
        let mut max_height = 0u32;

        for component in &self.data.components {
            match component {
                WidgetOrView::Widget(widget) => {
                    if widget.draw_height() > max_height {
                        max_height = widget.draw_height();
                    }
                }
                _ => {}
            }
        }

        max_height + self.data.padding.top + self.data.padding.bottom
    }
}

impl<T> ViewComponent<T> for HStack<T> where T: 'static{
    fn as_component2(self) -> WidgetOrView<T> {
        WidgetOrView::View(Box::new(self))
    }
}

#[macro_export]
macro_rules! HStack2 {
    ( $($x:expr), + ) => {
        {
            let mut components = Vec::new();
            // let mut vstack = VStack::new(components);
            let default_padding = 10;

            let mut current_x = 0;

            // FIXME: Replace this with string in widget.rs
            let mut current_id = 0;
            
            $(
                let mut component = $x.as_component();

                match &mut component {
                    // FIXME: Placement needs to occur in the init function
                    WidgetOrView::Widget(widget) => {
                        widget.assign_id(current_id);
                        current_id += 1;

                        // TODO: Account for padding here?
                        widget.place(current_x, 0);

                        current_x += widget.rect().width() as i32 + default_padding;
                    }

                    WidgetOrView::View(subview) => {
                        for widget in subview.widgets_mut() {
                            widget.translate(current_x, 0);
                        }
                        current_x += subview.draw_width() as i32;
                    }
                }

                components.push(component);
                
            )+

            HStack::new(components)
        }
    };
}