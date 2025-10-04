use crate::{
    Layout, StyleBuilder, hbox,
    position::{Offset, Quad, Size},
    unit::FillPerMille,
    vbox,
};

#[test]
fn cell_in_rows_line() {
    let ctx = &mut 0_usize;

    let cell = hbox().size(28);
    let line = hbox().child(cell).size(FillPerMille::full());

    let mut outer = vbox().child(line);

    outer.measure(ctx, Size::fixed(190, 277)).unwrap();
    outer
        .lay_out(ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    let mut iter = outer.iter();
    let line = iter.next().unwrap();

    assert_eq!(10, line.offset_ref().x.0);
    assert_eq!(10, line.offset_ref().y.0);
    assert_eq!(190, line.size_ref().x_dim().basis.size().unwrap().0);
    assert_eq!(
        28,
        line.content_size().unwrap().x_dim().basis.size().unwrap().0
    );
    assert_eq!(0, line.size_ref().y_dim().basis.size().unwrap().0);

    let mut iter = line.iter();
    let cell = iter.next().unwrap();

    assert_eq!(10, cell.offset_ref().x.0);
    assert_eq!(10, cell.offset_ref().y.0);
    assert_eq!(28, cell.size_ref().x_dim().basis.size().unwrap().0);
    assert_eq!(0, cell.size_ref().y_dim().basis.size().unwrap().0);
}

#[test]
fn decorated_cell_in_rows_line() {
    let ctx = &mut 0_usize;

    let cell = hbox()
        .size(28)
        .style(StyleBuilder::new().with_padding(Quad::square(4)));
    let line = hbox().child(cell).size(FillPerMille::full());

    let mut outer = vbox().child(line);

    outer.measure(ctx, Size::fixed(190, 277)).unwrap();
    outer
        .lay_out(ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    let mut iter = outer.iter();
    let line = iter.next().unwrap();

    assert_eq!(10, line.offset_ref().x.0);
    assert_eq!(10, line.offset_ref().y.0);
    assert_eq!(190, line.size_ref().x_dim().basis.size().unwrap().0);
    assert_eq!(8, line.size_ref().y_dim().basis.size().unwrap().0);

    let mut iter = line.iter();
    let cell = iter.next().unwrap();

    assert_eq!(10, cell.offset_ref().x.0);
    assert_eq!(10, cell.offset_ref().y.0);
    assert_eq!(28, cell.size_ref().x_dim().basis.size().unwrap().0);
    assert_eq!(8, cell.size_ref().y_dim().basis.size().unwrap().0);
}

#[test]
fn cell_in_rows_decorated_line() {
    let ctx = &mut 0_usize;

    let cell = hbox().size(28);
    let line = hbox()
        .child(cell)
        .size(FillPerMille::full())
        .style(StyleBuilder::new().with_padding(Quad::square(4)));

    let mut outer = vbox().child(line);

    outer.measure(ctx, Size::fixed(190, 277)).unwrap();
    outer
        .lay_out(ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    let mut iter = outer.iter();
    let line = iter.next().unwrap();

    assert_eq!(10, line.offset_ref().x.0);
    assert_eq!(10, line.offset_ref().y.0);
    assert_eq!(190, line.size_ref().x_dim().basis.size().unwrap().0);
    assert_eq!(8, line.size_ref().y_dim().basis.size().unwrap().0);

    let mut iter = line.iter();
    let cell = iter.next().unwrap();

    assert_eq!(14, cell.offset_ref().x.0);
    assert_eq!(14, cell.offset_ref().y.0);
    assert_eq!(28, cell.size_ref().x_dim().basis.size().unwrap().0);
    assert_eq!(0, cell.size_ref().y_dim().basis.size().unwrap().0);
}

#[test]
fn cells_in_rows_decorated_line() {
    let ctx = &mut 0_usize;

    let cell1 = hbox().size(28);
    let cell2 = hbox().size(28);
    let cell3 = hbox().size(28);
    let line = hbox()
        .child(cell1)
        .child(cell2)
        .child(cell3)
        .size(FillPerMille::full())
        .style(StyleBuilder::new().with_padding(Quad::square(4)));

    let mut outer = vbox().child(line);

    outer.measure(ctx, Size::fixed(190, 277)).unwrap();
    outer
        .lay_out(ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    let mut iter = outer.iter();
    let line = iter.next().unwrap();

    assert_eq!(10, line.offset_ref().x.0);
    assert_eq!(10, line.offset_ref().y.0);
    assert_eq!(190, line.size_ref().x_dim().basis.size().unwrap().0);
    assert_eq!(8, line.size_ref().y_dim().basis.size().unwrap().0);

    let mut iter = line.iter();
    let cell1 = iter.next().unwrap();

    assert_eq!(14, cell1.offset_ref().x.0);
    assert_eq!(14, cell1.offset_ref().y.0);
    assert_eq!(28, cell1.size_ref().x_dim().basis.size().unwrap().0);
    assert_eq!(0, cell1.size_ref().y_dim().basis.size().unwrap().0);

    let cell2 = iter.next().unwrap();

    assert_eq!(42, cell2.offset_ref().x.0);
    assert_eq!(14, cell2.offset_ref().y.0);
    assert_eq!(28, cell2.size_ref().x_dim().basis.size().unwrap().0);
    assert_eq!(0, cell2.size_ref().y_dim().basis.size().unwrap().0);

    let cell2 = iter.next().unwrap();

    assert_eq!(70, cell2.offset_ref().x.0);
    assert_eq!(14, cell2.offset_ref().y.0);
    assert_eq!(28, cell2.size_ref().x_dim().basis.size().unwrap().0);
    assert_eq!(0, cell2.size_ref().y_dim().basis.size().unwrap().0);
}
