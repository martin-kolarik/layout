use std::sync::Arc;

use crate::{
    dimension::DimAutoOrParent,
    position::{Offset, Size},
    unit::Fill,
    Axis, Error, Layout, MeasureContext, Position, Style, Styled,
};

pub struct Filling {
    mark: Option<&'static str>,
    axis: Axis,
    offset: Offset,
    size: Size,
    style: Arc<Style>,
    content_size: Option<Size>,
}

impl Filling {
    pub fn new(axis: Axis) -> Self {
        Self {
            mark: None,
            axis,
            offset: Offset::zero(),
            size: Size::zero(),
            style: Style::new(),
            content_size: None,
        }
    }

    pub fn mark(mut self, mark: &'static str) -> Self {
        self.mark = Some(mark);
        self
    }

    pub fn size(mut self, size: impl Into<DimAutoOrParent>) -> Self {
        self.axis.dim_mut(&mut self.size).set_basis(size);
        self
    }

    pub fn grow(mut self, weight: impl Into<Fill>) -> Self {
        self.axis.dim_mut(&mut self.size).set_grow(weight);
        self
    }

    pub fn shrink(mut self, weight: impl Into<Fill>) -> Self {
        self.axis.dim_mut(&mut self.size).set_shrink(weight);
        self
    }
}

impl Position for Filling {
    fn element(&self) -> &str {
        "Fill"
    }

    fn mark(&self) -> &str {
        self.mark.unwrap_or_default()
    }

    fn offset_ref(&self) -> &Offset {
        &self.offset
    }

    fn offset_mut(&mut self) -> &mut Offset {
        &mut self.offset
    }

    fn size_ref(&self) -> &Size {
        &self.size
    }

    fn size_mut(&mut self) -> &mut Size {
        &mut self.size
    }

    fn content_size(&self) -> Option<&Size> {
        self.content_size.as_ref()
    }
}

impl Styled for Filling {
    fn style_ref(&self) -> &Style {
        self.style.as_ref()
    }

    fn set_style(&mut self, style: Arc<Style>) {
        self.size.apply_style(self.axis, &style);
        self.style = style;
    }
}

impl Layout for Filling {
    fn lay_out(
        &mut self,
        _: &mut dyn MeasureContext,
        position: Offset,
        size: Size,
    ) -> Result<(), Error> {
        self.offset = position;
        self.content_size = Some(size);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        position::{Offset, Size},
        test::Ctx,
        DefaultFactory, Factory, Layout,
    };

    #[test]
    fn h_center() {
        let mut outer = DefaultFactory::hbox()
            .size(100)
            .child(DefaultFactory::hfilling().grow(2))
            .child(DefaultFactory::hbox().size(25))
            .child(DefaultFactory::hfilling().grow(1));

        outer
            .lay_out(
                &mut Ctx,
                Offset::new(10, 10),
                Size::fixed_depth(190, 277, 267),
            )
            .unwrap();

        let mut iter = outer.iter();

        let fill = iter.next().unwrap();
        assert_eq!(10, fill.offset_ref().x().0);
        assert_eq!(10, fill.offset_ref().y().0);
        assert_eq!(0, fill.size_ref().width().0);
        assert_eq!(50, fill.content_size().unwrap().width().0);
        assert_eq!(0, fill.size_ref().height().0);

        let box1 = iter.next().unwrap();
        assert_eq!(60, box1.offset_ref().x().0);
        assert_eq!(10, box1.offset_ref().y().0);
        assert_eq!(25, box1.size_ref().width().0);
        assert_eq!(0, box1.size_ref().height().0);

        let fill = iter.next().unwrap();
        assert_eq!(85, fill.offset_ref().x().0);
        assert_eq!(10, fill.offset_ref().y().0);
        assert_eq!(0, fill.size_ref().width().0);
        assert_eq!(25, fill.content_size().unwrap().width().0);
        assert_eq!(0, fill.size_ref().height().0);
    }

    #[test]
    fn v_center_auto_vbox_width_nomeasure() {
        let mut outer = DefaultFactory::hbox()
            .size(100)
            .child(DefaultFactory::hfilling().grow(2))
            .child(
                DefaultFactory::vbox()
                    .size(100)
                    .child(DefaultFactory::vfilling().grow(1))
                    .child(DefaultFactory::hbox().size(5))
                    .child(DefaultFactory::vfilling().grow(1)),
            )
            .child(DefaultFactory::hfilling().grow(1));

        outer
            .lay_out(
                &mut Ctx,
                Offset::new(10, 10),
                Size::fixed_depth(190, 277, 267),
            )
            .unwrap();

        let mut iter = outer.iter();

        let fill = iter.next().unwrap();
        assert_eq!(10, fill.offset_ref().x().0);
        assert_eq!(10, fill.offset_ref().y().0);
        assert_eq!(0, fill.size_ref().width().0);
        assert_eq!(67, fill.content_size().unwrap().width().0);
        assert_eq!(0, fill.size_ref().height().0);

        let box1 = iter.next().unwrap();
        assert_eq!(77, box1.offset_ref().x().0);
        assert_eq!(10, box1.offset_ref().y().0);
        assert_eq!(5, box1.size_ref().width().0);
        assert_eq!(100, box1.size_ref().height().0);

        {
            let mut iter = box1.iter();

            let fill = iter.next().unwrap();
            assert_eq!(77, fill.offset_ref().x().0);
            assert_eq!(10, fill.offset_ref().y().0);
            assert_eq!(0, fill.size_ref().width().0);
            assert_eq!(50, fill.content_size().unwrap().height().0);

            let box1 = iter.next().unwrap();
            assert_eq!(77, box1.offset_ref().x().0);
            assert_eq!(60, box1.offset_ref().y().0);
            assert_eq!(5, box1.size_ref().width().0);
            assert_eq!(0, box1.size_ref().height().0);

            let fill = iter.next().unwrap();
            assert_eq!(77, fill.offset_ref().x().0);
            assert_eq!(60, fill.offset_ref().y().0);
            assert_eq!(0, fill.size_ref().width().0);
            assert_eq!(50, fill.content_size().unwrap().height().0);
        }

        let fill = iter.next().unwrap();
        assert_eq!(77, fill.offset_ref().x().0);
        assert_eq!(10, fill.offset_ref().y().0);
        assert_eq!(33, fill.content_size().unwrap().width().0);
        assert_eq!(0, fill.size_ref().height().0);
    }

    #[test]
    fn v_center_auto_vbox_width() {
        let mut outer = DefaultFactory::hbox()
            .size(100)
            .child(DefaultFactory::hfilling().grow(2))
            .child(
                DefaultFactory::vbox()
                    .size(100)
                    .child(DefaultFactory::vfilling().grow(1))
                    .child(DefaultFactory::hbox().size(5))
                    .child(DefaultFactory::vfilling().grow(1)),
            )
            .child(DefaultFactory::hfilling().grow(1));

        let size = Size::fixed_depth(190, 277, 267);
        outer.measure(&mut Ctx, size.clone()).unwrap();
        outer.lay_out(&mut Ctx, Offset::new(10, 10), size).unwrap();

        let mut iter = outer.iter();

        let fill = iter.next().unwrap();
        assert_eq!(10, fill.offset_ref().x().0);
        assert_eq!(10, fill.offset_ref().y().0);
        assert_eq!(0, fill.size_ref().width().0);
        assert_eq!(63, fill.content_size().unwrap().width().0);
        assert_eq!(0, fill.size_ref().height().0);

        let box1 = iter.next().unwrap();
        assert_eq!(73, box1.offset_ref().x().0);
        assert_eq!(10, box1.offset_ref().y().0);
        assert_eq!(5, box1.size_ref().width().0);
        assert_eq!(100, box1.size_ref().height().0);

        {
            let mut iter = box1.iter();

            let fill = iter.next().unwrap();
            assert_eq!(73, fill.offset_ref().x().0);
            assert_eq!(10, fill.offset_ref().y().0);
            assert_eq!(0, fill.size_ref().width().0);
            assert_eq!(50, fill.content_size().unwrap().height().0);

            let box1 = iter.next().unwrap();
            assert_eq!(73, box1.offset_ref().x().0);
            assert_eq!(60, box1.offset_ref().y().0);
            assert_eq!(5, box1.size_ref().width().0);
            assert_eq!(0, box1.size_ref().height().0);

            let fill = iter.next().unwrap();
            assert_eq!(73, fill.offset_ref().x().0);
            assert_eq!(60, fill.offset_ref().y().0);
            assert_eq!(0, fill.size_ref().width().0);
            assert_eq!(50, fill.content_size().unwrap().height().0);
        }

        let fill = iter.next().unwrap();
        assert_eq!(78, fill.offset_ref().x().0);
        assert_eq!(10, fill.offset_ref().y().0);
        assert_eq!(32, fill.content_size().unwrap().width().0);
        assert_eq!(0, fill.size_ref().height().0);
    }

    #[test]
    fn v_center_zero_vbox_width() {
        let mut outer = DefaultFactory::hbox()
            .size(100)
            .child(DefaultFactory::hfilling().grow(2))
            .child(
                DefaultFactory::vbox()
                    .size(100)
                    .cross_size(0)
                    .child(DefaultFactory::vfilling().grow(1))
                    .child(DefaultFactory::hbox().size(5))
                    .child(DefaultFactory::vfilling().grow(1)),
            )
            .child(DefaultFactory::hfilling().grow(1));

        outer
            .lay_out(
                &mut Ctx,
                Offset::new(10, 10),
                Size::fixed_depth(190, 277, 267),
            )
            .unwrap();

        let mut iter = outer.iter();

        let fill = iter.next().unwrap();
        assert_eq!(10, fill.offset_ref().x().0);
        assert_eq!(10, fill.offset_ref().y().0);
        assert_eq!(67, fill.content_size().unwrap().width().0);
        assert_eq!(0, fill.size_ref().height().0);

        let box1 = iter.next().unwrap();
        assert_eq!(77, box1.offset_ref().x().0);
        assert_eq!(10, box1.offset_ref().y().0);
        assert_eq!(0, box1.size_ref().width().0);
        assert_eq!(100, box1.size_ref().height().0);

        {
            let mut iter = box1.iter();

            let fill = iter.next().unwrap();
            assert_eq!(77, fill.offset_ref().x().0);
            assert_eq!(10, fill.offset_ref().y().0);
            assert_eq!(0, fill.size_ref().width().0);
            assert_eq!(50, fill.content_size().unwrap().height().0);

            let box1 = iter.next().unwrap();
            assert_eq!(77, box1.offset_ref().x().0);
            assert_eq!(60, box1.offset_ref().y().0);
            assert_eq!(5, box1.size_ref().width().0);
            assert_eq!(0, box1.size_ref().height().0);

            let fill = iter.next().unwrap();
            assert_eq!(77, fill.offset_ref().x().0);
            assert_eq!(60, fill.offset_ref().y().0);
            assert_eq!(0, fill.size_ref().width().0);
            assert_eq!(50, fill.content_size().unwrap().height().0);
        }

        let fill = iter.next().unwrap();
        assert_eq!(77, fill.offset_ref().x().0);
        assert_eq!(10, fill.offset_ref().y().0);
        assert_eq!(33, fill.content_size().unwrap().width().0);
        assert_eq!(0, fill.size_ref().height().0);
    }
}
