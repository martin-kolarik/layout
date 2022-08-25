mod axis;

use std::sync::Arc;

pub use axis::*;

pub(crate) mod dimension;

pub mod position;

pub mod unit;

pub(crate) mod children;

use crate::{Error, Filling, LayoutBox, Stroke, Style, Text, TextPosition};

use self::{
    dimension::DimAutoOrParent,
    position::{Offset, Quad, Size},
    unit::Fill,
};

pub trait Position {
    fn element(&self) -> &str;

    fn mark(&self) -> &str {
        ""
    }

    fn offset_ref(&self) -> &Offset;
    fn offset_mut(&mut self) -> &mut Offset;

    fn size_ref(&self) -> &Size;
    fn size_mut(&mut self) -> &mut Size;

    fn content_size(&self) -> Option<&Size> {
        None
    }
}

pub trait Styled {
    fn style_ref(&self) -> &Style;
    fn set_style(&mut self, style: Arc<Style>);

    fn adopt_parent_style(&mut self, parent: &Style) {
        self.set_style(self.style_ref().merge(parent));
    }

    fn override_style(&mut self, with: &Style) {
        self.set_style(with.merge(self.style_ref()));
    }
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
        Box::new([].iter())
    }
}

pub trait MeasureContext {
    fn style(&self) -> &Style;

    fn typeset(&mut self, style: &Style, text: &str) -> Result<TextPosition, Error>;
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
    fn hfilling() -> Filling;

    fn hfill(weight: impl Into<Fill>) -> Filling {
        Self::hfilling().grow(weight)
    }

    fn hspace(size: impl Into<DimAutoOrParent>) -> Filling {
        Self::hfilling().size(size)
    }

    fn vbox() -> LayoutBox;
    fn vfilling() -> Filling;

    fn vfill(weight: impl Into<Fill>) -> Filling {
        Self::vfilling().grow(weight)
    }

    fn vspace(size: impl Into<DimAutoOrParent>) -> Filling {
        Self::vfilling().size(size)
    }

    fn text_str(text: &str) -> Text;
}

pub struct DefaultFactory;

impl DefaultFactory {
    pub fn text(text: impl ToString) -> Text {
        Text::new(text)
    }
}

impl Factory for DefaultFactory {
    fn hbox() -> LayoutBox {
        LayoutBox::new(Axis::Horizontal)
    }

    fn hfilling() -> Filling {
        Filling::new(Axis::Horizontal)
    }

    fn vbox() -> LayoutBox {
        LayoutBox::new(Axis::Vertical)
    }

    fn vfilling() -> Filling {
        Filling::new(Axis::Vertical)
    }

    fn text_str(text: &str) -> Text {
        Text::new(text)
    }
}
