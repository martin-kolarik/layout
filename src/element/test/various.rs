use crate::{
    position::{Offset, Size},
    unit::FillPerMille,
    DefaultFactory, Factory, Layout,
};

#[test]
fn layout_box_rows_line() {
    let ctx = &mut 0_usize;

    let cell = DefaultFactory::hbox().size(28);
    let line = DefaultFactory::hbox()
        .child(cell)
        .size(FillPerMille::full());

    let mut outer = DefaultFactory::vbox().child(line);

    outer.measure(ctx, Size::new(190, 277)).unwrap();
    outer
        .lay_out(ctx, Offset::new(10, 10), Size::new(190, 277))
        .unwrap();

    let mut iter = outer.iter();
    let line = iter.next().unwrap();

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

    assert_eq!(28, cell.size_ref().x_dim().basis_size().unwrap().0);
    assert_eq!(0, cell.size_ref().y_dim().basis_size().unwrap().0);
}

#[test]
fn decorated_box_rows_line() {
    let ctx = &mut 0_usize;

    let cell = DefaultFactory::hdbox().size(28);
    let line = DefaultFactory::hdbox()
        .child(cell)
        .size(FillPerMille::full());

    let mut outer = DefaultFactory::vbox().child(line);

    outer.measure(ctx, Size::new(190, 277)).unwrap();
    outer
        .lay_out(ctx, Offset::new(10, 10), Size::new(190, 277))
        .unwrap();

    let mut iter = outer.iter();
    let line = iter.next().unwrap();

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

    assert_eq!(28, cell.size_ref().x_dim().basis_size().unwrap().0);
    assert_eq!(0, cell.size_ref().y_dim().basis_size().unwrap().0);
}
