use crate::{Axis, Layout, unit::Unit};

use super::position::{Offset, Size};

#[allow(clippy::borrowed_box)]
#[allow(dead_code)]

pub struct Line<'a> {
    offset: Unit,
    size: Size,
    content: Vec<&'a mut Box<dyn Layout>>,
}

#[allow(clippy::borrowed_box)]

impl<'a> Line<'a> {
    fn new(offset: Unit, size: Size, content: Vec<&'a mut Box<dyn Layout>>) -> Self {
        Self {
            offset,
            size,
            content,
        }
    }

    #[cfg(test)]
    pub fn offset(&self) -> Unit {
        self.offset
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.content.len()
    }

    #[cfg(test)]
    pub fn content(&'a self) -> &[&mut Box<dyn Layout>] {
        &self.content
    }

    pub fn content_mut(&'a mut self) -> &'a mut [&'a mut Box<dyn Layout>] {
        &mut self.content
    }
}

pub fn lay_out_native<'a>(
    axis: Axis,
    children: &'a mut [Box<dyn Layout>],
    axis_room: impl Into<Unit>,
    axis_gap: impl Into<Unit>,
    cross_gap: impl Into<Unit>,
    wrap: bool,
    respect_baseline: bool,
) -> Vec<Line<'a>> {
    if children.is_empty() {
        return vec![];
    }

    let wrap_size = axis_room.into();
    let axis_gap = axis_gap.into();
    let cross_gap = cross_gap.into();
    let mut lines = vec![];
    let mut remaining = children.len();

    let mut offset = Offset::zero();
    let mut line_size = Size::zero();
    let mut line = Vec::with_capacity(remaining - 1);

    for child in children {
        let line_gap = if line.is_empty() {
            Unit::zero()
        } else {
            axis_gap
        };

        if wrap && axis.size(&line_size) + line_gap + axis.size(child.size_ref()) > wrap_size {
            let next_line_offset = axis
                .cross()
                .advance_dim(&offset, axis.cross().size(&line_size) + cross_gap);

            remaining -= line.len();
            lines.push(Line::new(axis.cross().offset(&offset), line_size, line));

            offset = next_line_offset;
            axis.set_offset(&mut offset, Unit::zero());

            line_size = Size::zero();
            line = Vec::with_capacity(remaining);
        } else if line_gap > Unit::zero() {
            offset = axis.advance_dim(&offset, line_gap);
            line_size = axis.extend_dim(&line_size, line_gap);
        }
        line_size = axis.extend_size(&line_size, child.size_after_wrap_ref(), respect_baseline);

        *child.offset_mut() = offset.clone();
        offset = axis.advance_dim(&offset, axis.size(child.size_after_wrap_ref()));

        line.push(child);
    }

    if !line.is_empty() {
        lines.push(Line::new(axis.cross().offset(&offset), line_size, line));
    }

    lines
}

#[cfg(test)]
mod tests {

    mod hbox {
        use crate::{Axis, DefaultFactory, Factory, Layout, children::lay_out_native};

        #[test]
        fn single_box() {
            let box1 = DefaultFactory::hbox().size(10);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 100, 0, 0, true, true);

            assert_eq!(1, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(10, output[0].size().width().0);
            assert_eq!(0, output[0].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(0, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes() {
            let box1 = DefaultFactory::hbox().size(10);
            let box2 = DefaultFactory::hbox().size(15);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 100, 0, 0, true, true);

            assert_eq!(1, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(25, output[0].size().width().0);
            assert_eq!(0, output[0].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(0, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(10, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(0, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes_gap() {
            let box1 = DefaultFactory::hbox().size(10);
            let box2 = DefaultFactory::hbox().size(15);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 100, 3, 3, true, true);

            assert_eq!(1, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(28, output[0].size().width().0);
            assert_eq!(0, output[0].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(0, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(13, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(0, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes_wrap() {
            let box1 = DefaultFactory::hbox().size(10).cross_size(4);
            let box2 = DefaultFactory::hbox().size(15).cross_size(6);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 15, 0, 0, true, true);

            assert_eq!(2, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(10, output[0].size().width().0);
            assert_eq!(4, output[0].size().height().0);

            assert_eq!(4, output[1].offset().0);
            assert_eq!(15, output[1].size().width().0);
            assert_eq!(6, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(4, item.offset_ref().y.0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(6, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes_wrap_gap() {
            let box1 = DefaultFactory::hbox().size(10).cross_size(4);
            let box2 = DefaultFactory::hbox().size(15).cross_size(6);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 15, 4, 4, true, true);

            assert_eq!(2, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(10, output[0].size().width().0);
            assert_eq!(4, output[0].size().height().0);

            assert_eq!(8, output[1].offset().0);
            assert_eq!(15, output[1].size().width().0);
            assert_eq!(6, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(8, item.offset_ref().y.0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(6, item.size_ref().height().0);
        }

        #[test]
        fn three_boxes_wrap_exact_1() {
            let box1 = DefaultFactory::hbox().size(10).cross_size(4);
            let box2 = DefaultFactory::hbox().size(15).cross_size(6);
            let box3 = DefaultFactory::hbox().size(10).cross_size(8);

            let mut children: Vec<Box<dyn Layout>> =
                vec![Box::new(box1), Box::new(box2), Box::new(box3)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 24, 0, 0, true, true);

            assert_eq!(3, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(10, output[0].size().width().0);
            assert_eq!(4, output[0].size().height().0);

            assert_eq!(4, output[1].offset().0);
            assert_eq!(15, output[1].size().width().0);
            assert_eq!(6, output[1].size().height().0);

            assert_eq!(10, output[2].offset().0);
            assert_eq!(10, output[2].size().width().0);
            assert_eq!(8, output[2].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(4, item.offset_ref().y.0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(6, item.size_ref().height().0);

            let item = &output[2].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(10, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(8, item.size_ref().height().0);
        }

        #[test]
        fn three_boxes_wrap_exact_2() {
            let box1 = DefaultFactory::hbox().size(10).cross_size(4);
            let box2 = DefaultFactory::hbox().size(15).cross_size(6);
            let box3 = DefaultFactory::hbox().size(10).cross_size(8);

            let mut children: Vec<Box<dyn Layout>> =
                vec![Box::new(box1), Box::new(box2), Box::new(box3)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 25, 0, 0, true, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(25, output[0].size().width().0);
            assert_eq!(6, output[0].size().height().0);

            assert_eq!(6, output[1].offset().0);
            assert_eq!(10, output[1].size().width().0);
            assert_eq!(8, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(10, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(6, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(6, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(8, item.size_ref().height().0);
        }

        #[test]
        fn three_boxes_wrap_exact_3() {
            let box1 = DefaultFactory::hbox().size(10).cross_size(4);
            let box2 = DefaultFactory::hbox().size(15).cross_size(6);
            let box3 = DefaultFactory::hbox().size(10).cross_size(8);

            let mut children: Vec<Box<dyn Layout>> =
                vec![Box::new(box1), Box::new(box2), Box::new(box3)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 26, 0, 0, true, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(25, output[0].size().width().0);
            assert_eq!(6, output[0].size().height().0);

            assert_eq!(6, output[1].offset().0);
            assert_eq!(10, output[1].size().width().0);
            assert_eq!(8, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(10, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(6, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(6, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(8, item.size_ref().height().0);
        }

        #[test]
        fn three_boxes_wrap_gap_exact_1() {
            let box1 = DefaultFactory::hbox().size(10).cross_size(4);
            let box2 = DefaultFactory::hbox().size(15).cross_size(6);
            let box3 = DefaultFactory::hbox().size(10).cross_size(8);

            let mut children: Vec<Box<dyn Layout>> =
                vec![Box::new(box1), Box::new(box2), Box::new(box3)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 27, 3, 3, true, true);

            assert_eq!(3, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(10, output[0].size().width().0);
            assert_eq!(4, output[0].size().height().0);

            assert_eq!(7, output[1].offset().0);
            assert_eq!(15, output[1].size().width().0);
            assert_eq!(6, output[1].size().height().0);

            assert_eq!(16, output[2].offset().0);
            assert_eq!(10, output[2].size().width().0);
            assert_eq!(8, output[2].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(7, item.offset_ref().y.0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(6, item.size_ref().height().0);

            let item = &output[2].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(16, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(8, item.size_ref().height().0);
        }

        #[test]
        fn three_boxes_wrap_gap_exact_2() {
            let box1 = DefaultFactory::hbox().size(10).cross_size(4);
            let box2 = DefaultFactory::hbox().size(15).cross_size(6);
            let box3 = DefaultFactory::hbox().size(10).cross_size(8);

            let mut children: Vec<Box<dyn Layout>> =
                vec![Box::new(box1), Box::new(box2), Box::new(box3)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 28, 3, 3, true, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(28, output[0].size().width().0);
            assert_eq!(6, output[0].size().height().0);

            assert_eq!(9, output[1].offset().0);
            assert_eq!(10, output[1].size().width().0);
            assert_eq!(8, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(13, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(6, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(9, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(8, item.size_ref().height().0);
        }

        #[test]
        fn three_boxes_wrap_gap_exact_3() {
            let box1 = DefaultFactory::hbox().size(10).cross_size(4);
            let box2 = DefaultFactory::hbox().size(15).cross_size(6);
            let box3 = DefaultFactory::hbox().size(10).cross_size(8);

            let mut children: Vec<Box<dyn Layout>> =
                vec![Box::new(box1), Box::new(box2), Box::new(box3)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 29, 3, 3, true, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(28, output[0].size().width().0);
            assert_eq!(6, output[0].size().height().0);

            assert_eq!(9, output[1].offset().0);
            assert_eq!(10, output[1].size().width().0);
            assert_eq!(8, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(13, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(6, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(9, item.offset_ref().y.0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(8, item.size_ref().height().0);
        }
    }

    mod vbox {
        use crate::{Axis, DefaultFactory, Factory, Layout, children::lay_out_native};

        #[test]
        fn single_box() {
            let box1 = DefaultFactory::vbox().size(10);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1)];

            let output = lay_out_native(Axis::Vertical, &mut children, 100, 0, 0, true, true);

            assert_eq!(1, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(0, output[0].size().width().0);
            assert_eq!(10, output[0].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(0, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes() {
            let box1 = DefaultFactory::vbox().size(10);
            let box2 = DefaultFactory::vbox().size(15);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Vertical, &mut children, 100, 0, 0, true, true);

            assert_eq!(1, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(0, output[0].size().width().0);
            assert_eq!(25, output[0].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(0, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(10, item.offset_ref().y.0);
            assert_eq!(0, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes_gap() {
            let box1 = DefaultFactory::vbox().size(10);
            let box2 = DefaultFactory::vbox().size(15);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Vertical, &mut children, 100, 3, 3, true, true);

            assert_eq!(1, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(0, output[0].size().width().0);
            assert_eq!(28, output[0].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(0, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(13, item.offset_ref().y.0);
            assert_eq!(0, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes_wrap() {
            let box1 = DefaultFactory::vbox().size(10).cross_size(4);
            let box2 = DefaultFactory::vbox().size(15).cross_size(6);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Vertical, &mut children, 15, 0, 0, true, true);

            assert_eq!(2, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(4, output[0].size().width().0);
            assert_eq!(10, output[0].size().height().0);

            assert_eq!(4, output[1].offset().0);
            assert_eq!(6, output[1].size().width().0);
            assert_eq!(15, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(4, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(6, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes_wrap_gap() {
            let box1 = DefaultFactory::vbox().size(10).cross_size(4);
            let box2 = DefaultFactory::vbox().size(15).cross_size(6);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Vertical, &mut children, 15, 4, 4, true, true);

            assert_eq!(2, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(4, output[0].size().width().0);
            assert_eq!(10, output[0].size().height().0);

            assert_eq!(8, output[1].offset().0);
            assert_eq!(6, output[1].size().width().0);
            assert_eq!(15, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(8, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(6, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);
        }

        #[test]
        fn three_boxes_wrap_exact_1() {
            let box1 = DefaultFactory::vbox().size(10).cross_size(4);
            let box2 = DefaultFactory::vbox().size(15).cross_size(6);
            let box3 = DefaultFactory::vbox().size(10).cross_size(8);

            let mut children: Vec<Box<dyn Layout>> =
                vec![Box::new(box1), Box::new(box2), Box::new(box3)];

            let output = lay_out_native(Axis::Vertical, &mut children, 24, 0, 0, true, true);

            assert_eq!(3, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(4, output[0].size().width().0);
            assert_eq!(10, output[0].size().height().0);

            assert_eq!(4, output[1].offset().0);
            assert_eq!(6, output[1].size().width().0);
            assert_eq!(15, output[1].size().height().0);

            assert_eq!(10, output[2].offset().0);
            assert_eq!(8, output[2].size().width().0);
            assert_eq!(10, output[2].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(4, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(6, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);

            let item = &output[2].content()[0];
            assert_eq!(10, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(8, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);
        }

        #[test]
        fn three_boxes_wrap_exact_2() {
            let box1 = DefaultFactory::vbox().size(10).cross_size(4);
            let box2 = DefaultFactory::vbox().size(15).cross_size(6);
            let box3 = DefaultFactory::vbox().size(10).cross_size(8);

            let mut children: Vec<Box<dyn Layout>> =
                vec![Box::new(box1), Box::new(box2), Box::new(box3)];

            let output = lay_out_native(Axis::Vertical, &mut children, 25, 0, 0, true, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(6, output[0].size().width().0);
            assert_eq!(25, output[0].size().height().0);

            assert_eq!(6, output[1].offset().0);
            assert_eq!(8, output[1].size().width().0);
            assert_eq!(10, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(10, item.offset_ref().y.0);
            assert_eq!(6, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(6, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(8, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);
        }

        #[test]
        fn three_boxes_wrap_exact_3() {
            let box1 = DefaultFactory::vbox().size(10).cross_size(4);
            let box2 = DefaultFactory::vbox().size(15).cross_size(6);
            let box3 = DefaultFactory::vbox().size(10).cross_size(8);

            let mut children: Vec<Box<dyn Layout>> =
                vec![Box::new(box1), Box::new(box2), Box::new(box3)];

            let output = lay_out_native(Axis::Vertical, &mut children, 26, 0, 0, true, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(6, output[0].size().width().0);
            assert_eq!(25, output[0].size().height().0);

            assert_eq!(6, output[1].offset().0);
            assert_eq!(8, output[1].size().width().0);
            assert_eq!(10, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(10, item.offset_ref().y.0);
            assert_eq!(6, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(6, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(8, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);
        }

        #[test]
        fn three_boxes_wrap_gap_exact_1() {
            let box1 = DefaultFactory::vbox().size(10).cross_size(4);
            let box2 = DefaultFactory::vbox().size(15).cross_size(6);
            let box3 = DefaultFactory::vbox().size(10).cross_size(8);

            let mut children: Vec<Box<dyn Layout>> =
                vec![Box::new(box1), Box::new(box2), Box::new(box3)];

            let output = lay_out_native(Axis::Vertical, &mut children, 27, 3, 3, true, true);

            assert_eq!(3, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(4, output[0].size().width().0);
            assert_eq!(10, output[0].size().height().0);

            assert_eq!(7, output[1].offset().0);
            assert_eq!(6, output[1].size().width().0);
            assert_eq!(15, output[1].size().height().0);

            assert_eq!(16, output[2].offset().0);
            assert_eq!(8, output[2].size().width().0);
            assert_eq!(10, output[2].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(7, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(6, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);

            let item = &output[2].content()[0];
            assert_eq!(16, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(8, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);
        }

        #[test]
        fn three_boxes_wrap_gap_exact_2() {
            let box1 = DefaultFactory::vbox().size(10).cross_size(4);
            let box2 = DefaultFactory::vbox().size(15).cross_size(6);
            let box3 = DefaultFactory::vbox().size(10).cross_size(8);

            let mut children: Vec<Box<dyn Layout>> =
                vec![Box::new(box1), Box::new(box2), Box::new(box3)];

            let output = lay_out_native(Axis::Vertical, &mut children, 28, 3, 3, true, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(6, output[0].size().width().0);
            assert_eq!(28, output[0].size().height().0);

            assert_eq!(9, output[1].offset().0);
            assert_eq!(8, output[1].size().width().0);
            assert_eq!(10, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(13, item.offset_ref().y.0);
            assert_eq!(6, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(9, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(8, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);
        }

        #[test]
        fn three_boxes_wrap_gap_exact_3() {
            let box1 = DefaultFactory::vbox().size(10).cross_size(4);
            let box2 = DefaultFactory::vbox().size(15).cross_size(6);
            let box3 = DefaultFactory::vbox().size(10).cross_size(8);

            let mut children: Vec<Box<dyn Layout>> =
                vec![Box::new(box1), Box::new(box2), Box::new(box3)];

            let output = lay_out_native(Axis::Vertical, &mut children, 29, 3, 3, true, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(6, output[0].size().width().0);
            assert_eq!(28, output[0].size().height().0);

            assert_eq!(9, output[1].offset().0);
            assert_eq!(8, output[1].size().width().0);
            assert_eq!(10, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(0, item.offset_ref().x.0);
            assert_eq!(13, item.offset_ref().y.0);
            assert_eq!(6, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(9, item.offset_ref().x.0);
            assert_eq!(0, item.offset_ref().y.0);
            assert_eq!(8, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);
        }
    }
}
