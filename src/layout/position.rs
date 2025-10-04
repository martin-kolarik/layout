use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::{
    Axis, Style,
    dimension::Dim,
    unit::{Unit, sub_unit},
};

use super::dimension::FlexDim;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offset {
    pub x: Unit,
    pub y: Unit,
}

impl Offset {
    pub fn new(x: impl Into<Unit>, y: impl Into<Unit>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn zero() -> Self {
        Self {
            x: Unit::zero(),
            y: Unit::zero(),
        }
    }

    pub fn x_advance(&mut self, amount: impl Into<Unit>) {
        self.x += amount.into();
    }

    pub fn y_advance(&mut self, amount: impl Into<Unit>) {
        self.y += amount.into();
    }
}

impl Add<&Offset> for Offset {
    type Output = Offset;

    fn add(self, rhs: &Offset) -> Self::Output {
        Offset {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Offset> for &Offset {
    type Output = Offset;

    fn add(self, rhs: &Offset) -> Self::Output {
        Offset {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Offset {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<&Offset> for Offset {
    type Output = Offset;

    fn sub(self, rhs: &Offset) -> Self::Output {
        Offset {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<&Offset> for &Offset {
    type Output = Offset;

    fn sub(self, rhs: &Offset) -> Self::Output {
        Offset {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Offset {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Size {
    width: FlexDim,
    height: FlexDim,
    depth: Option<Unit>,
}

impl Debug for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Size")
            .field("w", &self.width)
            .field("h", &self.height)
            .field("d", &format!("{:?}", self.depth))
            .finish()
    }
}

impl Size {
    pub const NONE: Self = Self {
        width: FlexDim::none(),
        height: FlexDim::none(),
        depth: None,
    };

    pub fn none() -> Self {
        Self::NONE.clone()
    }

    pub const fn content() -> Self {
        Self {
            width: FlexDim::content(),
            height: FlexDim::content(),
            depth: None,
        }
    }

    pub fn fixed(width: impl Into<Unit>, height: impl Into<Unit>) -> Self {
        Self {
            width: FlexDim::fixed(width),
            height: FlexDim::fixed(height),
            depth: None,
        }
    }

    pub fn fixed_depth(
        width: impl Into<Unit>,
        height: impl Into<Unit>,
        depth: impl Into<Unit>,
    ) -> Self {
        Self {
            width: FlexDim::fixed(width),
            height: FlexDim::fixed(height),
            depth: Some(depth.into()),
        }
    }

    pub fn zero() -> Self {
        Self {
            width: Unit::zero().into(),
            height: Unit::zero().into(),
            depth: None,
        }
    }

    pub fn apply_style(&mut self, axis: Axis, style: &Style) {
        self.width.complete_with_style(
            style.width(),
            style.min_width(),
            style.max_width(),
            if matches!(axis, Axis::Horizontal) {
                style.grow()
            } else {
                None
            },
            if matches!(axis, Axis::Horizontal) {
                style.shrink()
            } else {
                None
            },
        );
        self.height.complete_with_style(
            style.height(),
            style.min_height(),
            style.max_height(),
            if matches!(axis, Axis::Vertical) {
                style.grow()
            } else {
                None
            },
            if matches!(axis, Axis::Vertical) {
                style.shrink()
            } else {
                None
            },
        );
    }

    pub fn x_dim(&self) -> &FlexDim {
        &self.width
    }

    pub fn x_dim_mut(&mut self) -> &mut FlexDim {
        &mut self.width
    }

    pub fn y_dim(&self) -> &FlexDim {
        &self.height
    }

    pub fn y_dim_mut(&mut self) -> &mut FlexDim {
        &mut self.height
    }

    pub fn width_ref(&self) -> &Dim {
        &self.width.basis
    }

    pub fn width(&self) -> Unit {
        self.width.size()
    }

    pub fn set_width(&mut self, width: impl Into<Dim>) {
        self.width.set_basis(width);
    }

    pub fn height_ref(&self) -> &Dim {
        &self.height.basis
    }

    pub fn height(&self) -> Unit {
        self.height.size()
    }

    pub fn set_height(&mut self, height: impl Into<Dim>) {
        self.height.set_basis(height);
    }

    pub fn depth(&self) -> Option<Unit> {
        self.depth
    }

    pub fn set_depth(&mut self, depth: Option<impl Into<Unit>>) {
        self.depth = depth.map(|depth| depth.into());
    }

    pub fn ascent(&self) -> Option<Unit> {
        self.depth
            .and_then(|depth| self.height.basis.size().map(|height| height - depth))
    }

    pub fn ascent_size(&self) -> Unit {
        sub_unit(self.height.basis.size(), self.depth).unwrap_or_default()
    }

    pub fn x_extend(&mut self, rhs: &Size, respect_baseline: bool) {
        self.width += &rhs.width;
        if respect_baseline {
            let ascent = self.ascent_size().max(rhs.ascent_size());
            self.depth = self.depth.max(rhs.depth);
            self.height = (self.depth().unwrap_or_default() + ascent).into();
        } else {
            self.height = self.height.max_of(&rhs.height);
        }
    }

    pub fn y_extend(&mut self, rhs: &Size, respect_baseline: bool) {
        self.width = self.width.max_of(&rhs.width);
        self.height += &rhs.height;
        if respect_baseline && self.depth.is_none() {
            self.depth = rhs.depth;
        }
    }
}

impl Add<&Size> for Offset {
    type Output = Offset;

    fn add(self, rhs: &Size) -> Self::Output {
        &self + rhs
    }
}

impl Add<&Size> for &Offset {
    type Output = Offset;

    fn add(self, rhs: &Size) -> Self::Output {
        Offset {
            x: self.x + rhs.width(),
            y: self.y + rhs.height(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Quad {
    top: Option<Unit>,
    left: Option<Unit>,
    bottom: Option<Unit>,
    right: Option<Unit>,
}

impl Quad {
    pub const fn empty() -> Self {
        Self {
            top: None,
            left: None,
            bottom: None,
            right: None,
        }
    }

    pub fn merge(&self, parent: &Self) -> Self {
        Self {
            top: self.top.or(parent.top),
            left: self.left.or(parent.left),
            bottom: self.bottom.or(parent.bottom),
            right: self.right.or(parent.right),
        }
    }

    pub fn square(unit: impl Into<Unit>) -> Self {
        let unit = unit.into();
        Self {
            top: Some(unit),
            left: Some(unit),
            bottom: Some(unit),
            right: Some(unit),
        }
    }

    pub fn h_v(horizontal: impl Into<Unit>, vertical: impl Into<Unit>) -> Self {
        let h = horizontal.into();
        let v = vertical.into();
        Self {
            top: Some(v),
            left: Some(h),
            bottom: Some(v),
            right: Some(h),
        }
    }

    pub fn with_top(mut self, top: impl Into<Unit>) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn with_left(mut self, left: impl Into<Unit>) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn with_bottom(mut self, bottom: impl Into<Unit>) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn with_right(mut self, right: impl Into<Unit>) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn top(&self) -> Option<Unit> {
        self.top
    }

    pub fn top_size(&self) -> Unit {
        self.top.unwrap_or_default()
    }

    pub fn left(&self) -> Option<Unit> {
        self.left
    }

    pub fn left_size(&self) -> Unit {
        self.left.unwrap_or_default()
    }

    pub fn bottom(&self) -> Option<Unit> {
        self.bottom
    }

    pub fn bottom_size(&self) -> Unit {
        self.bottom.unwrap_or_default()
    }

    pub fn right(&self) -> Option<Unit> {
        self.right
    }

    pub fn right_size(&self) -> Unit {
        self.right.unwrap_or_default()
    }

    pub fn width(&self) -> Unit {
        self.left_size() + self.right_size()
    }

    pub fn height(&self) -> Unit {
        self.top_size() + self.bottom_size()
    }

    pub fn offset(&self, offset: &Offset) -> Offset {
        Offset::new(offset.x + self.left_size(), offset.y + self.top_size())
    }

    pub fn narrow(&self, offset: Option<&mut Offset>, size: Option<&mut Size>) {
        if let Some(offset) = offset {
            offset.x_advance(self.left_size());
            offset.y_advance(self.top_size());
        }
        if let Some(size) = size {
            if size.x_dim().is_resolved() {
                let mut width = size.width_ref().clone();
                width.set_size(size.width() - self.width());
                size.set_width(width);
            }
            if size.y_dim().is_resolved() {
                let mut height = size.height_ref().clone();
                height.set_size(size.height() - self.height());
                size.set_height(height);

                if let Some(depth) = &mut size.depth {
                    *depth -= self.bottom_size();
                }
            }
        }
    }

    pub fn widen(&self, offset: Option<&mut Offset>, size: Option<&mut Size>) {
        if let Some(offset) = offset {
            offset.x_advance(Unit::zero() - self.left_size());
            offset.y_advance(Unit::zero() - self.top_size());
        }
        if let Some(size) = size {
            if size.x_dim().is_resolved() {
                let mut width = size.width_ref().clone();
                width.set_size(size.width() + self.width());
                size.set_width(width);
            }
            if size.y_dim().is_resolved() {
                let mut height = size.height_ref().clone();
                height.set_size(size.height() + self.height());
                size.set_height(height);

                if let Some(depth) = &mut size.depth {
                    *depth += self.bottom_size();
                }
            }
        }
    }
}

impl From<&Quad> for Quad {
    fn from(quad: &Quad) -> Self {
        quad.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{Offset, Size};

    #[test]
    fn offset_constructs() {
        let ofs = Offset::zero();
        assert_eq!(0, ofs.x.0);
        assert_eq!(0, ofs.y.0);
    }

    #[test]
    fn offset_advances() {
        let mut ofs = Offset::zero();
        ofs.x_advance(10);

        assert_eq!(10, ofs.x.0);
        assert_eq!(0, ofs.y.0);

        let mut ofs = Offset::zero();
        ofs.y_advance(10);

        assert_eq!(0, ofs.x.0);
        assert_eq!(10, ofs.y.0);
    }

    #[test]
    fn size_constructs() {
        let size = Size::zero();
        assert_eq!(0, size.width().0);
        assert_eq!(0, size.height().0);

        let mut size = Size::zero();
        size.set_width(10);
        assert_eq!(10, size.width().0);
        assert_eq!(0, size.height().0);

        let mut size = Size::zero();
        size.set_height(10);
        assert_eq!(0, size.width().0);
        assert_eq!(10, size.height().0);

        let mut size = Size::content();
        assert!(size.width.is_content());
        assert!(size.height.is_content());

        size.set_depth(Some(2));
        assert_eq!(0, size.width().0);
        assert_eq!(0, size.height().0);
        assert_eq!(-2, size.ascent_size().0);
    }

    #[test]
    fn size_mut_works() {
        let mut size = Size::zero();
        size.x_dim_mut().set_basis(10);
        assert_eq!(10, size.width().0);
        assert_eq!(0, size.height().0);

        let mut size = Size::zero();
        size.y_dim_mut().set_basis(10);
        assert_eq!(0, size.width().0);
        assert_eq!(10, size.height().0);
    }

    #[test]
    fn depth_and_ascent() {
        let mut size = Size::content();
        size.set_height(10);
        size.set_depth(Some(2));

        assert_eq!(10, size.height().0);
        assert_eq!(8, size.ascent_size().0);
    }

    #[test]
    fn it_extends() {
        let size1 = Size::fixed_depth(10, 12, 2);
        let size2 = Size::fixed_depth(20, 12, 3);

        let mut size = size1.clone();
        size.x_extend(&size2, true);
        assert_eq!(30, size.width().0);
        assert_eq!(13, size.height().0);
        assert_eq!(10, size.ascent_size().0);

        let mut size = size2.clone();
        size.x_extend(&size1, true);
        assert_eq!(30, size.width().0);
        assert_eq!(13, size.height().0);
        assert_eq!(10, size.ascent_size().0);

        let mut size = size1.clone();
        size.y_extend(&size2, true);
        assert_eq!(20, size.width().0);
        assert_eq!(24, size.height().0);
        assert_eq!(22, size.ascent_size().0);

        let mut size = size2;
        size.y_extend(&size1, true);
        assert_eq!(20, size.width().0);
        assert_eq!(24, size.height().0);
        assert_eq!(21, size.ascent_size().0);
    }
}
