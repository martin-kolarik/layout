mod axis;

use std::sync::Arc;

pub use axis::*;

pub(crate) mod dimension;

pub mod position;

pub mod unit;

pub(crate) mod children;

use crate::{BlockBox, Error, Filling, LayoutBox, Stroke, Style, Text, TextPosition, Wrap};

use self::{
    dimension::Dim,
    position::{Offset, Quad, Size},
    unit::{Fill, Unit},
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

#[derive(Default)]
pub struct NewPageOptions {
    must_be_in_page: Option<(Unit, Unit)>,
    margin: Option<Quad>,
    size: Option<Size>,
}

impl NewPageOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_break_if_not_room(mut self, offset: &Offset, size: &Size) -> Self {
        self.must_be_in_page = Some((offset.y, size.height()));
        self
    }

    pub fn with_margin(mut self, margin: Quad) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    pub fn must_be_in_page(&self) -> Option<(Unit, Unit)> {
        self.must_be_in_page
    }

    pub fn margin(&self) -> Option<&Quad> {
        self.margin.as_ref()
    }

    pub fn size(&self) -> Option<&Size> {
        self.size.as_ref()
    }
}

pub trait RenderContext: MeasureContext {
    fn new_page(&mut self, options: Option<NewPageOptions>) -> bool;

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
    fn bbox() -> BlockBox;

    fn hbox() -> LayoutBox;
    fn hfilling() -> Filling;

    fn hcbox(layout: impl Layout + 'static) -> LayoutBox {
        Self::hbox()
            .child(Self::hfill(1))
            .child(layout)
            .child(Self::hfill(1))
    }

    fn hfill(weight: impl Into<Fill>) -> Filling {
        Self::hfilling().grow(weight)
    }

    fn hspace(size: impl Into<Dim>) -> Filling {
        Self::hfilling().size(size)
    }

    fn hwrap() -> Wrap;

    fn vbox() -> LayoutBox;
    fn vfilling() -> Filling;

    fn vcbox(layout: impl Layout + 'static) -> LayoutBox {
        Self::vbox()
            .child(Self::vfill(1))
            .child(layout)
            .child(Self::vfill(1))
    }

    fn vfill(weight: impl Into<Fill>) -> Filling {
        Self::vfilling().grow(weight)
    }

    fn vspace(size: impl Into<Dim>) -> Filling {
        Self::vfilling().size(size)
    }

    fn vwrap() -> Wrap;

    fn text_str(text: &str) -> Text;
}

pub struct DefaultFactory;

impl DefaultFactory {
    pub fn text(text: impl ToString) -> Text {
        Text::new(text)
    }
}

impl Factory for DefaultFactory {
    fn bbox() -> BlockBox {
        BlockBox::new()
    }

    fn hbox() -> LayoutBox {
        LayoutBox::new(Axis::Horizontal)
    }

    fn hfilling() -> Filling {
        Filling::new(Axis::Horizontal)
    }

    fn hwrap() -> Wrap {
        Wrap::new(Axis::Horizontal)
    }

    fn vbox() -> LayoutBox {
        LayoutBox::new(Axis::Vertical)
    }

    fn vfilling() -> Filling {
        Filling::new(Axis::Vertical)
    }

    fn vwrap() -> Wrap {
        Wrap::new(Axis::Vertical)
    }

    fn text_str(text: &str) -> Text {
        Text::new(text)
    }
}
