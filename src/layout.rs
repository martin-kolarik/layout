mod axis;

use std::sync::Arc;

pub use axis::*;

pub(crate) mod dimension;

pub mod position;

pub mod unit;

pub(crate) mod children;

use crate::{Error, Stroke, Style, TextPosition};

use self::position::{Offset, Quad, Size};

pub trait Position {
    fn element(&self) -> &str;

    fn mark(&self) -> &str {
        ""
    }

    fn offset_ref(&self) -> &Offset;
    fn offset_mut(&mut self) -> &mut Offset;

    fn size_ref(&self) -> &Size;
    fn size_mut(&mut self) -> &mut Size;

    fn size_after_wrap_ref(&self) -> &Size {
        self.size_ref()
    }

    // TODO
    fn size_after_lay_out(&self) -> Size {
        self.size_ref().clone()
    }

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
    fn new_page(&mut self, options: Option<NewPageOptions>) -> bool;

    fn debug_frame(&mut self, content_position: &Offset, size: &Size);

    fn line(&mut self, from: &Offset, to: &Offset, stroke: &Stroke);
    fn text(
        &mut self,
        content_position: &Offset,
        style: &Style,
        text: &TextPosition,
        position_is_baseline: bool,
    );
}

#[derive(Default)]
pub struct NewPageOptions {
    pub margin: Option<Quad>,
    pub size: Option<Size>,
}

impl NewPageOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_margin(mut self, margin: Quad) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }
}
