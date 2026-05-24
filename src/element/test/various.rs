use crate::{
    Axis, Layout, StyleBuilder,
    children::lay_out_native,
    hbox, hwrap,
    position::{Offset, Quad, Size},
    unit::FillPerMille,
    vbox,
};

#[test]
fn cell_in_rows_line() {
    let ctx = &mut 0_usize;

    let cell = hbox().axis_size(28);
    let line = hbox().child(cell).axis_size(FillPerMille::full());

    let mut outer = vbox().child(line);

    outer.measure(ctx, Size::fixed(190, 277)).unwrap();
    outer
        .lay_out(ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    let mut iter = outer.iter();
    let line = iter.next().unwrap();

    assert_eq!(10, line.offset().x.0);
    assert_eq!(10, line.offset().y.0);
    assert_eq!(190, line.size().width.base.size().unwrap().0);
    assert_eq!(
        28,
        line.content_size().unwrap().width.base.size().unwrap().0
    );
    assert_eq!(0, line.size().height.base.size().unwrap().0);

    let mut iter = line.iter();
    let cell = iter.next().unwrap();

    assert_eq!(10, cell.offset().x.0);
    assert_eq!(10, cell.offset().y.0);
    assert_eq!(28, cell.size().width.base.size().unwrap().0);
    assert_eq!(0, cell.size().height.base.size().unwrap().0);
}

#[test]
fn decorated_cell_in_rows_line() {
    let ctx = &mut 0_usize;

    let cell = hbox()
        .axis_size(28)
        .style(StyleBuilder::new().with_padding(Quad::square(4)));
    let line = hbox().child(cell).axis_size(FillPerMille::full());

    let mut outer = vbox().child(line);

    outer.measure(ctx, Size::fixed(190, 277)).unwrap();
    outer
        .lay_out(ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    let mut iter = outer.iter();
    let line = iter.next().unwrap();

    assert_eq!(10, line.offset().x.0);
    assert_eq!(10, line.offset().y.0);
    assert_eq!(190, line.size().width.base.size().unwrap().0);
    assert_eq!(8, line.size().height.base.size().unwrap().0);

    let mut iter = line.iter();
    let cell = iter.next().unwrap();

    assert_eq!(10, cell.offset().x.0);
    assert_eq!(10, cell.offset().y.0);
    assert_eq!(28, cell.size().width.base.size().unwrap().0);
    assert_eq!(8, cell.size().height.base.size().unwrap().0);
}

#[test]
fn cell_in_rows_decorated_line() {
    let ctx = &mut 0_usize;

    let cell = hbox().axis_size(28);
    let line = hbox()
        .child(cell)
        .axis_size(FillPerMille::full())
        .style(StyleBuilder::new().with_padding(Quad::square(4)));

    let mut outer = vbox().child(line);

    outer.measure(ctx, Size::fixed(190, 277)).unwrap();
    outer
        .lay_out(ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    let mut iter = outer.iter();
    let line = iter.next().unwrap();

    assert_eq!(10, line.offset().x.0);
    assert_eq!(10, line.offset().y.0);
    assert_eq!(190, line.size().width.base.size().unwrap().0);
    assert_eq!(8, line.size().height.base.size().unwrap().0);

    let mut iter = line.iter();
    let cell = iter.next().unwrap();

    assert_eq!(14, cell.offset().x.0);
    assert_eq!(14, cell.offset().y.0);
    assert_eq!(28, cell.size().width.base.size().unwrap().0);
    assert_eq!(0, cell.size().height.base.size().unwrap().0);
}

#[test]
fn cells_in_rows_decorated_line() {
    let ctx = &mut 0_usize;

    let cell1 = hbox().axis_size(28);
    let cell2 = hbox().axis_size(28);
    let cell3 = hbox().axis_size(28);
    let line = hbox()
        .child(cell1)
        .child(cell2)
        .child(cell3)
        .axis_size(FillPerMille::full())
        .style(StyleBuilder::new().with_padding(Quad::square(4)));

    let mut outer = vbox().child(line);

    outer.measure(ctx, Size::fixed(190, 277)).unwrap();
    outer
        .lay_out(ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    let mut iter = outer.iter();
    let line = iter.next().unwrap();

    assert_eq!(10, line.offset().x.0);
    assert_eq!(10, line.offset().y.0);
    assert_eq!(190, line.size().width.base.size().unwrap().0);
    assert_eq!(8, line.size().height.base.size().unwrap().0);

    let mut iter = line.iter();
    let cell1 = iter.next().unwrap();

    assert_eq!(14, cell1.offset().x.0);
    assert_eq!(14, cell1.offset().y.0);
    assert_eq!(28, cell1.size().width.base.size().unwrap().0);
    assert_eq!(0, cell1.size().height.base.size().unwrap().0);

    let cell2 = iter.next().unwrap();

    assert_eq!(42, cell2.offset().x.0);
    assert_eq!(14, cell2.offset().y.0);
    assert_eq!(28, cell2.size().width.base.size().unwrap().0);
    assert_eq!(0, cell2.size().height.base.size().unwrap().0);

    let cell2 = iter.next().unwrap();

    assert_eq!(70, cell2.offset().x.0);
    assert_eq!(14, cell2.offset().y.0);
    assert_eq!(28, cell2.size().width.base.size().unwrap().0);
    assert_eq!(0, cell2.size().height.base.size().unwrap().0);
}

#[test]
fn wrap_in_hbox_without_wrap1() {
    let box1 = hbox().axis_size(5).cross_size(4);
    let box2 = hbox().axis_size(5).cross_size(6);

    let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

    let output = lay_out_native(Axis::Horizontal, &mut children, 15, 4, 4, true, true);

    assert_eq!(1, output.len());
    assert_eq!(2, output[0].len());

    assert_eq!(0, output[0].offset().0);
    assert_eq!(14, output[0].size().base_width().0);
    assert_eq!(6, output[0].size().base_height().0);

    let item = &output[0].content()[0];
    assert_eq!(0, item.offset().x.0);
    assert_eq!(0, item.offset().y.0);
    assert_eq!(5, item.size().base_width().0);
    assert_eq!(4, item.size().base_height().0);

    let item = &output[0].content()[1];
    assert_eq!(9, item.offset().x.0);
    assert_eq!(0, item.offset().y.0);
    assert_eq!(5, item.size().base_width().0);
    assert_eq!(6, item.size().base_height().0);
}

#[test]
fn wrap_in_hbox_without_wrap2() {
    let box1 = hbox().axis_size(10).cross_size(4);
    let box2 = hbox().axis_size(15).cross_size(6);

    let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

    let output = lay_out_native(Axis::Horizontal, &mut children, 15, 4, 4, true, true);

    assert_eq!(2, output.len());
    assert_eq!(1, output[0].len());

    assert_eq!(0, output[0].offset().0);
    assert_eq!(10, output[0].size().base_width().0);
    assert_eq!(4, output[0].size().base_height().0);

    assert_eq!(8, output[1].offset().0);
    assert_eq!(15, output[1].size().base_width().0);
    assert_eq!(6, output[1].size().base_height().0);

    let item = &output[0].content()[0];
    assert_eq!(0, item.offset().x.0);
    assert_eq!(0, item.offset().y.0);
    assert_eq!(10, item.size().base_width().0);
    assert_eq!(4, item.size().base_height().0);

    let item = &output[1].content()[0];
    assert_eq!(0, item.offset().x.0);
    assert_eq!(8, item.offset().y.0);
    assert_eq!(15, item.size().base_width().0);
    assert_eq!(6, item.size().base_height().0);
}

#[test]
fn wrap_in_hbox_with_wrap() {
    let box1 = hbox().axis_size(5).cross_size(4);
    let wrap = hwrap();
    let box2 = hbox().axis_size(5).cross_size(6);

    let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(wrap), Box::new(box2)];

    let output = lay_out_native(Axis::Horizontal, &mut children, 15, 4, 4, true, true);

    assert_eq!(2, output.len());
    assert_eq!(1, output[0].len());

    assert_eq!(0, output[0].offset().0);
    assert_eq!(5, output[0].size().base_width().0);
    assert_eq!(4, output[0].size().base_height().0);

    assert_eq!(8, output[1].offset().0);
    assert_eq!(5, output[1].size().base_width().0);
    assert_eq!(6, output[1].size().base_height().0);

    let item = &output[0].content()[0];
    assert_eq!(0, item.offset().x.0);
    assert_eq!(0, item.offset().y.0);
    assert_eq!(5, item.size().base_width().0);
    assert_eq!(4, item.size().base_height().0);

    let item = &output[1].content()[0];
    assert_eq!(0, item.offset().x.0);
    assert_eq!(8, item.offset().y.0);
    assert_eq!(5, item.size().base_width().0);
    assert_eq!(6, item.size().base_height().0);
}
