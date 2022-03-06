mod axis;

pub use axis::*;

pub(crate) mod dimension;

pub mod position;

pub mod unit;

pub(crate) mod children;

use crate::{Error, Features, Filling, LayoutBox, Stroke, Style, Text, TextPosition, DecoratedBox};

use self::position::{Offset, Quad, Size};

pub trait Position {
    fn mark(&self) -> &str {
        ""
    }

    fn offset_ref(&self) -> &Offset;
    fn offset_mut(&mut self) -> &mut Offset;

    fn size_ref(&self) -> &Size;
    fn size_mut(&mut self) -> &mut Size;

    fn native_size(&self) -> Option<&Size> {
        Some(self.size_ref())
    }

    fn content_size(&self) -> Option<&Size> {
        None
    }
}

pub trait Styled {
    fn style_ref(&self) -> &Style;
    fn set_style(&mut self, style: Style);
    fn adopt_parent_style(&mut self, parent: &Style);
    fn override_style(&mut self, with: &Style);
}

#[allow(unused_variables)]
pub trait Layout: Send + Position + Styled {
    fn measure(&mut self, ctx: &mut dyn MeasureContext, size: Size) -> Result<(), Error> {
        Ok(())
    }

    fn lay_out(
        &mut self,
        ctx: &mut dyn MeasureContext,
        position: Offset,
        size: Size,
    ) -> Result<(), Error> {
        *self.offset_mut() = position;
        Ok(())
    }

    fn render(&self, ctx: &mut dyn RenderContext) -> Result<(), Error> {
        Ok(())
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &Box<dyn Layout>> + '_> {
        unreachable!()
    }
}

pub trait MeasureContext {
    fn style(&self) -> &Style;

    fn typeset(
        &mut self,
        style: &Style,
        text: &str,
        features: Option<&Features>,
    ) -> Result<TextPosition, Error>;
}

pub trait RenderContext: MeasureContext {
    fn new_page(&mut self);
    fn new_page_size(&mut self, margin: Quad, size: Size);

    fn debug_frame(&self, content_position: &Offset, size: &Size);

    fn line(&mut self, from: &Offset, to: &Offset, stroke: &Stroke);
    fn text(
        &mut self,
        content_position: &Offset,
        style: &Style,
        text: &TextPosition,
        position_is_baseline: bool,
    );
}

pub trait Factory {
    fn hbox() -> LayoutBox;
    fn hfill() -> Filling;
    fn hdbox() -> DecoratedBox;

    fn vbox() -> LayoutBox;
    fn vfill() -> Filling;
    fn vdbox() -> DecoratedBox;

    fn text_ref(text: &str) -> Text;
}

pub struct DefaultFactory;

impl DefaultFactory {
    pub fn text(text: impl Into<String>) -> Text {
        Text::new(text)
    }
}

impl Factory for DefaultFactory {
    fn hbox() -> LayoutBox {
        LayoutBox::new(Axis::Horizontal)
    }

    fn hfill() -> Filling {
        Filling::new(Axis::Horizontal)
    }

    fn hdbox() -> DecoratedBox {
        DecoratedBox::new(Axis::Horizontal)
    }

    fn vbox() -> LayoutBox {
        LayoutBox::new(Axis::Vertical)
    }

    fn vfill() -> Filling {
        Filling::new(Axis::Vertical)
    }

    fn vdbox() -> DecoratedBox {
        DecoratedBox::new(Axis::Vertical)
    }

    fn text_ref(text: &str) -> Text {
        Text::new(text)
    }
}

#[macro_export]
macro_rules! styled {
    ($layout:path) => {
        impl $crate::Styled for $layout {
            fn style_ref(&self) -> &$crate::Style {
                &self.style
            }

            fn set_style(&mut self, style: $crate::Style) {
                self.style = style;
            }

            fn adopt_parent_style(&mut self, parent: &$crate::Style) {
                self.style = self.style.merge(parent);
            }

            fn override_style(&mut self, with: &$crate::Style) {
                self.style = with.merge(&self.style);
            }
        }
    };
}
