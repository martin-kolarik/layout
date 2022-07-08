use crate::{unit::Unit, Axis, Layout};

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

    pub fn content_mut(&'a mut self) -> &mut [&mut Box<dyn Layout>] {
        &mut self.content
    }
}

pub fn lay_out_native(
    axis: Axis,
    children: &mut [Box<dyn Layout>],
    wrap_size: impl Into<Unit>,
    gap: impl Into<Unit>,
    respect_baseline: bool,
) -> Vec<Line> {
    if children.is_empty() {
        return vec![];
    }

    let wrap_size = wrap_size.into();
    let gap = gap.into();
    let mut lines = vec![];
    let mut remaining = children.len();

    let mut offset = Offset::zero();
    let mut line_size = Size::zero();
    let mut line = Vec::with_capacity(remaining - 1);

    for child in children {
        let line_gap = if line.is_empty() { Unit::zero() } else { gap };

        let child_size = child.native_size().unwrap_or_else(|| child.size_ref());
        let child_axis_size = axis.size(child_size);

        if axis.size(&line_size) + line_gap + child_axis_size > wrap_size {
            let next_line_offset = axis
                .cross()
                .advance_dim(&offset, axis.cross().size(&line_size) + gap);

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
        line_size = axis.extend_size(&line_size, child_size, respect_baseline);

        *child.offset_mut() = offset.clone();
        offset = axis.advance_dim(&offset, child_axis_size);

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
        use crate::{children::lay_out_native, Axis, DefaultFactory, Factory, Layout};

        #[test]
        fn single_box() {
            let box1 = DefaultFactory::hbox().size(10);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 100, 0, true);

            assert_eq!(1, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(10, output[0].size().width().0);
            assert_eq!(0, output[0].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(0, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes() {
            let box1 = DefaultFactory::hbox().size(10);
            let box2 = DefaultFactory::hbox().size(15);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 100, 0, true);

            assert_eq!(1, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(25, output[0].size().width().0);
            assert_eq!(0, output[0].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(0, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(10, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(0, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes_gap() {
            let box1 = DefaultFactory::hbox().size(10);
            let box2 = DefaultFactory::hbox().size(15);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 100, 3, true);

            assert_eq!(1, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(28, output[0].size().width().0);
            assert_eq!(0, output[0].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(0, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(13, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(0, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes_wrap() {
            let box1 = DefaultFactory::hbox().size(10).cross_size(4);
            let box2 = DefaultFactory::hbox().size(15).cross_size(6);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 15, 0, true);

            assert_eq!(2, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(10, output[0].size().width().0);
            assert_eq!(4, output[0].size().height().0);

            assert_eq!(4, output[1].offset().0);
            assert_eq!(15, output[1].size().width().0);
            assert_eq!(6, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(4, item.offset_ref().y().0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(6, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes_wrap_gap() {
            let box1 = DefaultFactory::hbox().size(10).cross_size(4);
            let box2 = DefaultFactory::hbox().size(15).cross_size(6);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Horizontal, &mut children, 15, 4, true);

            assert_eq!(2, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(10, output[0].size().width().0);
            assert_eq!(4, output[0].size().height().0);

            assert_eq!(8, output[1].offset().0);
            assert_eq!(15, output[1].size().width().0);
            assert_eq!(6, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(8, item.offset_ref().y().0);
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

            let output = lay_out_native(Axis::Horizontal, &mut children, 24, 0, true);

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
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(4, item.offset_ref().y().0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(6, item.size_ref().height().0);

            let item = &output[2].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(10, item.offset_ref().y().0);
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

            let output = lay_out_native(Axis::Horizontal, &mut children, 25, 0, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(25, output[0].size().width().0);
            assert_eq!(6, output[0].size().height().0);

            assert_eq!(6, output[1].offset().0);
            assert_eq!(10, output[1].size().width().0);
            assert_eq!(8, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(10, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(6, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(6, item.offset_ref().y().0);
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

            let output = lay_out_native(Axis::Horizontal, &mut children, 26, 0, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(25, output[0].size().width().0);
            assert_eq!(6, output[0].size().height().0);

            assert_eq!(6, output[1].offset().0);
            assert_eq!(10, output[1].size().width().0);
            assert_eq!(8, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(10, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(6, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(6, item.offset_ref().y().0);
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

            let output = lay_out_native(Axis::Horizontal, &mut children, 27, 3, true);

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
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(7, item.offset_ref().y().0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(6, item.size_ref().height().0);

            let item = &output[2].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(16, item.offset_ref().y().0);
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

            let output = lay_out_native(Axis::Horizontal, &mut children, 28, 3, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(28, output[0].size().width().0);
            assert_eq!(6, output[0].size().height().0);

            assert_eq!(9, output[1].offset().0);
            assert_eq!(10, output[1].size().width().0);
            assert_eq!(8, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(13, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(6, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(9, item.offset_ref().y().0);
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

            let output = lay_out_native(Axis::Horizontal, &mut children, 29, 3, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(28, output[0].size().width().0);
            assert_eq!(6, output[0].size().height().0);

            assert_eq!(9, output[1].offset().0);
            assert_eq!(10, output[1].size().width().0);
            assert_eq!(8, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(4, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(13, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(15, item.size_ref().width().0);
            assert_eq!(6, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(9, item.offset_ref().y().0);
            assert_eq!(10, item.size_ref().width().0);
            assert_eq!(8, item.size_ref().height().0);
        }
    }

    mod vbox {
        use crate::{children::lay_out_native, Axis, DefaultFactory, Factory, Layout};

        #[test]
        fn single_box() {
            let box1 = DefaultFactory::vbox().size(10);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1)];

            let output = lay_out_native(Axis::Vertical, &mut children, 100, 0, true);

            assert_eq!(1, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(0, output[0].size().width().0);
            assert_eq!(10, output[0].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(0, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes() {
            let box1 = DefaultFactory::vbox().size(10);
            let box2 = DefaultFactory::vbox().size(15);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Vertical, &mut children, 100, 0, true);

            assert_eq!(1, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(0, output[0].size().width().0);
            assert_eq!(25, output[0].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(0, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(10, item.offset_ref().y().0);
            assert_eq!(0, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes_gap() {
            let box1 = DefaultFactory::vbox().size(10);
            let box2 = DefaultFactory::vbox().size(15);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Vertical, &mut children, 100, 3, true);

            assert_eq!(1, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(0, output[0].size().width().0);
            assert_eq!(28, output[0].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(0, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(13, item.offset_ref().y().0);
            assert_eq!(0, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes_wrap() {
            let box1 = DefaultFactory::vbox().size(10).cross_size(4);
            let box2 = DefaultFactory::vbox().size(15).cross_size(6);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Vertical, &mut children, 15, 0, true);

            assert_eq!(2, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(4, output[0].size().width().0);
            assert_eq!(10, output[0].size().height().0);

            assert_eq!(4, output[1].offset().0);
            assert_eq!(6, output[1].size().width().0);
            assert_eq!(15, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(4, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(6, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);
        }

        #[test]
        fn two_boxes_wrap_gap() {
            let box1 = DefaultFactory::vbox().size(10).cross_size(4);
            let box2 = DefaultFactory::vbox().size(15).cross_size(6);

            let mut children: Vec<Box<dyn Layout>> = vec![Box::new(box1), Box::new(box2)];

            let output = lay_out_native(Axis::Vertical, &mut children, 15, 4, true);

            assert_eq!(2, output.len());
            assert_eq!(1, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(4, output[0].size().width().0);
            assert_eq!(10, output[0].size().height().0);

            assert_eq!(8, output[1].offset().0);
            assert_eq!(6, output[1].size().width().0);
            assert_eq!(15, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(8, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
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

            let output = lay_out_native(Axis::Vertical, &mut children, 24, 0, true);

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
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(4, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(6, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);

            let item = &output[2].content()[0];
            assert_eq!(10, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
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

            let output = lay_out_native(Axis::Vertical, &mut children, 25, 0, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(6, output[0].size().width().0);
            assert_eq!(25, output[0].size().height().0);

            assert_eq!(6, output[1].offset().0);
            assert_eq!(8, output[1].size().width().0);
            assert_eq!(10, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(10, item.offset_ref().y().0);
            assert_eq!(6, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(6, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
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

            let output = lay_out_native(Axis::Vertical, &mut children, 26, 0, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(6, output[0].size().width().0);
            assert_eq!(25, output[0].size().height().0);

            assert_eq!(6, output[1].offset().0);
            assert_eq!(8, output[1].size().width().0);
            assert_eq!(10, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(10, item.offset_ref().y().0);
            assert_eq!(6, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(6, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
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

            let output = lay_out_native(Axis::Vertical, &mut children, 27, 3, true);

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
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(7, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(6, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);

            let item = &output[2].content()[0];
            assert_eq!(16, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
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

            let output = lay_out_native(Axis::Vertical, &mut children, 28, 3, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(6, output[0].size().width().0);
            assert_eq!(28, output[0].size().height().0);

            assert_eq!(9, output[1].offset().0);
            assert_eq!(8, output[1].size().width().0);
            assert_eq!(10, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(13, item.offset_ref().y().0);
            assert_eq!(6, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(9, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
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

            let output = lay_out_native(Axis::Vertical, &mut children, 29, 3, true);

            assert_eq!(2, output.len());
            assert_eq!(2, output[0].len());

            assert_eq!(0, output[0].offset().0);
            assert_eq!(6, output[0].size().width().0);
            assert_eq!(28, output[0].size().height().0);

            assert_eq!(9, output[1].offset().0);
            assert_eq!(8, output[1].size().width().0);
            assert_eq!(10, output[1].size().height().0);

            let item = &output[0].content()[0];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(4, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);

            let item = &output[0].content()[1];
            assert_eq!(0, item.offset_ref().x().0);
            assert_eq!(13, item.offset_ref().y().0);
            assert_eq!(6, item.size_ref().width().0);
            assert_eq!(15, item.size_ref().height().0);

            let item = &output[1].content()[0];
            assert_eq!(9, item.offset_ref().x().0);
            assert_eq!(0, item.offset_ref().y().0);
            assert_eq!(8, item.size_ref().width().0);
            assert_eq!(10, item.size_ref().height().0);
        }
    }
}
