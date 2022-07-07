use std::ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Unit(pub(crate) i64);

impl Unit {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fill(pub(crate) usize);
pub type FillPerMille = Fill;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mm(pub f64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pt(pub f64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Em(pub f64);

impl Em {
    pub fn is_zero(&self) -> bool {
        self.0 == 0.0
    }
}

impl From<i64> for Unit {
    fn from(unit: i64) -> Self {
        Self(unit)
    }
}

impl From<usize> for Fill {
    fn from(fill: usize) -> Self {
        Self::new(fill)
    }
}

impl Add for Unit {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}

impl AddAssign for Unit {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Unit {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl SubAssign for Unit {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul<usize> for Unit {
    type Output = Self;

    fn mul(mut self, rhs: usize) -> Self::Output {
        self.mul_assign(rhs);
        self
    }
}

impl MulAssign<usize> for Unit {
    fn mul_assign(&mut self, rhs: usize) {
        self.0 *= rhs as i64;
    }
}

impl Mul<Unit> for usize {
    type Output = Unit;

    fn mul(self, mut rhs: Unit) -> Self::Output {
        rhs.mul_assign(self);
        rhs
    }
}

impl Mul<f64> for Unit {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.mul_assign(rhs);
        self
    }
}

impl MulAssign<f64> for Unit {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 = (self.0 as f64 * rhs) as i64;
    }
}

impl Mul<Unit> for f64 {
    type Output = Unit;

    fn mul(self, mut rhs: Unit) -> Self::Output {
        rhs.mul_assign(self);
        rhs
    }
}

impl Div<f64> for Unit {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        self.div_assign(rhs);
        self
    }
}

impl DivAssign<f64> for Unit {
    fn div_assign(&mut self, rhs: f64) {
        self.0 = (self.0 as f64 / rhs.max(1.0e-6)) as i64;
    }
}

impl Mul<(Fill, Fill)> for Unit {
    type Output = Self;

    fn mul(mut self, rhs: (Fill, Fill)) -> Self::Output {
        self.mul_assign(rhs);
        self
    }
}

impl MulAssign<(Fill, Fill)> for Unit {
    fn mul_assign(&mut self, rhs: (Fill, Fill)) {
        let divisor = rhs.1 .0 as i64;
        let rounding = self.0.signum() * divisor / 2;
        self.0 = (self.0 * rhs.0 .0 as i64 + rounding) / divisor;
    }
}

impl Fill {
    pub fn new(fill: usize) -> Self {
        Self(fill)
    }

    pub fn none() -> Self {
        Self(0)
    }

    pub fn equal() -> Self {
        Self(1)
    }

    pub fn full() -> Self {
        Self(1000)
    }

    pub fn mille() -> Self {
        Self(1000)
    }
}

impl Add for Fill {
    type Output = Self;

    fn add(mut self, rhs: Fill) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}

impl AddAssign for Fill {
    fn add_assign(&mut self, rhs: Fill) {
        self.0 += rhs.0;
    }
}

impl Sub for Fill {
    type Output = Self;

    fn sub(mut self, rhs: Fill) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl SubAssign for Fill {
    fn sub_assign(&mut self, rhs: Fill) {
        self.0 -= rhs.0.min(self.0);
    }
}

impl Mul<f64> for Fill {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.mul_assign(rhs);
        self
    }
}

impl MulAssign<f64> for Fill {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 = (self.0 as f64 * rhs.max(0.0)) as usize;
    }
}

impl Div<f64> for Fill {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        self.div_assign(rhs);
        self
    }
}

impl DivAssign<f64> for Fill {
    fn div_assign(&mut self, rhs: f64) {
        self.0 = (self.0 as f64 / rhs.max(1.0e-3)) as usize;
    }
}

impl Add<Em> for Em {
    type Output = Em;

    fn add(self, rhs: Em) -> Self::Output {
        Em(self.0 + rhs.0)
    }
}

impl AddAssign for Em {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub<Em> for Em {
    type Output = Em;

    fn sub(self, rhs: Em) -> Self::Output {
        Em(self.0 - rhs.0)
    }
}

impl SubAssign for Em {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul<f64> for Em {
    type Output = Em;

    fn mul(self, rhs: f64) -> Self::Output {
        Em(self.0 * rhs)
    }
}

impl MulAssign<f64> for Em {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl Mul<Pt> for Em {
    type Output = Pt;

    fn mul(self, rhs: Pt) -> Self::Output {
        Pt(self.0 * rhs.0)
    }
}

pub fn add_unit(l: Option<Unit>, r: Option<Unit>) -> Option<Unit> {
    match (l, r) {
        (None, None) => None,
        (None, Some(_)) => r,
        (Some(_), None) => l,
        (Some(l), Some(r)) => Some(l + r),
    }
}

pub fn sub_unit(l: Option<Unit>, r: Option<Unit>) -> Option<Unit> {
    match (l, r) {
        (None, None) => None,
        (None, Some(r)) => Some(Unit(0) - r),
        (Some(_), None) => l,
        (Some(l), Some(r)) => Some(l - r),
    }
}

pub fn add_fill(l: Option<Fill>, r: Option<Fill>) -> Option<Fill> {
    match (l, r) {
        (None, None) => None,
        (None, Some(_)) => r,
        (Some(_), None) => l,
        (Some(l), Some(r)) => Some(l + r),
    }
}

pub fn sub_fill(l: Option<Fill>, r: Option<Fill>) -> Option<Fill> {
    match (l, r) {
        (None, None) => None,
        (None, Some(r)) => Some(Fill(0) - r),
        (Some(_), None) => l,
        (Some(l), Some(r)) => Some(l - r),
    }
}

impl From<Mm> for Unit {
    fn from(mm: Mm) -> Self {
        Self((mm.0 * 1000.0).round() as i64)
    }
}

impl From<Unit> for Mm {
    fn from(unit: Unit) -> Self {
        Self(unit.0 as f64 / 1000.0)
    }
}

impl From<Pt> for Unit {
    fn from(pt: Pt) -> Self {
        Mm::from(pt).into()
    }
}

impl From<Unit> for Pt {
    fn from(unit: Unit) -> Self {
        Mm::from(unit).into()
    }
}

impl From<f64> for Mm {
    fn from(mm: f64) -> Self {
        Self(mm)
    }
}

impl From<Pt> for Mm {
    fn from(pt: Pt) -> Self {
        Self(pt.0 * 25.4 / 72.0)
    }
}

impl From<f64> for Pt {
    fn from(pt: f64) -> Self {
        Self(pt)
    }
}

impl From<Mm> for Pt {
    fn from(mm: Mm) -> Self {
        Self(mm.0 * 72.0 / 25.4)
    }
}

impl Add<Mm> for Unit {
    type Output = Unit;

    fn add(self, rhs: Mm) -> Self::Output {
        Unit(self.0 + Unit::from(rhs).0)
    }
}

impl Add<Pt> for Unit {
    type Output = Unit;

    fn add(self, rhs: Pt) -> Self::Output {
        Unit(self.0 + Unit::from(rhs).0)
    }
}

impl Add for Mm {
    type Output = Mm;

    fn add(self, rhs: Mm) -> Self::Output {
        Mm(self.0 + rhs.0)
    }
}

impl Sub for Mm {
    type Output = Mm;

    fn sub(self, rhs: Mm) -> Self::Output {
        Mm(self.0 - rhs.0)
    }
}

impl Mul<f64> for Mm {
    type Output = Mm;

    fn mul(self, rhs: f64) -> Self::Output {
        Mm(self.0 * rhs)
    }
}

impl Mul<Mm> for f64 {
    type Output = Mm;

    fn mul(self, rhs: Mm) -> Self::Output {
        Mm(rhs.0 * self)
    }
}

impl Add<Pt> for Mm {
    type Output = Mm;

    fn add(self, rhs: Pt) -> Self::Output {
        Mm(self.0) + Mm::from(rhs)
    }
}

impl Sub<Pt> for Mm {
    type Output = Mm;

    fn sub(self, rhs: Pt) -> Self::Output {
        Mm(self.0) - Mm::from(rhs)
    }
}

impl Mul<f64> for Pt {
    type Output = Pt;

    fn mul(self, rhs: f64) -> Self::Output {
        Pt(self.0 * rhs)
    }
}

impl Mul<Pt> for f64 {
    type Output = Pt;

    fn mul(self, rhs: Pt) -> Self::Output {
        Pt(rhs.0 * self)
    }
}

impl Deref for Mm {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Pt {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Em {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::unit::{add_fill, sub_fill, Fill, Unit};

    #[test]
    fn they_constructs() {
        let unit: Unit = 297.into();
        assert_eq!(297, unit.0);

        let fill: Fill = 15.into();
        assert_eq!(15, fill.0);
    }

    #[test]
    fn unit_bin_ops() {
        let unit1: Unit = 12.into();
        let unit2: Unit = (-6).into();

        assert_eq!(Unit::from(6), unit1 + unit2);
        assert_eq!(Unit::from(18), unit1 - unit2);

        assert_eq!(Unit::from(24), unit1 * 2.0);
        assert_eq!(Unit::from(6), unit1 / 2.0);
    }

    #[test]
    fn unit_assign_ops() {
        let unit1: Unit = 12.into();
        let unit2: Unit = (-6).into();

        let mut unit3 = unit1;
        unit3 += unit2;
        assert_eq!(Unit::from(6), unit3);

        let mut unit3 = unit1;
        unit3 -= unit2;
        assert_eq!(Unit::from(18), unit3);

        let mut unit3 = unit1;
        unit3 *= 2.0;
        assert_eq!(Unit::from(24), unit3);

        let mut unit3 = unit1;
        unit3 /= 2.0;
        assert_eq!(Unit::from(6), unit3);
    }

    #[test]
    fn fill_bin_ops() {
        let fill1: Fill = 12.into();
        let fill2: Fill = 6.into();

        assert_eq!(18, (fill1 + fill2).0);
        assert_eq!(6, (fill1 - fill2).0);

        assert_eq!(24, (fill1 * 2.0).0);
        assert_eq!(6, (fill1 / 2.0).0);

        assert_eq!(0, (fill1 * -2.0).0);
        assert_eq!(12_000, (fill1 / -2.0).0);
    }

    #[test]
    fn fill_assign_ops() {
        let fill1: Fill = 12.into();
        let fill2: Fill = 6.into();

        let mut fill3 = fill1;
        fill3 += fill2;
        assert_eq!(18, fill3.0);

        let mut fill3 = fill1;
        fill3 -= fill2;
        assert_eq!(6, fill3.0);

        let mut fill3 = fill1;
        fill3 *= 2.0;
        assert_eq!(24, fill3.0);

        let mut fill3 = fill1;
        fill3 /= 2.0;
        assert_eq!(6, fill3.0);
    }

    #[test]
    fn unit_fill_bin_ops() {
        let unit: Unit = 12.into();
        let fill: Fill = 2.into();

        assert_eq!(Unit::from(24), unit * (fill, Fill::equal()));
        assert_eq!(Unit::from(6), unit * (Fill::equal(), fill));
    }

    #[test]
    fn unit_fill_assign_ops() {
        let unit: Unit = 12.into();
        let fill: Fill = 2.into();

        let mut unit1 = unit;
        unit1 *= (fill, Fill::equal());
        assert_eq!(Unit::from(24), unit1);

        let mut unit1 = unit;
        unit1 *= (Fill::equal(), fill);
        assert_eq!(Unit::from(6), unit1);
    }

    #[test]
    fn fill_ops_with_option() {
        let fill1 = Some(Fill::from(5));
        let fill2 = Some(Fill::from(15));

        assert!(add_fill(None, None).is_none());
        assert_eq!(add_fill(fill1, None).unwrap().0, 5);
        assert_eq!(add_fill(None, fill2).unwrap().0, 15);
        assert_eq!(add_fill(fill1, fill2).unwrap().0, 20);

        assert!(sub_fill(None, None).is_none());
        assert_eq!(sub_fill(fill1, None).unwrap().0, 5);
        assert_eq!(sub_fill(None, fill2).unwrap().0, 0);
        assert_eq!(sub_fill(fill1, fill2).unwrap().0, 0);
    }
}
