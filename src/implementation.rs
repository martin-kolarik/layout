use crate::{
    Axis, BlockBox, Filling, Layout, LayoutBox, PageBreak, Text, Wrap, dimension::Dim,
    position::Offset, unit::Fill,
};

pub fn bbox(offset: Offset) -> BlockBox {
    BlockBox::new(offset)
}

pub fn hbox() -> LayoutBox {
    LayoutBox::new(crate::Axis::Horizontal)
}

pub fn hfilling() -> Filling {
    Filling::new(Axis::Horizontal)
}

pub fn hcbox(layout: impl Layout + 'static) -> LayoutBox {
    hbox().child(hfill(1)).child(layout).child(hfill(1))
}

pub fn hfill(weight: impl Into<Fill>) -> Filling {
    let weight = weight.into();
    hfilling().grow(weight).shrink(weight)
}

pub fn hspace(size: impl Into<Dim>) -> Filling {
    hfilling().size(size)
}

pub fn hwrap() -> Wrap {
    Wrap::new(Axis::Horizontal)
}

pub fn vbox() -> LayoutBox {
    LayoutBox::new(Axis::Vertical)
}

pub fn vfilling() -> Filling {
    Filling::new(Axis::Vertical)
}

pub fn vcbox(layout: impl Layout + 'static) -> LayoutBox {
    vbox().child(vfill(1)).child(layout).child(vfill(1))
}

pub fn vfill(weight: impl Into<Fill>) -> Filling {
    let weight = weight.into();
    vfilling().grow(weight).shrink(weight)
}

pub fn vspace(size: impl Into<Dim>) -> Filling {
    vfilling().size(size)
}

pub fn vwrap() -> Wrap {
    Wrap::new(Axis::Vertical)
}

pub fn vbreak() -> PageBreak {
    PageBreak::new()
}

pub fn text(text: impl ToString) -> Text {
    Text::new(text)
}
