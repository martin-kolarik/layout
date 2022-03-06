use rtext::Apply;

use crate::{
    children::lay_out_native,
    dimension::{DimAutoOrParent, DimOrParent},
    position::{Offset, Size},
    styled,
    unit::{sub_unit, Fill, Unit},
    AlignItems, Axis, Error, Layout, MeasureContext, Position, RenderContext, Style, Styled,
};

pub struct LayoutBox {
    mark: Option<&'static str>,
    axis: Axis,
    offset: Offset,
    size: Size,
    style: Style,
    children: Vec<Box<dyn Layout>>,
    native_size: Option<Size>,
    content_size: Option<Size>,
}

impl LayoutBox {
    pub fn new(axis: Axis) -> Self {
        Self {
            mark: None,
            axis,
            offset: Offset::zero(),
            size: Size::new_auto(),
            style: Style::new().with_align_items(if matches!(axis, Axis::Horizontal) {
                AlignItems::Baseline
            } else {
                AlignItems::Start
            }),
            children: vec![],
            native_size: None,
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

    pub fn min(mut self, size: impl Into<DimOrParent>) -> Self {
        self.axis.dim_mut(&mut self.size).set_min(size);
        self
    }

    pub fn max(mut self, size: impl Into<DimOrParent>) -> Self {
        self.axis.dim_mut(&mut self.size).set_max(size);
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

    pub fn depth(mut self, depth: impl Into<Unit>) -> Self {
        self.style = self.style.with_align_items(AlignItems::Baseline);
        self.size.set_depth(Some(depth));
        self
    }

    pub fn cross_size(mut self, size: impl Into<DimAutoOrParent>) -> Self {
        self.axis.cross().dim_mut(&mut self.size).set_basis(size);
        self
    }

    pub fn cross_min(mut self, size: impl Into<DimOrParent>) -> Self {
        self.axis.cross().dim_mut(&mut self.size).set_min(size);
        self
    }

    pub fn cross_max(mut self, size: impl Into<DimOrParent>) -> Self {
        self.axis.cross().dim_mut(&mut self.size).set_max(size);
        self
    }

    pub fn cross_grow(mut self, weight: impl Into<Fill>) -> Self {
        self.axis.cross().dim_mut(&mut self.size).set_grow(weight);
        self
    }

    pub fn cross_shrink(mut self, weight: impl Into<Fill>) -> Self {
        self.axis.cross().dim_mut(&mut self.size).set_shrink(weight);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn take_over_position(&mut self, offset: Offset, size: Size) {
        self.offset = offset;
        self.size = size;
    }

    pub fn axis(&self) -> Axis {
        self.axis
    }

    pub fn child(self, child: impl Layout + 'static) -> Self {
        self.child_box(Box::new(child))
    }

    pub fn children<L, IL, IIL>(mut self, children: IIL) -> Self
    where
        IIL: IntoIterator<Item = IL>,
        IL: Into<L>,
        L: Layout + 'static,
    {
        let children = children
            .into_iter()
            .map(|child| {
                let mut child: Box<dyn Layout> = Box::new(child.into());
                child.adopt_parent_style(self.style_ref());
                child
            })
            .collect::<Vec<_>>();
        self.children.extend(children);
        self
    }

    pub fn child_box(mut self, mut child: Box<dyn Layout>) -> Self {
        child.adopt_parent_style(self.style_ref());
        self.children.push(child);
        self
    }

    pub fn children_box<IL>(mut self, children: IL) -> Self
    where
        IL: IntoIterator<Item = Box<dyn Layout>>,
    {
        let children = children
            .into_iter()
            .map(|mut child| {
                child.adopt_parent_style(self.style_ref());
                child
            })
            .collect::<Vec<_>>();
        self.children.extend(children);
        self
    }
}

pub struct ChildrenIterator<'a> {
    index: usize,
    of: &'a LayoutBox,
}

impl<'a> ChildrenIterator<'a> {
    pub fn new(of: &'a LayoutBox) -> Self {
        Self { index: 0, of }
    }
}

impl<'a> Iterator for ChildrenIterator<'a> {
    type Item = &'a Box<dyn Layout>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.of.children.len() {
            None
        } else {
            let item = &self.of.children[self.index];
            self.index += 1;
            Some(item)
        }
    }
}

impl Apply for LayoutBox {}

impl Position for LayoutBox {
    fn mark(&self) -> &'static str {
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

    fn native_size(&self) -> Option<&Size> {
        self.native_size.as_ref().or(Some(&self.size))
    }

    fn content_size(&self) -> Option<&Size> {
        self.content_size.as_ref()
    }
}

styled!(LayoutBox);

impl Layout for LayoutBox {
    fn measure(&mut self, ctx: &mut dyn MeasureContext, room: Size) -> Result<(), Error> {
        if self.children.is_empty() {
            return Ok(());
        }

        let axis = self.axis;
        let given_axis_room = axis.size(&room);
        let axis_room = axis.dim(&self.size).size_available(given_axis_room);
        self.children
            .iter_mut()
            .for_each(|child| axis.dim_mut(child.size_mut()).resolve_parented(axis_room));

        let cross = axis.cross();
        let given_cross_room = cross.size(&room);
        let cross_room = cross.dim(&self.size).size_available(given_cross_room);
        self.children
            .iter_mut()
            .for_each(|child| cross.dim_mut(child.size_mut()).resolve_parented(cross_room));

        for child in self.children.iter_mut() {
            child.measure(ctx, room.clone())?;
        }

        let gap = self.style_ref().gap_size();
        let lines = lay_out_native(self.axis, &mut self.children, axis_room, gap);
        let last = lines.last().unwrap();
        self.native_size = Some(axis.extend_dim(last.size(), last.offset()));

        Ok(())
    }

    fn lay_out(
        &mut self,
        ctx: &mut dyn MeasureContext,
        mut self_position: Offset,
        size: Size,
    ) -> Result<(), Error> {
        // axes preparation
        let axis = self.axis;
        let cross = axis.cross();
        let cross_takes_native = cross.dim(&self.size).is_content_fixed();

        // dimensions preparation
        let given_axis_size = axis.size(&size);
        let wrap_size = axis.dim(&self.size).size_available(given_axis_size);

        let given_cross_size = cross.size(&size);
        let cross_room = cross.dim(&self.size).size_available(given_cross_size);

        let align_items = self.style_ref().align_items();
        let gap = self.style_ref().gap_size();

        // Resolve relative positioning of request and self ascents, when aligning to baseline.
        let given_ascent = sub_unit(size.ascent(), self.size_ref().ascent());
        if matches!(
            (align_items, size.depth(), self.size_ref().depth()),
            (AlignItems::Baseline, Some(_), Some(_))
        ) {
            self_position.y_advance(given_ascent.unwrap_or_default());
        }

        // wrap children using native size
        let lines = lay_out_native(self.axis, &mut self.children, wrap_size, gap);

        // prepare loop over lines
        let mut position = self_position.clone();
        let mut content_size = Size::zero();
        let mut ascent_inherit_cache = None;
        let mut ascent;
        let mut first_line = Some(());
        let multi_line = lines.len() > 1;

        for mut line in lines {
            let native_line_size = line.size().clone();
            ascent = native_line_size.ascent();

            if first_line.take().is_some() {
                if matches!((axis, given_ascent), (Axis::Horizontal, Some(_))) {
                    // if lines are horizontal, only the first line respects baseline
                    ascent = given_ascent;
                }
                ascent_inherit_cache = ascent;
            } else {
                position = cross.advance_dim(&position, gap);
                content_size = cross.extend_dim(&content_size, gap);
            }

            let room_to_distribute = wrap_size - axis.size(&native_line_size);
            let sum_grow = axis.dim(&native_line_size).grow();
            let sum_shrink = axis.dim(&native_line_size).shrink();

            // If lines are more, or if cross axis has no dimension, use native size.
            // Otherwise (single line with cross axis known size) cross axis may stretch.
            let line_cross_room = if multi_line || cross_takes_native {
                cross.size(&native_line_size)
            } else {
                cross_room
            };

            // prepare loop over children in line
            axis.set_offset(&mut position, axis.offset(&self_position)); // reset axis offset for new line
            let mut line_size = Size::zero();
            let mut first_child = Some(());

            for child in line.content_mut() {
                let first = first_child.take();
                if first.is_some() {
                    if matches!((axis, given_ascent), (Axis::Vertical, Some(_))) {
                        // if lines are vertical, each first child of each line respects baseline
                        ascent = given_ascent;
                    }
                    if ascent_inherit_cache.is_none() {
                        ascent_inherit_cache = ascent;
                    }
                } else {
                    position = axis.advance_dim(&position, gap);
                    line_size = axis.extend_dim(&line_size, gap);
                }

                // Resolve axis streches.
                let child_axis_size = axis.dim(child.native_size().unwrap()).size_distributed(
                    room_to_distribute,
                    sum_grow,
                    sum_shrink,
                );

                // Resolve cross stretches. only if both me and child has auto dimension, they stretch.
                // The behavior is the same as in FlexBox.
                let line_cross_grows = cross.dim(&self.size).is_dyn();
                let child_cross_grows = cross.dim(child.size_ref()).is_content_or_dyn();
                let child_cross_size = if child_cross_grows && line_cross_grows {
                    cross.dim(child.size_ref()).size_available(line_cross_room)
                } else {
                    cross
                        .dim(child.size_ref())
                        .size_filled(cross.size(&native_line_size))
                };

                // Baseline alignment for vertical axis (baseline offsets main axis).
                let child_axis_offset =
                    match (align_items, first, axis, ascent, child.size_ref().ascent()) {
                        (
                            AlignItems::Baseline,
                            Some(_),
                            Axis::Vertical,
                            Some(ascent),
                            Some(child_ascent),
                        ) => ascent - child_ascent,
                        _ => Unit::zero(),
                    };
                position = axis.advance_dim(&position, child_axis_offset);

                // Cross axis alignment. Baseline alignment for horizontal axis (baseline offsets cross axis).
                let child_cross_offset = match (&self.style.align_items(), axis) {
                    (AlignItems::Start, _) => Unit::zero(),
                    (AlignItems::Center, _) => (line_cross_room - child_cross_size) * 0.5,
                    (AlignItems::End, _) => line_cross_room - child_cross_size,
                    (AlignItems::Baseline, Axis::Horizontal) => {
                        match (ascent, child.size_ref().ascent()) {
                            (Some(ascent), Some(child_ascent)) => ascent - child_ascent,
                            // the following creates artificial baseline of child box in its lower edge, if the child box has no baseline
                            // (Some(ascent), None) => ascent - child_cross_size,
                            _ => Unit::zero(),
                        }
                    }
                    (AlignItems::Baseline, Axis::Vertical) => Unit::zero(),
                };

                let cross_offsetted_position = cross.advance_dim(&position, child_cross_offset);

                // Construct child size.
                let (width, height) = match axis {
                    Axis::Horizontal => (child_axis_size, child_cross_size),
                    Axis::Vertical => (child_cross_size, child_axis_size),
                };
                let child_depth = child.size_ref().depth();
                let child_size = match child_depth {
                    Some(depth) => Size::new_depth(width, height, depth),
                    None => Size::new(width, height),
                };

                // recurse into
                child.lay_out(ctx, cross_offsetted_position, child_size)?;

                // line_child_size incorporates bounding box of child offsetted in both axes.
                // line_child_size can be bigger than child_size.
                let line_child_size = child.size_ref();
                let line_child_size = axis.extend_dim(line_child_size, child_axis_offset);
                let line_child_size = cross.extend_dim(&line_child_size, child_cross_offset);

                // move forward in main axis, gap is added at the loop begin
                position = axis.advance_dim(&position, child_axis_size);
                line_size = axis.extend_size(&line_size, &line_child_size);
            }

            // Move forward in cross axis (over lines), gap is added at the loop begin.
            // Multiple lines never stretch (the same behavior as FlexBox has).
            position = cross.advance_dim(&position, cross.size(&native_line_size));
            content_size = cross.extend_size(&content_size, &line_size);
        }

        // Sets own axis dimension to native content size, if the dimension is auto_fixed.
        // Otherwise own axis dimension can be stretchable, therefore size_filled is called to resolve it.
        axis.resolve_content_size(&mut self.size, &content_size, given_axis_size);

        // Sets own cross dimension to native content size, if the dimension is auto_fixed.
        // Otherwise own cross dimension can be stretchable, therefore size_filled is called to resolve it.
        cross.resolve_content_size(&mut self.size, &content_size, given_cross_size);

        // Adopt children depth, if I have not it set.
        if self.size.depth().is_none() {
            self.size.set_depth(
                ascent_inherit_cache.map(|first_ascent| self.size.height() - first_ascent),
            );
        }

        self.offset = self_position.clone();
        self.content_size = Some(content_size);

        Ok(())
    }

    fn render(&self, ctx: &mut dyn RenderContext) -> Result<(), Error> {
        for child in self.iter() {
            child.render(ctx)?;
        }

        ctx.debug_frame(self.offset_ref(), self.size_ref());

        Ok(())
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &Box<dyn Layout>> + '_> {
        Box::new(ChildrenIterator::new(self))
    }
}
