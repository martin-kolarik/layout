use crate::{
    AlignItems, Layout, Position, StyleBuilder,
    position::{Offset, Size},
    test::Ctx,
    vbox,
};

#[test]
fn self_not_baseline_on_baseline() {
    let mut box1 = vbox().axis_size(100);

    box1.lay_out(
        &mut Ctx,
        Offset::new(10, 10),
        Size::fixed_depth(190, 277, 247),
    )
    .unwrap();

    assert_eq!(10, box1.offset().x.0);
    assert_eq!(10, box1.offset().y.0);
    assert_eq!(0, box1.size().base_width().0);
    assert_eq!(100, box1.size().base_height().0);
    assert!(box1.size().depth().is_none());
    assert_eq!(0, box1.content_size().unwrap().base_width().0);
    assert_eq!(0, box1.content_size().unwrap().base_height().0);
}

#[test]
fn self_baseline_on_not_baseline() {
    let mut box1 = vbox().axis_size(100).axis_depth(85);

    box1.lay_out(&mut Ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    assert_eq!(10, box1.offset().x.0);
    assert_eq!(10, box1.offset().y.0);
    assert_eq!(0, box1.size().base_width().0);
    assert_eq!(100, box1.size().base_height().0);
    assert_eq!(85, box1.size().depth().unwrap().0);
    assert_eq!(0, box1.content_size().unwrap().base_width().0);
    assert_eq!(0, box1.content_size().unwrap().base_height().0);
}

#[test]
fn self_baseline_on_baseline() {
    let mut box1 = vbox()
        .axis_size(100)
        .axis_depth(85)
        .style(StyleBuilder::new().with_align_items(AlignItems::Baseline));

    box1.lay_out(
        &mut Ctx,
        Offset::new(10, 10),
        Size::fixed_depth(190, 277, 247),
    )
    .unwrap();

    assert_eq!(10, box1.offset().x.0);
    assert_eq!(25, box1.offset().y.0);
    assert_eq!(0, box1.size().base_width().0);
    assert_eq!(100, box1.size().base_height().0);
    assert_eq!(85, box1.size().depth().unwrap().0);
    assert_eq!(0, box1.content_size().unwrap().base_width().0);
    assert_eq!(0, box1.content_size().unwrap().base_height().0);
}

#[test]
fn self_baseline_inherits() {
    let box1 = vbox().axis_size(100).axis_depth(85);

    let mut outer = vbox()
        .child(box1)
        .style(StyleBuilder::new().with_align_items(AlignItems::Baseline))
        // depth here is artificial as there measure pass is missing
        .axis_depth(85);

    outer
        .lay_out(&mut Ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    assert_eq!(10, outer.offset().x.0);
    assert_eq!(10, outer.offset().y.0);
    assert_eq!(0, outer.size().base_width().0);
    assert_eq!(100, outer.size().base_height().0);
    assert_eq!(85, outer.size().depth().unwrap().0);
    assert_eq!(0, outer.content_size().unwrap().base_width().0);
    assert_eq!(100, outer.content_size().unwrap().base_height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset().x.0);
    assert_eq!(10, box1.offset().y.0);
    assert_eq!(0, box1.size().base_width().0);
    assert_eq!(100, box1.size().base_height().0);
    assert_eq!(85, box1.size().depth().unwrap().0);
}

#[test]
fn self_baseline_inherits_and_positions() {
    let box1 = vbox().axis_size(100).axis_depth(85);

    let mut outer = vbox()
        .child(box1)
        .style(StyleBuilder::new().with_align_items(AlignItems::Baseline))
        // size and depth are set due to emulate missing measure phase
        .axis_size(100)
        .axis_depth(85);

    outer
        .lay_out(
            &mut Ctx,
            Offset::new(10, 10),
            Size::fixed_depth(190, 277, 267),
        )
        .unwrap();

    assert_eq!(10, outer.offset().x.0);
    assert_eq!(5, outer.offset().y.0);
    assert_eq!(0, outer.size().base_width().0);
    assert_eq!(100, outer.size().base_height().0);
    assert_eq!(85, outer.size().depth().unwrap().0);
    assert_eq!(0, outer.content_size().unwrap().base_width().0);
    assert_eq!(100, outer.content_size().unwrap().base_height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset().x.0);
    assert_eq!(5, box1.offset().y.0);
    assert_eq!(0, box1.size().base_width().0);
    assert_eq!(100, box1.size().base_height().0);
    assert_eq!(85, box1.size().depth().unwrap().0);
}
