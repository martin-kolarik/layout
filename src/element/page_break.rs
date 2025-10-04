use std::sync::Arc;

use crate::{
    Error, Layout, MeasureContext, Position, Style, Styled,
    position::{Offset, Size},
    unit::Unit,
};

pub struct PageBreak {
    mark: Option<&'static str>,
    offset: Offset,
    size: Size,
    style: Arc<Style>,
}

impl PageBreak {
    pub fn new() -> Self {
        Self {
            mark: None,
            offset: Offset::zero(),
            size: Size::fixed(0, Unit::infinity()),
            style: Style::new(),
        }
    }

    pub fn mark(mut self, mark: &'static str) -> Self {
        self.mark = Some(mark);
        self
    }
}

impl Position for PageBreak {
    fn element(&self) -> &str {
        "Wrap"
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

impl Styled for PageBreak {
    fn style_ref(&self) -> &Style {
        self.style.as_ref()
    }

    fn set_style(&mut self, style: Arc<Style>) {
        self.style = style;
    }
}

impl Layout for PageBreak {
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
