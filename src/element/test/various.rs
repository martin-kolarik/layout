use crate::{
    position::{Offset, Quad, Size},
    unit::FillPerMille,
    DefaultFactory, Factory, Layout, Style,
};

#[test]
fn cell_in_rows_line() {
    let ctx = &mut 0_usize;

    let cell = DefaultFactory::hbox().size(28);
    let line = DefaultFactory::hbox()
        .child(cell)
        .size(FillPerMille::full());

    let mut outer = DefaultFactory::vbox().child(line);

    outer.measure(ctx, Size::fixed(190, 277)).unwrap();
    outer
        .lay_out(ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    let mut iter = outer.iter();
    let line = iter.next().unwrap();

    assert_eq!(10, line.offset_ref().x().0);
    assert_eq!(10, line.offset_ref().y().0);
    assert_eq!(190, line.size_ref().x_dim().basis_size().unwrap().0);
    assert_eq!(
        28,
        line.native_size().unwrap().x_dim().basis_size().unwrap().0
    );
    assert_eq!(
        28,
        line.content_size().unwrap().x_dim().basis_size().unwrap().0
    );
    assert_eq!(0, line.size_ref().y_dim().basis_size().unwrap().0);

    let mut iter = line.iter();
    let cell = iter.next().unwrap();

    assert_eq!(10, cell.offset_ref().x().0);
    assert_eq!(10, cell.offset_ref().y().0);
    assert_eq!(28, cell.size_ref().x_dim().basis_size().unwrap().0);
    assert_eq!(0, cell.size_ref().y_dim().basis_size().unwrap().0);
}

#[test]
fn decorated_cell_in_rows_line() {
    let ctx = &mut 0_usize;

    let cell = DefaultFactory::hbox()
        .size(28)
        .style(Style::new().with_padding(Quad::square(4)));
    let line = DefaultFactory::hbox()
        .child(cell)
        .size(FillPerMille::full());

    let mut outer = DefaultFactory::vbox().child(line);

    outer.measure(ctx, Size::fixed(190, 277)).unwrap();
    outer
        .lay_out(ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    let mut iter = outer.iter();
    let line = iter.next().unwrap();

    assert_eq!(10, line.offset_ref().x().0);
    assert_eq!(10, line.offset_ref().y().0);
    assert_eq!(190, line.size_ref().x_dim().basis_size().unwrap().0);
    assert_eq!(
        28,
        line.native_size().unwrap().x_dim().basis_size().unwrap().0
    );
    assert_eq!(8, line.size_ref().y_dim().basis_size().unwrap().0);

    let mut iter = line.iter();
    let cell = iter.next().unwrap();

    assert_eq!(10, cell.offset_ref().x().0);
    assert_eq!(10, cell.offset_ref().y().0);
    assert_eq!(28, cell.size_ref().x_dim().basis_size().unwrap().0);
    assert_eq!(8, cell.size_ref().y_dim().basis_size().unwrap().0);
}

#[test]
fn cell_in_rows_decorated_line() {
    let ctx = &mut 0_usize;

    let cell = DefaultFactory::hbox().size(28);
    let line = DefaultFactory::hbox()
        .child(cell)
        .size(FillPerMille::full())
        .style(Style::new().with_padding(Quad::square(4)));

    let mut outer = DefaultFactory::vbox().child(line);

    outer.measure(ctx, Size::fixed(190, 277)).unwrap();
    outer
        .lay_out(ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    let mut iter = outer.iter();
    let line = iter.next().unwrap();

    assert_eq!(10, line.offset_ref().x().0);
    assert_eq!(10, line.offset_ref().y().0);
    assert_eq!(190, line.size_ref().x_dim().basis_size().unwrap().0);
    assert_eq!(
        36,
        line.native_size().unwrap().x_dim().basis_size().unwrap().0
    );
    assert_eq!(8, line.size_ref().y_dim().basis_size().unwrap().0);

    let mut iter = line.iter();
    let cell = iter.next().unwrap();

    assert_eq!(14, cell.offset_ref().x().0);
    assert_eq!(14, cell.offset_ref().y().0);
    assert_eq!(28, cell.size_ref().x_dim().basis_size().unwrap().0);
    assert_eq!(0, cell.size_ref().y_dim().basis_size().unwrap().0);
}

#[test]
fn cells_in_rows_decorated_line() {
    let ctx = &mut 0_usize;

    let cell1 = DefaultFactory::hbox().size(28);
    let cell2 = DefaultFactory::hbox().size(28);
    let cell3 = DefaultFactory::hbox().size(28);
    let line = DefaultFactory::hbox()
        .child(cell1)
        .child(cell2)
        .child(cell3)
        .size(FillPerMille::full())
        .style(Style::new().with_padding(Quad::square(4)));

    let mut outer = DefaultFactory::vbox().child(line);

    outer.measure(ctx, Size::fixed(190, 277)).unwrap();
    outer
        .lay_out(ctx, Offset::new(10, 10), Size::fixed(190, 277))
        .unwrap();

    let mut iter = outer.iter();
    let line = iter.next().unwrap();

    assert_eq!(10, line.offset_ref().x().0);
    assert_eq!(10, line.offset_ref().y().0);
    assert_eq!(190, line.size_ref().x_dim().basis_size().unwrap().0);
    assert_eq!(
        92,
        line.native_size().unwrap().x_dim().basis_size().unwrap().0
    );
    assert_eq!(8, line.size_ref().y_dim().basis_size().unwrap().0);

    let mut iter = line.iter();
    let cell1 = iter.next().unwrap();

    assert_eq!(14, cell1.offset_ref().x().0);
    assert_eq!(14, cell1.offset_ref().y().0);
    assert_eq!(28, cell1.size_ref().x_dim().basis_size().unwrap().0);
    assert_eq!(0, cell1.size_ref().y_dim().basis_size().unwrap().0);

    let cell2 = iter.next().unwrap();

    assert_eq!(42, cell2.offset_ref().x().0);
    assert_eq!(14, cell2.offset_ref().y().0);
    assert_eq!(28, cell2.size_ref().x_dim().basis_size().unwrap().0);
    assert_eq!(0, cell2.size_ref().y_dim().basis_size().unwrap().0);

    let cell2 = iter.next().unwrap();

    assert_eq!(70, cell2.offset_ref().x().0);
    assert_eq!(14, cell2.offset_ref().y().0);
    assert_eq!(28, cell2.size_ref().x_dim().basis_size().unwrap().0);
    assert_eq!(0, cell2.size_ref().y_dim().basis_size().unwrap().0);
}
