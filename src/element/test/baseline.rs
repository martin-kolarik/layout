use crate::{
    position::{Offset, Size},
    test::Ctx,
    AlignItems, DefaultFactory, Factory, Layout, Position, Style,
};

#[test]
fn self_not_baseline_on_baseline() {
    let mut box1 = DefaultFactory::vbox().size(100);

    box1.lay_out(
        &mut Ctx,
        Offset::new(10, 10),
        Size::fixed_depth(190, 277, 247),
    )
    .unwrap();

    assert_eq!(10, box1.offset_ref().x().0);
    assert_eq!(10, box1.offset_ref().y().0);
    assert_eq!(0, box1.size_ref().width().0);
    assert_eq!(100, box1.size_ref().height().0);
    assert!(box1.size_ref().depth().is_none());
    assert_eq!(0, box1.content_size().unwrap().width().0);
    assert_eq!(0, box1.content_size().unwrap().height().0);
}

#[test]
fn self_baseline_on_not_baseline() {
    let mut box1 = DefaultFactory::vbox().size(100).depth(85);

    box1.lay_out(&mut Ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    assert_eq!(10, box1.offset_ref().x().0);
    assert_eq!(10, box1.offset_ref().y().0);
    assert_eq!(0, box1.size_ref().width().0);
    assert_eq!(100, box1.size_ref().height().0);
    assert_eq!(85, box1.size_ref().depth().unwrap().0);
    assert_eq!(0, box1.content_size().unwrap().width().0);
    assert_eq!(0, box1.content_size().unwrap().height().0);
}

#[test]
fn self_baseline_on_baseline() {
    let mut box1 = DefaultFactory::vbox().size(100).depth(85);

    box1.lay_out(
        &mut Ctx,
        Offset::new(10, 10),
        Size::fixed_depth(190, 277, 247),
    )
    .unwrap();

    assert_eq!(10, box1.offset_ref().x().0);
    assert_eq!(25, box1.offset_ref().y().0);
    assert_eq!(0, box1.size_ref().width().0);
    assert_eq!(100, box1.size_ref().height().0);
    assert_eq!(85, box1.size_ref().depth().unwrap().0);
    assert_eq!(0, box1.content_size().unwrap().width().0);
    assert_eq!(0, box1.content_size().unwrap().height().0);
}

#[test]
fn self_baseline_inherits() {
    let box1 = DefaultFactory::vbox().size(100).depth(85);

    let mut outer = DefaultFactory::vbox()
        .child(box1)
        .style(Style::new().with_align_items(AlignItems::Baseline));

    outer
        .lay_out(&mut Ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    assert_eq!(10, outer.offset_ref().x().0);
    assert_eq!(10, outer.offset_ref().y().0);
    assert_eq!(0, outer.size_ref().width().0);
    assert_eq!(100, outer.size_ref().height().0);
    assert_eq!(85, outer.size_ref().depth().unwrap().0);
    assert_eq!(0, outer.content_size().unwrap().width().0);
    assert_eq!(100, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x().0);
    assert_eq!(10, box1.offset_ref().y().0);
    assert_eq!(0, box1.size_ref().width().0);
    assert_eq!(100, box1.size_ref().height().0);
    assert_eq!(85, box1.size_ref().depth().unwrap().0);
}

#[test]
fn self_baseline_inherits_and_positions() {
    let box1 = DefaultFactory::vbox().size(100).depth(85);

    let mut outer = DefaultFactory::vbox()
        .child(box1)
        .style(Style::new().with_align_items(AlignItems::Baseline));

    outer
        .lay_out(
            &mut Ctx,
            Offset::new(10, 10),
            Size::fixed_depth(190, 277, 267),
        )
        .unwrap();

    assert_eq!(10, outer.offset_ref().x().0);
    assert_eq!(10, outer.offset_ref().y().0);
    assert_eq!(0, outer.size_ref().width().0);
    assert_eq!(95, outer.size_ref().height().0);
    assert_eq!(80, outer.size_ref().depth().unwrap().0);
    assert_eq!(0, outer.content_size().unwrap().width().0);
    assert_eq!(95, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x().0);
    assert_eq!(5, box1.offset_ref().y().0);
    assert_eq!(0, box1.size_ref().width().0);
    assert_eq!(100, box1.size_ref().height().0);
    assert_eq!(85, box1.size_ref().depth().unwrap().0);
}
