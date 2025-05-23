use crate::{
    position::{Offset, Size},
    test::Ctx,
    unit::Fill,
    AlignItems, DefaultFactory, Factory, Layout, Position, StyleBuilder,
};

#[test]
fn single_fixed_box_c_0_0() {
    let box1 = DefaultFactory::hbox().size(15);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(15, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(15, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);
}

#[test]
fn single_fixed_box_c_0_1() {
    let box1 = DefaultFactory::hbox().size(15).cross_size(12);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(15, outer.size_ref().width().0);
    assert_eq!(12, outer.size_ref().height().0);
    assert_eq!(15, outer.content_size().unwrap().width().0);
    assert_eq!(12, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(12, box1.size_ref().height().0);
}

#[test]
fn single_fixed_box_c_1_0() {
    let box1 = DefaultFactory::hbox().size(15);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .cross_size(12);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(15, outer.size_ref().width().0);
    assert_eq!(12, outer.size_ref().height().0);
    assert_eq!(15, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);
}

#[test]
fn single_fixed_box_c_1_1() {
    let box1 = DefaultFactory::hbox().size(15).cross_size(21);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .cross_size(12);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(15, outer.size_ref().width().0);
    assert_eq!(12, outer.size_ref().height().0);
    assert_eq!(15, outer.content_size().unwrap().width().0);
    assert_eq!(21, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(21, box1.size_ref().height().0);
}

#[test]
fn single_grow_main_0_1_box() {
    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);

    let box1 = DefaultFactory::hbox().size(15).grow(1);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1);

    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(190, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(190, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(190, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);
}

#[test]
fn single_grow_main_1_0_box() {
    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);

    let box1 = DefaultFactory::hbox().size(15);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .grow(1);

    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(190, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(15, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);
}

#[test]
fn single_grow_main_1_1_box() {
    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);

    let box1 = DefaultFactory::hbox().size(15).grow(1);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .grow(1);

    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(190, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(190, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(190, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);
}

#[test]
fn single_grow_main_1parent_1_box() {
    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);

    let box1 = DefaultFactory::hbox().size(15).grow(1);

    let mut outer = DefaultFactory::hbox()
        .size(Fill::full())
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .grow(1);

    outer.measure(&mut Ctx, size.clone()).unwrap();
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(190, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(190, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(190, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);
}

#[test]
fn single_grow_main_0_1parent_box() {
    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);

    let box1 = DefaultFactory::hbox().size(Fill::full());

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1);

    outer.measure(&mut Ctx, size.clone()).unwrap();
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(190, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(190, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(190, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);
}

#[test]
fn single_grow_main_1parent_1parent_box() {
    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);

    let box1 = DefaultFactory::hbox().size(Fill::full());

    let mut outer = DefaultFactory::hbox()
        .size(Fill::full())
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .grow(1);

    outer.measure(&mut Ctx, size.clone()).unwrap();
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(190, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(190, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(190, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);
}

#[test]
fn single_grow_cross_0_1_box() {
    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);

    let box1 = DefaultFactory::hbox().size(15).cross_size(7).cross_grow(1);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1);

    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(15, outer.size_ref().width().0);
    assert_eq!(7, outer.size_ref().height().0);
    assert_eq!(15, outer.content_size().unwrap().width().0);
    assert_eq!(7, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(7, box1.size_ref().height().0);
}

#[test]
fn single_grow_cross_1_0fix_box() {
    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);

    let box1 = DefaultFactory::hbox().size(15).cross_size(7);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .cross_grow(1);

    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(15, outer.size_ref().width().0);
    assert_eq!(277, outer.size_ref().height().0);
    assert_eq!(15, outer.content_size().unwrap().width().0);
    assert_eq!(7, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(7, box1.size_ref().height().0);
}

#[test]
fn single_grow_cross_1_0dyn_box() {
    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);

    let box1 = DefaultFactory::hbox().size(15);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .cross_grow(1);

    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(15, outer.size_ref().width().0);
    assert_eq!(277, outer.size_ref().height().0);
    assert_eq!(15, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);
}

#[test]
fn single_grow_cross_1_1_box() {
    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);

    let box1 = DefaultFactory::hbox().size(15).cross_size(7).cross_grow(1);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .cross_grow(1);

    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(15, outer.size_ref().width().0);
    assert_eq!(277, outer.size_ref().height().0);
    assert_eq!(15, outer.content_size().unwrap().width().0);
    assert_eq!(277, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(277, box1.size_ref().height().0);
}

// #[test]
// single_cross_align_begin_box same as single_grow_cross_1_0dyn_box

#[test]
fn single_cross_align_end_box() {
    let position = Offset::new(10, 10);
    let size = Size::fixed_depth(190, 277, 3);

    let box1 = DefaultFactory::hbox().size(15).cross_size(7);

    let mut outer = DefaultFactory::hbox()
        .child(box1)
        .cross_grow(1)
        .style(StyleBuilder::new().with_align_items(AlignItems::End));

    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(15, outer.size_ref().width().0);
    assert_eq!(277, outer.size_ref().height().0);
    assert_eq!(15, outer.content_size().unwrap().width().0);
    assert_eq!(277, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(280, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(7, box1.size_ref().height().0);
}

#[test]
fn single_cross_align_center_box() {
    let position = Offset::new(10, 10);
    let size = Size::fixed_depth(190, 277, 3);

    let box1 = DefaultFactory::hbox().size(15).cross_size(7);

    let mut outer = DefaultFactory::hbox()
        .child(box1)
        .cross_grow(1)
        .style(StyleBuilder::new().with_align_items(AlignItems::Center));

    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(15, outer.size_ref().width().0);
    assert_eq!(277, outer.size_ref().height().0);
    assert_eq!(15, outer.content_size().unwrap().width().0);
    assert_eq!(142, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(145, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(7, box1.size_ref().height().0);
}

#[test]
fn single_cross_align_center_box_underflow() {
    let position = Offset::new(10, 10);
    let size = Size::fixed_depth(190, 277, 3);

    let box1 = DefaultFactory::hbox().size(15).cross_size(477);

    let mut outer = DefaultFactory::hbox()
        .child(box1)
        .cross_grow(1)
        .style(StyleBuilder::new().with_align_items(AlignItems::Center));

    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(15, outer.size_ref().width().0);
    assert_eq!(277, outer.size_ref().height().0);
    assert_eq!(15, outer.content_size().unwrap().width().0);
    assert_eq!(377, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(-90, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(477, box1.size_ref().height().0);
}

#[test]
fn single_cross_align_baseline_box() {
    let position = Offset::new(10, 10);
    let size = Size::fixed_depth(190, 277, 267);

    let box1 = DefaultFactory::hbox().size(15).cross_size(7).depth(1);

    let mut outer = DefaultFactory::hbox()
        .child(box1)
        .cross_grow(1)
        .style(StyleBuilder::new().with_align_items(AlignItems::Baseline))
        // cross_size and depth below is forced size assignment due to ommited measure call
        .cross_size(7)
        .depth(1);

    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(14, outer.offset_ref().y.0);
    assert_eq!(15, outer.size_ref().width().0);
    assert_eq!(277, outer.size_ref().height().0);
    assert_eq!(15, outer.content_size().unwrap().width().0);
    // assert_eq!(11, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let box1 = iter.next().unwrap();

    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(14, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(7, box1.size_ref().height().0);
}

#[test]
fn double_fixed_boxes_c_0_0() {
    let box1 = DefaultFactory::hbox().size(15);
    let box2 = DefaultFactory::hbox().size(18);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .child(box2);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(33, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(33, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(25, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(18, box2.size_ref().width().0);
    assert_eq!(0, box2.size_ref().height().0);
}

#[test]
fn triple_fixed_boxes_c() {
    let box1 = DefaultFactory::hbox().size(15);
    let box2 = DefaultFactory::hbox().size(18);
    let box3 = DefaultFactory::hbox().size(16);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .child(box2)
        .child(box3);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(49, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(49, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(25, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(18, box2.size_ref().width().0);
    assert_eq!(0, box2.size_ref().height().0);

    let box3 = iter.next().unwrap();
    assert_eq!(43, box3.offset_ref().x.0);
    assert_eq!(10, box3.offset_ref().y.0);
    assert_eq!(16, box3.size_ref().width().0);
    assert_eq!(0, box3.size_ref().height().0);
}

#[test]
fn triple_fixed_boxes_g() {
    let box1 = DefaultFactory::hbox().size(15);
    let box2 = DefaultFactory::hbox().size(18);
    let box3 = DefaultFactory::hbox().size(16);

    let mut outer = DefaultFactory::hbox()
        .style(
            StyleBuilder::new()
                .with_align_items(AlignItems::Start)
                .with_horizontal_gap(2),
        )
        .child(box1)
        .child(box2)
        .child(box3);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(53, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(53, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(27, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(18, box2.size_ref().width().0);
    assert_eq!(0, box2.size_ref().height().0);

    let box3 = iter.next().unwrap();
    assert_eq!(47, box3.offset_ref().x.0);
    assert_eq!(10, box3.offset_ref().y.0);
    assert_eq!(16, box3.size_ref().width().0);
    assert_eq!(0, box3.size_ref().height().0);
}

#[test]
fn triple_grow_boxes_c() {
    let box1 = DefaultFactory::hbox().size(15).grow(1);
    let box2 = DefaultFactory::hbox().size(18).grow(3);
    let box3 = DefaultFactory::hbox().size(16).grow(1);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .child(box2)
        .child(box3);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(190, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(190, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(43, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(53, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(103, box2.size_ref().width().0);
    assert_eq!(0, box2.size_ref().height().0);

    let box3 = iter.next().unwrap();
    assert_eq!(156, box3.offset_ref().x.0);
    assert_eq!(10, box3.offset_ref().y.0);
    assert_eq!(44, box3.size_ref().width().0);
    assert_eq!(0, box3.size_ref().height().0);
}

#[test]
fn triple_grow_boxes_g() {
    let box1 = DefaultFactory::hbox().size(15).grow(1);
    let box2 = DefaultFactory::hbox().size(18).grow(3);
    let box3 = DefaultFactory::hbox().size(16).grow(1);

    let mut outer = DefaultFactory::hbox()
        .style(
            StyleBuilder::new()
                .with_align_items(AlignItems::Start)
                .with_horizontal_gap(2),
        )
        .child(box1)
        .child(box2)
        .child(box3);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(189, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(189, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(42, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(54, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(100, box2.size_ref().width().0);
    assert_eq!(0, box2.size_ref().height().0);

    let box3 = iter.next().unwrap();
    assert_eq!(156, box3.offset_ref().x.0);
    assert_eq!(10, box3.offset_ref().y.0);
    assert_eq!(43, box3.size_ref().width().0);
    assert_eq!(0, box3.size_ref().height().0);
}

#[test]
fn triple_fixed_boxes_c_wrap1() {
    let box1 = DefaultFactory::hbox().size(80);
    let box2 = DefaultFactory::hbox().size(80);
    let box3 = DefaultFactory::hbox().size(80);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .child(box2)
        .child(box3);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(160, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(160, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(80, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(90, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(80, box2.size_ref().width().0);
    assert_eq!(0, box2.size_ref().height().0);

    let box3 = iter.next().unwrap();
    assert_eq!(10, box3.offset_ref().x.0);
    assert_eq!(10, box3.offset_ref().y.0);
    assert_eq!(80, box3.size_ref().width().0);
    assert_eq!(0, box3.size_ref().height().0);
}

#[test]
fn triple_fixed_boxes_c_wrap2() {
    let box1 = DefaultFactory::hbox().size(80).cross_size(10);
    let box2 = DefaultFactory::hbox().size(80).cross_size(15);
    let box3 = DefaultFactory::hbox().size(80).cross_size(20);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .child(box2)
        .child(box3);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(160, outer.size_ref().width().0);
    assert_eq!(35, outer.size_ref().height().0);
    assert_eq!(160, outer.content_size().unwrap().width().0);
    assert_eq!(35, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(80, box1.size_ref().width().0);
    assert_eq!(10, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(90, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(80, box2.size_ref().width().0);
    assert_eq!(15, box2.size_ref().height().0);

    let box3 = iter.next().unwrap();
    assert_eq!(10, box3.offset_ref().x.0);
    assert_eq!(25, box3.offset_ref().y.0);
    assert_eq!(80, box3.size_ref().width().0);
    assert_eq!(20, box3.size_ref().height().0);
}

#[test]
fn triple_fixed_boxes_g_wrap() {
    let box1 = DefaultFactory::hbox().size(80).cross_size(10);
    let box2 = DefaultFactory::hbox().size(80).cross_size(15);
    let box3 = DefaultFactory::hbox().size(80).cross_size(20);

    let mut outer = DefaultFactory::hbox()
        .style(
            StyleBuilder::new()
                .with_align_items(AlignItems::Start)
                .with_horizontal_gap(2)
                .with_vertical_gap(2),
        )
        .child(box1)
        .child(box2)
        .child(box3);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(162, outer.size_ref().width().0);
    assert_eq!(37, outer.size_ref().height().0);
    assert_eq!(162, outer.content_size().unwrap().width().0);
    assert_eq!(37, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(80, box1.size_ref().width().0);
    assert_eq!(10, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(92, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(80, box2.size_ref().width().0);
    assert_eq!(15, box2.size_ref().height().0);

    let box3 = iter.next().unwrap();
    assert_eq!(10, box3.offset_ref().x.0);
    assert_eq!(27, box3.offset_ref().y.0);
    assert_eq!(80, box3.size_ref().width().0);
    assert_eq!(20, box3.size_ref().height().0);
}

#[test]
fn triple_fixed_boxes_c_wrap_grow1() {
    let box1 = DefaultFactory::hbox().size(80).cross_size(10).grow(1);
    let box2 = DefaultFactory::hbox().size(80).cross_size(15).grow(2);
    let box3 = DefaultFactory::hbox().size(80).cross_size(20);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .child(box2)
        .child(box3);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(190, outer.size_ref().width().0);
    assert_eq!(35, outer.size_ref().height().0);
    assert_eq!(190, outer.content_size().unwrap().width().0);
    assert_eq!(35, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(90, box1.size_ref().width().0);
    assert_eq!(10, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(100, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(100, box2.size_ref().width().0);
    assert_eq!(15, box2.size_ref().height().0);

    let box3 = iter.next().unwrap();
    assert_eq!(10, box3.offset_ref().x.0);
    assert_eq!(25, box3.offset_ref().y.0);
    assert_eq!(80, box3.size_ref().width().0);
    assert_eq!(20, box3.size_ref().height().0);
}

#[test]
fn triple_fixed_boxes_c_wrap_grow2() {
    let box1 = DefaultFactory::hbox().size(80).cross_size(10).grow(1);
    let box2 = DefaultFactory::hbox().size(80).cross_size(15).grow(2);
    let box3 = DefaultFactory::hbox().size(80).cross_size(20).grow(1);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .child(box2)
        .child(box3);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(190, outer.size_ref().width().0);
    assert_eq!(35, outer.size_ref().height().0);
    assert_eq!(190, outer.content_size().unwrap().width().0);
    assert_eq!(35, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(90, box1.size_ref().width().0);
    assert_eq!(10, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(100, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(100, box2.size_ref().width().0);
    assert_eq!(15, box2.size_ref().height().0);

    let box3 = iter.next().unwrap();
    assert_eq!(10, box3.offset_ref().x.0);
    assert_eq!(25, box3.offset_ref().y.0);
    assert_eq!(190, box3.size_ref().width().0);
    assert_eq!(20, box3.size_ref().height().0);
}

#[test]
fn triple_fixed_boxes_g_wrap_grow1() {
    let box1 = DefaultFactory::hbox().size(80).cross_size(10).grow(1);
    let box2 = DefaultFactory::hbox().size(80).cross_size(15).grow(2);
    let box3 = DefaultFactory::hbox().size(80).cross_size(20);

    let mut outer = DefaultFactory::hbox()
        .style(
            StyleBuilder::new()
                .with_align_items(AlignItems::Start)
                .with_horizontal_gap(5)
                .with_vertical_gap(5),
        )
        .child(box1)
        .child(box2)
        .child(box3);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(190, outer.size_ref().width().0);
    assert_eq!(40, outer.size_ref().height().0);
    assert_eq!(190, outer.content_size().unwrap().width().0);
    assert_eq!(40, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(88, box1.size_ref().width().0);
    assert_eq!(10, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(103, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(97, box2.size_ref().width().0);
    assert_eq!(15, box2.size_ref().height().0);

    let box3 = iter.next().unwrap();
    assert_eq!(10, box3.offset_ref().x.0);
    assert_eq!(30, box3.offset_ref().y.0);
    assert_eq!(80, box3.size_ref().width().0);
    assert_eq!(20, box3.size_ref().height().0);
}

#[test]
fn triple_fixed_boxes_g_wrap_grow2() {
    let box1 = DefaultFactory::hbox().size(80).cross_size(10).grow(1);
    let box2 = DefaultFactory::hbox().size(80).cross_size(15).grow(2);
    let box3 = DefaultFactory::hbox().size(80).cross_size(20).grow(2);

    let mut outer = DefaultFactory::hbox()
        .style(
            StyleBuilder::new()
                .with_align_items(AlignItems::Start)
                .with_horizontal_gap(5)
                .with_vertical_gap(5),
        )
        .child(box1)
        .child(box2)
        .child(box3);

    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(190, outer.size_ref().width().0);
    assert_eq!(40, outer.size_ref().height().0);
    assert_eq!(190, outer.content_size().unwrap().width().0);
    assert_eq!(40, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(88, box1.size_ref().width().0);
    assert_eq!(10, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(103, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(97, box2.size_ref().width().0);
    assert_eq!(15, box2.size_ref().height().0);

    let box3 = iter.next().unwrap();
    assert_eq!(10, box3.offset_ref().x.0);
    assert_eq!(30, box3.offset_ref().y.0);
    assert_eq!(190, box3.size_ref().width().0);
    assert_eq!(20, box3.size_ref().height().0);
}

#[test]
fn triple_fixed_boxes_cross_fixed_start() {
    let box1 = DefaultFactory::hbox().size(40).cross_size(20);
    let box2 = DefaultFactory::hbox().size(40).cross_size(30);
    let box3 = DefaultFactory::hbox().size(40).cross_size(15);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(box1)
        .child(box2)
        .child(box3);

    let position = Offset::new(10, 10);
    let size = Size::fixed_depth(190, 277, 3);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(120, outer.size_ref().width().0);
    assert_eq!(30, outer.size_ref().height().0);
    assert_eq!(120, outer.content_size().unwrap().width().0);
    assert_eq!(30, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(40, box1.size_ref().width().0);
    assert_eq!(20, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(50, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(40, box2.size_ref().width().0);
    assert_eq!(30, box2.size_ref().height().0);

    let box3 = iter.next().unwrap();
    assert_eq!(90, box3.offset_ref().x.0);
    assert_eq!(10, box3.offset_ref().y.0);
    assert_eq!(40, box3.size_ref().width().0);
    assert_eq!(15, box3.size_ref().height().0);
}

#[test]
fn triple_fixed_boxes_cross_fixed_center() {
    let box1 = DefaultFactory::hbox().size(40).cross_size(20);
    let box2 = DefaultFactory::hbox().size(40).cross_size(30);
    let box3 = DefaultFactory::hbox().size(40).cross_size(15);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Center))
        .child(box1)
        .child(box2)
        .child(box3);

    let position = Offset::new(10, 10);
    let size = Size::fixed_depth(190, 277, 3);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(120, outer.size_ref().width().0);
    assert_eq!(30, outer.size_ref().height().0);
    assert_eq!(120, outer.content_size().unwrap().width().0);
    assert_eq!(30, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(15, box1.offset_ref().y.0);
    assert_eq!(40, box1.size_ref().width().0);
    assert_eq!(20, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(50, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(40, box2.size_ref().width().0);
    assert_eq!(30, box2.size_ref().height().0);

    let box3 = iter.next().unwrap();
    assert_eq!(90, box3.offset_ref().x.0);
    assert_eq!(18, box3.offset_ref().y.0);
    assert_eq!(40, box3.size_ref().width().0);
    assert_eq!(15, box3.size_ref().height().0);
}

#[test]
fn triple_fixed_boxes_cross_fixed_end() {
    let box1 = DefaultFactory::hbox().size(40).cross_size(20);
    let box2 = DefaultFactory::hbox().size(40).cross_size(30);
    let box3 = DefaultFactory::hbox().size(40).cross_size(15);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::End))
        .child(box1)
        .child(box2)
        .child(box3);

    let position = Offset::new(10, 10);
    let size = Size::fixed_depth(190, 277, 3);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(120, outer.size_ref().width().0);
    assert_eq!(30, outer.size_ref().height().0);
    assert_eq!(120, outer.content_size().unwrap().width().0);
    assert_eq!(30, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(20, box1.offset_ref().y.0);
    assert_eq!(40, box1.size_ref().width().0);
    assert_eq!(20, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(50, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(40, box2.size_ref().width().0);
    assert_eq!(30, box2.size_ref().height().0);

    let box3 = iter.next().unwrap();
    assert_eq!(90, box3.offset_ref().x.0);
    assert_eq!(25, box3.offset_ref().y.0);
    assert_eq!(40, box3.size_ref().width().0);
    assert_eq!(15, box3.size_ref().height().0);
}

#[test]
fn triple_fixed_boxes_cross_fixed_baseline() {
    let box1 = DefaultFactory::hbox().size(40).cross_size(20).depth(5);
    let box2 = DefaultFactory::hbox().size(40).cross_size(30).depth(7);
    let box3 = DefaultFactory::hbox().size(40).cross_size(15).depth(2);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Baseline))
        .child(box1)
        .child(box2)
        .child(box3);

    let position = Offset::new(10, 10);
    let size = Size::fixed_depth(190, 277, 247);
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(120, outer.size_ref().width().0);
    assert_eq!(30, outer.size_ref().height().0);
    assert_eq!(120, outer.content_size().unwrap().width().0);
    assert_eq!(30, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();

    let box1 = iter.next().unwrap();
    assert_eq!(10, box1.offset_ref().x.0);
    assert_eq!(18, box1.offset_ref().y.0);
    assert_eq!(40, box1.size_ref().width().0);
    assert_eq!(20, box1.size_ref().height().0);

    let box2 = iter.next().unwrap();
    assert_eq!(50, box2.offset_ref().x.0);
    assert_eq!(10, box2.offset_ref().y.0);
    assert_eq!(40, box2.size_ref().width().0);
    assert_eq!(30, box2.size_ref().height().0);

    let box3 = iter.next().unwrap();
    assert_eq!(90, box3.offset_ref().x.0);
    assert_eq!(20, box3.offset_ref().y.0);
    assert_eq!(40, box3.size_ref().width().0);
    assert_eq!(15, box3.size_ref().height().0);
}

#[test]
fn surrounded_single_1content_1box1() {
    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);

    let fill1 = DefaultFactory::hfill(1);
    let box1 = DefaultFactory::hbox().size(15);
    let fill2 = DefaultFactory::hfill(1);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(fill1)
        .child(box1)
        .child(fill2);

    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(191, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(191, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let fill1 = iter.next().unwrap();

    assert_eq!(10, fill1.offset_ref().x.0);
    assert_eq!(10, fill1.offset_ref().y.0);
    assert_eq!(88, fill1.content_size().unwrap().width().0);
    assert_eq!(0, fill1.size_ref().height().0);

    let box1 = iter.next().unwrap();

    assert_eq!(98, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);

    let fill2 = iter.next().unwrap();

    assert_eq!(113, fill2.offset_ref().x.0);
    assert_eq!(10, fill2.offset_ref().y.0);
    assert_eq!(88, fill2.content_size().unwrap().width().0);
    assert_eq!(0, fill2.size_ref().height().0);
}

#[test]
fn surrounded_single_1fixed_1box1() {
    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);

    let fill1 = DefaultFactory::hfill(1);
    let box1 = DefaultFactory::hbox().size(15);
    let fill2 = DefaultFactory::hfill(1);

    let mut outer = DefaultFactory::hbox()
        .size(190)
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(fill1)
        .child(box1)
        .child(fill2);

    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(190, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(191, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let fill1 = iter.next().unwrap();

    assert_eq!(10, fill1.offset_ref().x.0);
    assert_eq!(10, fill1.offset_ref().y.0);
    assert_eq!(88, fill1.content_size().unwrap().width().0);
    assert_eq!(0, fill1.size_ref().height().0);

    let box1 = iter.next().unwrap();

    assert_eq!(98, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);

    let fill2 = iter.next().unwrap();

    assert_eq!(113, fill2.offset_ref().x.0);
    assert_eq!(10, fill2.offset_ref().y.0);
    assert_eq!(88, fill2.content_size().unwrap().width().0);
    assert_eq!(0, fill2.size_ref().height().0);
}

#[test]
fn surrounded_single_1parented_1box1() {
    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);

    let fill1 = DefaultFactory::hfill(1);
    let box1 = DefaultFactory::hbox().size(15);
    let fill2 = DefaultFactory::hfill(1);

    let mut outer = DefaultFactory::hbox()
        .size(Fill::full())
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(fill1)
        .child(box1)
        .child(fill2);

    outer.measure(&mut Ctx, size.clone()).unwrap();
    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(190, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(191, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let fill1 = iter.next().unwrap();

    assert_eq!(10, fill1.offset_ref().x.0);
    assert_eq!(10, fill1.offset_ref().y.0);
    assert_eq!(88, fill1.content_size().unwrap().width().0);
    assert_eq!(0, fill1.size_ref().height().0);

    let box1 = iter.next().unwrap();

    assert_eq!(98, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);

    let fill2 = iter.next().unwrap();

    assert_eq!(113, fill2.offset_ref().x.0);
    assert_eq!(10, fill2.offset_ref().y.0);
    assert_eq!(88, fill2.content_size().unwrap().width().0);
    assert_eq!(0, fill2.size_ref().height().0);
}

#[test]
fn surrounded_single_1grow_1box1() {
    let position = Offset::new(10, 10);
    let size = Size::fixed(190, 277);

    let fill1 = DefaultFactory::hfill(1);
    let box1 = DefaultFactory::hbox().size(15);
    let fill2 = DefaultFactory::hfill(1);

    let mut outer = DefaultFactory::hbox()
        .style(StyleBuilder::new().with_align_items(AlignItems::Start))
        .child(fill1)
        .child(box1)
        .child(fill2)
        .grow(1);

    outer.lay_out(&mut Ctx, position, size).unwrap();

    assert_eq!(10, outer.offset_ref().x.0);
    assert_eq!(10, outer.offset_ref().y.0);
    assert_eq!(190, outer.size_ref().width().0);
    assert_eq!(0, outer.size_ref().height().0);
    assert_eq!(191, outer.content_size().unwrap().width().0);
    assert_eq!(0, outer.content_size().unwrap().height().0);

    let mut iter = outer.iter();
    let fill1 = iter.next().unwrap();

    assert_eq!(10, fill1.offset_ref().x.0);
    assert_eq!(10, fill1.offset_ref().y.0);
    assert_eq!(88, fill1.content_size().unwrap().width().0);
    assert_eq!(0, fill1.size_ref().height().0);

    let box1 = iter.next().unwrap();

    assert_eq!(98, box1.offset_ref().x.0);
    assert_eq!(10, box1.offset_ref().y.0);
    assert_eq!(15, box1.size_ref().width().0);
    assert_eq!(0, box1.size_ref().height().0);

    let fill2 = iter.next().unwrap();

    assert_eq!(113, fill2.offset_ref().x.0);
    assert_eq!(10, fill2.offset_ref().y.0);
    assert_eq!(88, fill2.content_size().unwrap().width().0);
    assert_eq!(0, fill2.size_ref().height().0);
}
