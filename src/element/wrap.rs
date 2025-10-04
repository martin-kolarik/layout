use std::sync::Arc;

use crate::{
    Axis, Error, Layout, MeasureContext, Position, Style, Styled,
    position::{Offset, Size},
    unit::Unit,
};

pub struct Wrap {
    mark: Option<&'static str>,
    offset: Offset,
    size: Size,
    style: Arc<Style>,
}

impl Wrap {
    pub fn new(axis: Axis) -> Self {
        Self {
            mark: None,
            offset: Offset::zero(),
            size: if matches!(axis, Axis::Horizontal) {
                Size::fixed(Unit::infinity(), 0)
            } else {
                Size::fixed(0, Unit::infinity())
            },
            style: Style::new(),
        }
    }

    pub fn mark(mut self, mark: &'static str) -> Self {
        self.mark = Some(mark);
        self
    }
}

impl Position for Wrap {
    fn element(&self) -> &str {
        "Wrap"
    }

    fn mark(&self) -> &str {
        self.mark.unwrap_or_default()
    }

    fn offset(&self) -> &Offset {
        &self.offset
    }

    fn offset_mut(&mut self) -> &mut Offset {
        &mut self.offset
    }

    fn size(&self) -> &Size {
        &self.size
    }

    fn size_mut(&mut self) -> &mut Size {
        &mut self.size
    }

    fn size_after_wrap_ref(&self) -> &Size {
        &Size::NONE
    }

    fn size_after_lay_out(&self) -> Size {
        Size::NONE
    }

    fn content_size(&self) -> Option<&Size> {
        None
    }
}

impl Styled for Wrap {
    fn style_ref(&self) -> &Style {
        self.style.as_ref()
    }

    fn set_style(&mut self, style: Arc<Style>) {
        self.style = style;
    }
}

impl Layout for Wrap {
    fn lay_out(
        &mut self,
        _: &mut dyn MeasureContext,
        position: Offset,
        _: Size,
    ) -> Result<(), Error> {
        self.offset = position;
        Ok(())
    }
}
