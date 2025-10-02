use std::sync::Arc;

use crate::{
    Axis, Error, Layout, MeasureContext, Position, Style, Styled,
    position::{Offset, Size},
    unit::Unit,
};

pub struct LineBreak {
    mark: Option<&'static str>,
    offset: Offset,
    size_before_lay_out: Size,
    size: Size,
    style: Arc<Style>,
}

impl LineBreak {
    pub fn new(axis: Axis) -> Self {
        Self {
            mark: None,
            offset: Offset::zero(),
            size_before_lay_out: if matches!(axis, Axis::Horizontal) {
                Size::fixed(Unit::infinity(), 0)
            } else {
                Size::fixed(0, Unit::infinity())
            },
            size: Size::none(),
            style: Style::new(),
        }
    }

    pub fn mark(mut self, mark: &'static str) -> Self {
        self.mark = Some(mark);
        self
    }
}

impl Position for LineBreak {
    fn element(&self) -> &str {
        "Break"
    }

    fn mark(&self) -> &str {
        self.mark.unwrap_or_default()
    }

    fn offset_ref(&self) -> &Offset {
        &self.offset
    }

    fn offset_mut(&mut self) -> &mut Offset {
        &mut self.offset
    }

    fn size_ref(&self) -> &Size {
        &self.size_before_lay_out
    }

    fn size_mut(&mut self) -> &mut Size {
        &mut self.size_before_lay_out
    }

    fn size_after_lay_out(&self) -> Size {
        self.size.clone()
    }

    fn content_size(&self) -> Option<&Size> {
        None
    }
}

impl Styled for LineBreak {
    fn style_ref(&self) -> &Style {
        self.style.as_ref()
    }

    fn set_style(&mut self, style: Arc<Style>) {
        self.style = style;
    }
}

impl Layout for LineBreak {
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
