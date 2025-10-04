use std::ops::AddAssign;

use crate::unit::Unit;

use super::{
    dimension::FlexDim,
    position::{Offset, Size},
};

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    pub fn cross(&self) -> Self {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal,
        }
    }

    pub fn select(&self, horizontal: Unit, vertical: Unit) -> Unit {
        match self {
            Axis::Horizontal => horizontal,
            Axis::Vertical => vertical,
        }
    }

    pub fn offset(&self, offset: &Offset) -> Unit {
        match self {
            Axis::Horizontal => offset.x,
            Axis::Vertical => offset.y,
        }
    }

    pub fn set_offset(&self, offset: &mut Offset, value: Unit) {
        match self {
            Axis::Horizontal => offset.x = value,
            Axis::Vertical => offset.y = value,
        }
    }

    pub fn advance_dim(&self, offset: &Offset, amount: impl Into<Unit>) -> Offset {
        let mut offset = offset.clone();
        match self {
            Axis::Horizontal => offset.x_advance(amount),
            Axis::Vertical => offset.y_advance(amount),
        }
        offset
    }

    pub fn dim<'s>(&self, size: &'s Size) -> &'s FlexDim {
        match self {
            Axis::Horizontal => &size.width,
            Axis::Vertical => &size.height,
        }
    }

    pub fn dim_mut<'s>(&self, size: &'s mut Size) -> &'s mut FlexDim {
        match self {
            Axis::Horizontal => &mut size.width,
            Axis::Vertical => &mut size.height,
        }
    }

    pub fn base_size(&self, size: &Size) -> Unit {
        match self {
            Axis::Horizontal => size.base_width(),
            Axis::Vertical => size.base_height(),
        }
    }

    pub fn extend_dim(&self, size: &Size, amount: Unit) -> Size {
        let mut size = size.clone();
        self.dim_mut(&mut size).add_assign(amount);
        size
    }

    pub fn extend_size(&self, size: &Size, amount: &Size, respect_baseline: bool) -> Size {
        let mut size = size.clone();
        match self {
            Axis::Horizontal => size.width_extend(amount, respect_baseline),
            Axis::Vertical => size.height_extend(amount, respect_baseline),
        }
        size
    }

    pub fn resolve_content_size(&self, size: &mut Size, inner_size: &Size, outer_room: Unit) {
        let dim = self.dim_mut(size);
        if dim.is_content_fixed() {
            dim.set_base(self.base_size(inner_size));
        } else {
            dim.set_base(dim.size_filled(outer_room));
        }
    }
}
