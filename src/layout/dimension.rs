use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use rtext::Apply;

use crate::unit::{Fill, FillPerMille, Unit, add_fill, sub_fill};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaybeDim {
    None,
    Fixed(Unit),
    Parent(FillPerMille, Option<Unit>),
}

impl MaybeDim {
    pub fn as_mut(&mut self) -> Option<&mut Unit> {
        match self {
            Self::None => None,
            Self::Fixed(unit) => Some(unit),
            Self::Parent(_, unit) => unit.as_mut(),
        }
    }

    pub const fn is_parented(&self) -> bool {
        matches!(self, Self::Parent(..))
    }

    pub const fn parent_fill(&self) -> FillPerMille {
        match self {
            Self::Parent(fill, _) => *fill,
            _ => FillPerMille::none(),
        }
    }

    pub fn size(&self) -> Option<Unit> {
        match self {
            Self::None => None,
            Self::Fixed(size) => Some(*size),
            Self::Parent(_, size) => *size,
        }
    }

    pub fn set_size(&mut self, unit: impl Into<Unit>) {
        *self = Self::Fixed(unit.into());
    }

    pub fn resolve(&mut self, size: Unit) {
        if let Self::Parent(fill, None) = self {
            *self = Self::Parent(*fill, Some(size));
        }
    }

    pub fn or(self, other: Self) -> Self {
        match &self {
            Self::None => other,
            _ => self,
        }
    }
}

impl<IU> From<IU> for MaybeDim
where
    IU: Into<Unit>,
{
    fn from(unit: IU) -> Self {
        Self::Fixed(unit.into())
    }
}

impl From<FillPerMille> for MaybeDim {
    fn from(fill: FillPerMille) -> Self {
        Self::Parent(fill, None)
    }
}

impl Add<&MaybeDim> for &MaybeDim {
    type Output = MaybeDim;

    fn add(self, rhs: &MaybeDim) -> Self::Output {
        match (self.size(), rhs.size()) {
            (None, None) => MaybeDim::None,
            (None, Some(_)) => *rhs,
            (Some(_), None) => *self,
            (Some(l), Some(r)) => MaybeDim::Fixed(l + r),
        }
    }
}

impl Sub<&MaybeDim> for &MaybeDim {
    type Output = MaybeDim;

    fn sub(self, rhs: &MaybeDim) -> Self::Output {
        match (self.size(), rhs.size()) {
            (None, None) => MaybeDim::None,
            (None, Some(r)) => MaybeDim::Fixed(Unit::zero() - r),
            (Some(_), None) => *self,
            (Some(l), Some(r)) => MaybeDim::Fixed(l - r),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dim {
    Content(Option<Unit>),
    Fixed(Unit),
    Parent(FillPerMille, Option<Unit>),
}

impl Dim {
    pub const fn content() -> Self {
        Self::Content(None)
    }

    pub const fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed(_))
    }

    pub const fn is_content(&self) -> bool {
        matches!(self, Self::Content(_))
    }

    pub const fn is_parented(&self) -> bool {
        matches!(self, Self::Parent(..))
    }

    pub const fn is_resolved(&self) -> bool {
        matches!(
            self,
            Self::Content(Some(_)) | Self::Fixed(_) | Self::Parent(_, Some(_))
        )
    }

    pub const fn parent_fill(&self) -> FillPerMille {
        match self {
            Self::Parent(fill, _) => *fill,
            _ => FillPerMille::none(),
        }
    }

    pub const fn size(&self) -> Option<Unit> {
        match self {
            Self::Content(size) => *size,
            Self::Fixed(size) => Some(*size),
            Self::Parent(_, size) => *size,
        }
    }

    pub fn set_size(&mut self, size: impl Into<Unit>) {
        let size = size.into();
        match self {
            Self::Parent(fill, _) => *self = Self::Parent(*fill, Some(size)),
            Self::Content(_) => *self = Self::Content(Some(size)),
            Self::Fixed(_) => *self = Self::Fixed(size),
        }
    }

    pub fn resolve(&mut self, size: impl Into<Unit>) {
        let size = size.into();
        match self {
            Self::Parent(fill, None) => *self = Self::Parent(*fill, Some(size)),
            Self::Content(None) => *self = Self::Content(Some(size)),
            _ => (),
        }
    }

    pub fn min_of(&self, r: &Self) -> Self {
        match (self.size(), r.size()) {
            (None, None) => Self::Content(None),
            (None, Some(_)) => *r,
            (Some(_), None) => *self,
            (Some(l), Some(r)) => Self::Fixed(l.min(r)),
        }
    }

    pub fn max_of(&self, r: &Self) -> Self {
        match (self.size(), r.size()) {
            (None, None) => Self::Content(None),
            (None, Some(_)) => *r,
            (Some(_), None) => *self,
            (Some(l), Some(r)) => Self::Fixed(l.max(r)),
        }
    }

    pub fn add(&self, r: &Self) -> Self {
        match (self.size(), r.size()) {
            (None, None) => Self::Content(None),
            (None, Some(_)) => *r,
            (Some(_), None) => *self,
            (Some(l), Some(r)) => Self::Fixed(l + r),
        }
    }

    pub fn sub(&self, r: &Self) -> Self {
        match (self.size(), r.size()) {
            (None, None) => Self::Content(None),
            (None, Some(r)) => Self::Fixed(Unit::zero() - r),
            (Some(_), None) => *self,
            (Some(l), Some(r)) => Self::Fixed(l - r),
        }
    }

    pub fn or(self, other: Self) -> Self {
        match &self {
            Self::Content(None) => other,
            _ => self,
        }
    }
}

impl<IU> From<IU> for Dim
where
    IU: Into<Unit>,
{
    fn from(unit: IU) -> Self {
        Self::Fixed(unit.into())
    }
}

impl From<FillPerMille> for Dim {
    fn from(fill: FillPerMille) -> Self {
        Self::Parent(fill, None)
    }
}

impl From<MaybeDim> for Dim {
    fn from(dim: MaybeDim) -> Self {
        match dim {
            MaybeDim::None => Self::Content(None),
            MaybeDim::Fixed(unit) => Self::Fixed(unit),
            MaybeDim::Parent(fill, unit) => Self::Parent(fill, unit),
        }
    }
}

impl Add<&Dim> for &Dim {
    type Output = Dim;

    fn add(self, rhs: &Dim) -> Self::Output {
        match (self.size(), rhs.size()) {
            (None, None) => Dim::content(),
            (None, Some(_)) => *rhs,
            (Some(_), None) => *self,
            (Some(l), Some(r)) => Dim::Fixed(l + r),
        }
    }
}

impl Sub<&Dim> for &Dim {
    type Output = Dim;

    fn sub(self, rhs: &Dim) -> Self::Output {
        match (self.size(), rhs.size()) {
            (None, None) => Dim::content(),
            (None, Some(r)) => Dim::Fixed(Unit::zero() - r),
            (Some(_), None) => *self,
            (Some(l), Some(r)) => Dim::Fixed(l - r),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct FlexDim {
    pub base: Dim,
    pub min: MaybeDim,
    pub max: MaybeDim,
    pub grow: Option<Fill>,
    pub shrink: Option<Fill>,
}

impl Debug for FlexDim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?}]{:?}+{:?}-{:?}[{:?}",
            self.min, self.base, self.grow, self.shrink, self.max,
        ))
    }
}

impl FlexDim {
    pub const fn none() -> Self {
        Self {
            base: Dim::Content(None),
            min: MaybeDim::None,
            max: MaybeDim::None,
            grow: None,
            shrink: None,
        }
    }

    pub const fn content() -> Self {
        Self {
            base: Dim::Content(None),
            min: MaybeDim::None,
            max: MaybeDim::None,
            grow: None,
            shrink: None,
        }
    }

    pub fn parented(fill: impl Into<FillPerMille>) -> Self {
        Self {
            base: Dim::Parent(fill.into(), None),
            min: MaybeDim::None,
            max: MaybeDim::None,
            grow: None,
            shrink: None,
        }
    }

    pub fn fixed(size: impl Into<Unit>) -> Self {
        size.into().into()
    }

    pub fn with_min(mut self, min: impl Into<MaybeDim>) -> Self {
        self.set_min(min);
        self
    }

    pub fn with_max(mut self, max: impl Into<MaybeDim>) -> Self {
        self.set_max(max);
        self
    }

    pub fn with_grow(mut self, fill: impl Into<Fill>) -> Self {
        self.set_grow(fill);
        self
    }

    pub fn with_shrink(mut self, fill: impl Into<Fill>) -> Self {
        self.set_shrink(fill);
        self
    }

    pub fn complete_with_style(
        &mut self,
        size: Dim,
        min: MaybeDim,
        max: MaybeDim,
        grow: Option<Fill>,
        shrink: Option<Fill>,
    ) {
        self.base = self.base.or(size);
        self.min = self.min.or(min);
        self.max = self.max.or(max);
        self.grow = self.grow.or(grow);
        self.shrink = self.shrink.or(shrink);
    }

    pub const fn is_fixed(&self) -> bool {
        self.base.is_fixed()
    }

    pub const fn is_parented(&self) -> bool {
        self.base.is_parented()
    }

    pub const fn is_content(&self) -> bool {
        self.base.is_content()
    }

    pub const fn is_content_fixed(&self) -> bool {
        self.base.is_content() && self.grow.is_none() && self.shrink.is_none()
    }

    pub const fn is_dyn(&self) -> bool {
        self.grow.is_some() || self.shrink.is_some()
    }

    pub const fn is_content_or_dyn(&self) -> bool {
        self.is_content() || self.is_dyn()
    }

    pub const fn is_resolved(&self) -> bool {
        self.base.is_resolved()
    }

    pub fn set_base(&mut self, base: impl Into<Dim>) {
        let mut base = base.into();
        if let (Some(base_size), Some(min)) = (base.size(), self.min.size()) {
            if base_size < min {
                base = self.min.into();
            }
        }
        if let (Some(base_size), Some(max)) = (base.size(), self.max.size()) {
            if base_size > max {
                base = self.max.into();
            }
        }
        self.base = base;
    }

    pub fn set_min(&mut self, min: impl Into<MaybeDim>) {
        let min = min.into();
        if let (Some(base), Some(min_size)) = (self.base.size(), min.size()) {
            if base < min_size {
                self.base = min.into()
            }
        }
        if let (Some(max_size), Some(min_size)) = (self.max.size(), min.size()) {
            if max_size < min_size {
                self.max = min;
            }
        }
        self.min = min;
    }

    pub fn set_max(&mut self, max: impl Into<MaybeDim>) {
        let max = max.into();
        if let (Some(base_size), Some(max_size)) = (self.base.size(), max.size()) {
            if base_size > max_size {
                self.base = max.into();
            }
        }
        if let (Some(min_size), Some(max_size)) = (self.min.size(), max.size()) {
            if min_size > max_size {
                self.min = max;
            }
        }
        self.max = max;
    }

    pub fn resolve_content(&mut self, content_size: impl Into<Unit>) {
        if self.base.is_content() {
            self.base.resolve(content_size.into());
        }
    }

    pub fn resolve_parented(&mut self, parent_size: impl Into<Unit>) {
        let parent = parent_size.into();

        if self.min.is_parented() {
            let min = parent * (self.min.parent_fill(), FillPerMille::mille());
            if matches!(self.max.size(), Some(max) if max < min) {
                self.max.resolve(min);
            }
            if matches!(self.base.size(), Some(base) if base < min) {
                self.base.resolve(min);
            }
            self.min.resolve(min);
        }

        if self.max.is_parented() {
            let max = parent * (self.max.parent_fill(), FillPerMille::mille());
            if matches!(self.min.size(), Some(min) if min > max) {
                self.min.resolve(max);
            }
            if matches!(self.base.size(), Some(base) if base > max) {
                self.base.resolve(max);
            }
            self.max.resolve(max);
        }

        if self.base.is_parented() {
            let base = parent * (self.base.parent_fill(), FillPerMille::mille());
            match self.min.size() {
                Some(min) if min > base => self.base.resolve(min),
                _ => (),
            }
            match self.max.size() {
                Some(max) if max < base => self.base.resolve(max),
                _ => (),
            }
            self.base.resolve(base);
        }
    }

    pub fn set_grow(&mut self, fill: impl Into<Fill>) {
        self.grow = Some(fill.into());
    }

    pub fn set_shrink(&mut self, fill: impl Into<Fill>) {
        self.shrink = Some(fill.into());
    }

    pub fn base_size(&self) -> Unit {
        self.base.size().unwrap_or_default()
    }

    pub fn size_available(&self, room: Unit) -> Unit {
        if self.is_content() {
            room.apply_then(self.max.size(), |room, max| room.min(max))
                .apply_then(self.min.size(), |room, min| room.max(min))
        } else {
            self.size_filled(room)
        }
    }

    pub fn size_filled(&self, room: Unit) -> Unit {
        let size = self.base_size();
        match size.cmp(&room) {
            Ordering::Less => match (self.max.size(), self.grow) {
                (_, None) => size,
                (None, Some(_)) => room,
                (Some(max), Some(_)) => room.min(max),
            },
            Ordering::Equal => size,
            Ordering::Greater => match (self.min.size(), self.shrink) {
                (_, None) => size,
                (None, Some(_)) => room,
                (Some(min), Some(_)) => room.max(min),
            },
        }
    }

    pub fn size_distributed(
        &self,
        room_to_distribute: Unit,
        sum_grow: Option<Fill>,
        sum_shrink: Option<Fill>,
    ) -> Unit {
        let size = self.base_size();
        match room_to_distribute.cmp(&Unit::zero()) {
            Ordering::Less => match (self.shrink, sum_shrink) {
                (Some(shrink), Some(sum_shrink)) => {
                    self.size_filled(size + room_to_distribute * (shrink, sum_shrink))
                }
                (_, _) => size,
            },
            Ordering::Equal => size,
            Ordering::Greater => match (self.grow, sum_grow) {
                (Some(grow), Some(sum_grow)) => {
                    self.size_filled(size + room_to_distribute * (grow, sum_grow))
                }
                (_, _) => size,
            },
        }
    }

    pub fn min_of(&self, rhs: &FlexDim) -> Self {
        let mut min = self.clone();
        min.base = min.base.min_of(&rhs.base);
        min
    }

    pub fn max_of(&self, rhs: &FlexDim) -> Self {
        let mut max = self.clone();
        max.base = max.base.max_of(&rhs.base);
        max
    }
}

impl From<Unit> for FlexDim {
    fn from(size: Unit) -> Self {
        Self {
            base: Dim::Fixed(size),
            min: MaybeDim::None,
            max: MaybeDim::None,
            shrink: None,
            grow: None,
        }
    }
}

impl From<Fill> for FlexDim {
    fn from(fill: Fill) -> Self {
        Self {
            base: Dim::Fixed(Unit::zero()),
            min: MaybeDim::None,
            max: MaybeDim::None,
            shrink: None,
            grow: Some(fill),
        }
    }
}

impl Add<&Self> for FlexDim {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl AddAssign<&Self> for FlexDim {
    fn add_assign(&mut self, rhs: &Self) {
        self.base = &self.base + &rhs.base;
        self.min = &self.min + &rhs.min;
        self.max = &self.max + &rhs.max;
        self.grow = add_fill(self.grow, rhs.grow);
        self.shrink = add_fill(self.shrink, rhs.shrink);
    }
}

impl Add<Unit> for FlexDim {
    type Output = Self;

    fn add(mut self, rhs: Unit) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}

impl AddAssign<Unit> for FlexDim {
    fn add_assign(&mut self, rhs: Unit) {
        self.base.set_size(self.base_size() + rhs);
    }
}

impl Sub<&Self> for FlexDim {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl SubAssign<&Self> for FlexDim {
    fn sub_assign(&mut self, rhs: &Self) {
        self.base = &self.base - &rhs.base;
        self.min = &self.min - &rhs.min;
        self.max = &self.max - &rhs.max;
        self.grow = sub_fill(self.grow, rhs.grow);
        self.shrink = sub_fill(self.shrink, rhs.shrink);

        if let (Some(base), Some(min)) = (self.base.size(), self.min.size()) {
            if min > base {
                self.min.set_size(base);
            }
        }

        if let (Some(base), Some(max)) = (self.base.size(), self.max.size()) {
            if max < base {
                self.max.set_size(base);
            }
        }
    }
}

impl Sub<Unit> for FlexDim {
    type Output = Self;

    fn sub(mut self, rhs: Unit) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl SubAssign<Unit> for FlexDim {
    fn sub_assign(&mut self, rhs: Unit) {
        self.base.set_size(self.base_size() - rhs);
    }
}

impl Mul<f64> for FlexDim {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.mul_assign(rhs);
        self
    }
}

impl MulAssign<f64> for FlexDim {
    fn mul_assign(&mut self, rhs: f64) {
        let rhs = rhs.max(0.0);
        self.base.set_size(self.base_size() * rhs);
        if let Some(min) = self.min.as_mut() {
            *min *= rhs;
        }
        if let Some(max) = self.max.as_mut() {
            *max *= rhs;
        }
        if let Some(grow) = self.grow.as_mut() {
            *grow *= rhs;
        }
        if let Some(shrink) = self.shrink.as_mut() {
            *shrink *= rhs;
        }
    }
}

impl Div<f64> for FlexDim {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        self.div_assign(rhs);
        self
    }
}

impl DivAssign<f64> for FlexDim {
    fn div_assign(&mut self, rhs: f64) {
        let rhs = rhs.max(1.0e-6);
        self.base.set_size(self.base_size() / rhs);
        if let Some(min) = self.min.as_mut() {
            *min /= rhs;
        }
        if let Some(max) = self.max.as_mut() {
            *max /= rhs;
        }
        if let Some(grow) = self.grow.as_mut() {
            *grow /= rhs;
        }
        if let Some(shrink) = self.shrink.as_mut() {
            *shrink /= rhs;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dimension::Dim,
        layout::dimension::FlexDim,
        unit::{Fill, Unit},
    };

    #[test]
    fn it_constructs() {
        let dim: FlexDim = Unit::from(15).into();

        assert!(matches!(dim.base, Dim::Fixed(Unit(15))));
        assert_eq!(Some(Unit::from(15)), dim.base.size());
        assert_eq!(None, dim.min.size());
        assert_eq!(None, dim.max.size());
        assert!(dim.grow.is_none());
        assert!(dim.shrink.is_none());

        let dim: FlexDim = Fill::from(5).into();

        assert_eq!(Some(Unit::zero()), dim.base.size());
        assert_eq!(None, dim.min.size());
        assert_eq!(None, dim.max.size());
        assert_eq!(Fill::from(5), dim.grow.unwrap());
        assert!(dim.shrink.is_none());

        let dim = FlexDim::content();

        assert!(matches!(dim.base, Dim::Content(None)));
        assert!(dim.base.size().is_none());
        assert_eq!(None, dim.min.size());
        assert_eq!(None, dim.max.size());
        assert!(dim.grow.is_none());
        assert!(dim.shrink.is_none());

        let dim = FlexDim::parented(3);

        assert!(matches!(dim.base, Dim::Parent(Fill(3), None)));
        assert!(dim.base.size().is_none());
        assert_eq!(None, dim.min.size());
        assert_eq!(None, dim.max.size());
        assert!(dim.grow.is_none());
        assert!(dim.shrink.is_none());
    }

    #[test]
    fn it_builds() {
        let mut dim = FlexDim::fixed(100).with_min(25);

        assert_eq!(Some(Unit::from(100)), dim.base.size());
        assert_eq!(Some(Unit::from(25)), dim.min.size());
        assert_eq!(None, dim.max.size());
        assert!(dim.grow.is_none());
        assert!(dim.shrink.is_none());

        dim = dim.with_max(400);

        assert_eq!(Some(Unit::from(100)), dim.base.size());
        assert_eq!(Some(Unit::from(25)), dim.min.size());
        assert_eq!(Some(Unit::from(400)), dim.max.size());
        assert!(dim.grow.is_none());
        assert!(dim.shrink.is_none());

        dim = dim.with_grow(2);

        assert_eq!(Some(Unit::from(100)), dim.base.size());
        assert_eq!(Some(Unit::from(25)), dim.min.size());
        assert_eq!(Some(Unit::from(400)), dim.max.size());
        assert_eq!(Some(Fill::from(2)), dim.grow);
        assert!(dim.shrink.is_none());

        dim = dim.with_shrink(3);

        assert_eq!(Some(Unit::from(100)), dim.base.size());
        assert_eq!(Some(Unit::from(25)), dim.min.size());
        assert_eq!(Some(Unit::from(400)), dim.max.size());
        assert_eq!(Some(Fill::from(2)), dim.grow);
        assert_eq!(Some(Fill::from(3)), dim.shrink);
    }

    #[test]
    fn size_is_zero() {
        let dim = FlexDim::content();
        assert_eq!(None, dim.base.size());
        assert!(dim.is_content());
        assert_eq!(Unit::zero(), dim.base_size());
    }

    #[test]
    fn it_clamps() {
        let dim = FlexDim::fixed(10).with_min(15);
        assert_eq!(Unit::from(15), dim.base_size());
        assert_eq!(Unit::from(15), dim.min.size().unwrap());

        let dim = FlexDim::fixed(15).with_max(10);
        assert_eq!(Unit::from(10), dim.base_size());
        assert_eq!(Unit::from(10), dim.max.size().unwrap());

        let dim = FlexDim::fixed(10).with_min(15).with_max(5);
        assert_eq!(Unit::from(5), dim.base_size());
        assert_eq!(Unit::from(5), dim.min.size().unwrap());
        assert_eq!(Unit::from(5), dim.max.size().unwrap());

        let dim = FlexDim::fixed(10).with_max(5).with_min(15);
        assert_eq!(Unit::from(15), dim.base_size());
        assert_eq!(Unit::from(15), dim.min.size().unwrap());
        assert_eq!(Unit::from(15), dim.max.size().unwrap());
    }

    #[test]
    fn min_max() {
        let dim = FlexDim::fixed(15).min_of(&FlexDim::fixed(10));
        assert_eq!(Unit::from(10), dim.base_size());

        let dim = FlexDim::fixed(5).max_of(&FlexDim::fixed(10));
        assert_eq!(Unit::from(10), dim.base_size());
    }

    #[test]
    fn binary_ops() {
        let dim1 = FlexDim::fixed(10)
            .with_min(5)
            .with_max(15)
            .with_shrink(1)
            .with_grow(2);
        let dim2 = FlexDim::fixed(11)
            .with_min(4)
            .with_max(16)
            .with_shrink(1)
            .with_grow(1);

        let dim = dim1.clone() + &dim2;
        assert_eq!(Unit::from(21), dim.base_size());
        assert_eq!(9, dim.min.size().unwrap().0);
        assert_eq!(31, dim.max.size().unwrap().0);
        assert_eq!(2, dim.shrink.unwrap().0);
        assert_eq!(3, dim.grow.unwrap().0);

        let dim = dim1.clone() - &dim2;
        assert_eq!(Unit::from(-1), dim.base_size());
        assert_eq!(-1, dim.min.size().unwrap().0,);
        assert_eq!(-1, dim.max.size().unwrap().0,);
        assert_eq!(0, dim.shrink.unwrap().0,);
        assert_eq!(1, dim.grow.unwrap().0,);

        let dim = dim2 - &dim1;
        assert_eq!(Unit::from(1), dim.base_size());
        assert_eq!(-1, dim.min.size().unwrap().0,);
        assert_eq!(1, dim.max.size().unwrap().0,);
        assert_eq!(0, dim.shrink.unwrap().0,);
        assert_eq!(0, dim.grow.unwrap().0,);

        let unit = Unit(2);
        let dim = dim1.clone() + unit;
        assert_eq!(Unit::from(12), dim.base_size());
        assert_eq!(5, dim.min.size().unwrap().0,);
        assert_eq!(15, dim.max.size().unwrap().0,);
        assert_eq!(1, dim.shrink.unwrap().0,);
        assert_eq!(2, dim.grow.unwrap().0,);

        let dim = dim1.clone() - unit;
        assert_eq!(Unit::from(8), dim.base_size());
        assert_eq!(5, dim.min.size().unwrap().0,);
        assert_eq!(15, dim.max.size().unwrap().0,);
        assert_eq!(1, dim.shrink.unwrap().0,);
        assert_eq!(2, dim.grow.unwrap().0,);

        let dim = dim1.clone() * 2.0;
        assert_eq!(Unit::from(20), dim.base_size());
        assert_eq!(10, dim.min.size().unwrap().0,);
        assert_eq!(30, dim.max.size().unwrap().0,);
        assert_eq!(2, dim.shrink.unwrap().0,);
        assert_eq!(4, dim.grow.unwrap().0,);

        let dim = dim1.clone() / 2.0;
        assert_eq!(Unit::from(5), dim.base_size());
        assert_eq!(2, dim.min.size().unwrap().0,);
        assert_eq!(7, dim.max.size().unwrap().0,);
        assert_eq!(0, dim.shrink.unwrap().0,);
        assert_eq!(1, dim.grow.unwrap().0,);

        let dim = dim1 * -2.0;
        assert_eq!(Unit::from(0), dim.base_size());
        assert_eq!(0, dim.min.size().unwrap().0,);
        assert_eq!(0, dim.max.size().unwrap().0);
        assert_eq!(0, dim.shrink.unwrap().0);
        assert_eq!(0, dim.grow.unwrap().0);
    }

    #[test]
    fn it_fills_size() {
        let dim = FlexDim::fixed(10);
        let size = dim.size_filled(Unit(20));
        assert_eq!(Unit::from(10), size);

        let dim = FlexDim::fixed(10).with_grow(1);
        let size = dim.size_filled(Unit(20));
        assert_eq!(Unit::from(20), size);

        let dim = FlexDim::fixed(10).with_grow(1).with_max(15);
        let size = dim.size_filled(Unit(20));
        assert_eq!(Unit::from(15), size);

        let dim = FlexDim::fixed(10);
        let size = dim.size_filled(Unit(5));
        assert_eq!(Unit::from(10), size);

        let dim = FlexDim::fixed(10).with_shrink(1);
        let size = dim.size_filled(Unit(5));
        assert_eq!(Unit::from(5), size);

        let dim = FlexDim::fixed(100).with_shrink(1).with_min(75);
        let size = dim.size_filled(Unit(50));
        assert_eq!(Unit::from(75), size);
    }

    #[test]
    fn it_distributes_size() {
        let dim = FlexDim::fixed(10);
        let size = dim.size_distributed(10.into(), None, None);
        assert_eq!(Unit::from(10), size);

        let dim = FlexDim::fixed(10);
        let size = dim.size_distributed(10.into(), Some(10.into()), Some(10.into()));
        assert_eq!(Unit::from(10), size);

        let dim = FlexDim::fixed(10).with_grow(5);
        let size = dim.size_distributed(10.into(), None, None);
        assert_eq!(Unit::from(10), size);

        let dim = FlexDim::fixed(10).with_grow(5);
        let size = dim.size_distributed(10.into(), Some(5.into()), None);
        assert_eq!(Unit::from(20), size);

        let dim = FlexDim::fixed(10).with_grow(5);
        let size = dim.size_distributed(10.into(), Some(10.into()), None);
        assert_eq!(Unit::from(15), size);

        let dim = FlexDim::fixed(10).with_grow(5).with_shrink(5);
        let size = dim.size_distributed(10.into(), Some(10.into()), Some(10.into()));
        assert_eq!(Unit::from(15), size);

        let dim = FlexDim::fixed(10).with_shrink(5);
        let size = dim.size_distributed((-10).into(), None, None);
        assert_eq!(Unit::from(10), size);

        let dim = FlexDim::fixed(10).with_shrink(5);
        let size = dim.size_distributed((-10).into(), None, Some(5.into()));
        assert_eq!(Unit::from(0), size);

        let dim = FlexDim::fixed(10).with_shrink(5);
        let size = dim.size_distributed((-10).into(), None, Some(10.into()));
        assert_eq!(Unit::from(5), size);

        let dim = FlexDim::fixed(10).with_grow(5).with_shrink(5);
        let size = dim.size_distributed((-10).into(), Some(10.into()), Some(10.into()));
        assert_eq!(Unit::from(5), size);
    }
}
