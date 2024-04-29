use std::sync::Arc;

use crate::{
    children::lay_out_native,
    dimension::{Dim, MaybeDim},
    position::{Offset, Size},
    unit::{sub_unit, Fill, Unit},
    AlignItems, Axis, DefaultFactory, Error, Layout, MeasureContext, NewPageOptions, Position,
    RenderContext, Style, StyleBuilder, Styled,
};

pub struct LayoutBox {
    mark: Option<&'static str>,
    axis: Axis,
    offset: Offset,
    size: Size,
    break_inside: bool,
    style: Arc<Style>,
    children: Vec<Box<dyn Layout>>,
    content_size: Option<Size>,
}

impl LayoutBox {
    pub fn new(axis: Axis) -> Self {
        Self {
            mark: None,
            axis,
            offset: Offset::zero(),
            size: Size::none(),
            break_inside: true,
            style: StyleBuilder::new().build(),
            children: vec![],
            content_size: None,
        }
    }

    pub fn mark(mut self, mark: &'static str) -> Self {
        self.mark = Some(mark);
        self
    }

    pub fn size(mut self, size: impl Into<Dim>) -> Self {
        self.axis.dim_mut(&mut self.size).set_basis(size);
        self
    }

    pub fn avoid_break(mut self) -> Self {
        self.break_inside = false;
        self
    }

    pub fn min(mut self, size: impl Into<MaybeDim>) -> Self {
        self.axis.dim_mut(&mut self.size).set_min(size);
        self
    }

    pub fn max(mut self, size: impl Into<MaybeDim>) -> Self {
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
        if !matches!(self.style.align_items(), AlignItems::Baseline) {
            tracing::warn!("Depth set for a box having items not aligned on a baseline");
        }
        self.size.set_depth(Some(depth));
        self
    }

    pub fn cross_size(mut self, size: impl Into<Dim>) -> Self {
        self.axis.cross().dim_mut(&mut self.size).set_basis(size);
        self
    }

    pub fn cross_min(mut self, size: impl Into<MaybeDim>) -> Self {
        self.axis.cross().dim_mut(&mut self.size).set_min(size);
        self
    }

    pub fn cross_max(mut self, size: impl Into<MaybeDim>) -> Self {
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

    pub fn style(mut self, style: impl Into<Arc<Style>>) -> Self {
        self.set_style(style.into());
        self
    }

    pub fn add_style(mut self, style: impl Into<Arc<Style>>) -> Self {
        self.set_style(style.into().merge(&self.style));
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
        self.child_dyn(Box::new(child))
    }

    pub fn children<L, IL, IIL>(self, children: IIL) -> Self
    where
        IIL: IntoIterator<Item = IL>,
        IL: Into<L>,
        L: Layout + 'static,
    {
        self.children_dyn(children.into_iter().map(|child| {
            let child: Box<dyn Layout> = Box::new(child.into());
            child
        }))
    }

    pub fn child_dyn(mut self, mut child: Box<dyn Layout>) -> Self {
        child.set_style(child.style_ref().inherit(self.style_ref()));
        self.children.push(child);
        self
    }

    pub fn children_dyn<IL>(mut self, children: IL) -> Self
    where
        IL: IntoIterator<Item = Box<dyn Layout>>,
    {
        let children = children
            .into_iter()
            .map(|mut child| {
                child.set_style(child.style_ref().inherit(self.style_ref()));
                child
            })
            .collect::<Vec<_>>();
        self.children.extend(children);
        self
    }

    pub fn text(self, text: impl ToString) -> Self {
        self.child_dyn(Box::new(DefaultFactory::text(text)))
    }
}

impl Position for LayoutBox {
    fn element(&self) -> &str {
        "LayoutBox"
    }

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

    fn content_size(&self) -> Option<&Size> {
        self.content_size.as_ref()
    }
}

impl Styled for LayoutBox {
    fn style_ref(&self) -> &Style {
        &self.style
    }

    fn set_style(&mut self, style: Arc<Style>) {
        self.children.iter_mut().for_each(|child| {
            child.set_style(child.style_ref().inherit(&style));
        });
        self.size.apply_style(self.axis, &style);
        self.style = style;
    }
}

impl Layout for LayoutBox {
    fn measure(&mut self, ctx: &mut dyn MeasureContext, mut room: Size) -> Result<(), Error> {
        let axis = self.axis;
        let axis_room = axis.size(&room);
        axis.dim_mut(self.size_mut()).resolve_parented(axis_room);

        let cross = axis.cross();
        let cross_room = cross.size(&room);
        cross.dim_mut(self.size_mut()).resolve_parented(cross_room);

        let respect_baseline = matches!(self.style_ref().align_items(), AlignItems::Baseline);
        let mut self_size = self.size.clone();

        self.style_ref()
            .padding()
            .narrow(None, Some(&mut self_size));

        let mut self_size = if self.children.is_empty() {
            self_size
        } else {
            // TODO: should not be limiting room fo children here??
            // self.style_ref().padding().narrow(None, Some(&mut room));
            for child in self.children.iter_mut() {
                child.measure(ctx, room.clone())?;
            }

            self.style_ref().padding().narrow(None, Some(&mut room));
            let axis_room = axis.size(&room);
            let axis_room = axis.dim(&self_size).size_available(axis_room);
            let wrap = self
                .style_ref()
                .wrap()
                .unwrap_or(matches!(axis, Axis::Horizontal));

            let axis_gap = axis.select(
                self.style_ref().horizontal_gap_size(),
                self.style_ref().vertical_gap_size(),
            );
            let cross_gap = cross.select(
                self.style_ref().horizontal_gap_size(),
                self.style_ref().vertical_gap_size(),
            );

            let lines = lay_out_native(
                self.axis,
                &mut self.children,
                axis_room,
                axis_gap,
                cross_gap,
                wrap,
                respect_baseline,
            );

            let mut children_size =
                lines
                    .iter()
                    .enumerate()
                    .fold(Size::none(), |mut sum, (index, line)| {
                        *axis.dim_mut(&mut sum) = axis.dim(&sum).max_of(axis.dim(line.size()));
                        let with_gap = if index == 0 {
                            Size::none()
                        } else {
                            cross.extend_dim(&sum, cross_gap)
                        };
                        cross.extend_size(
                            &with_gap,
                            line.size(),
                            respect_baseline && (index == 0 || matches!(axis, Axis::Horizontal)),
                        )
                    });

            if axis.dim(&self_size).is_fixed() {
                *axis.dim_mut(&mut children_size) = axis.dim(&self_size).clone();
            }
            if cross.dim(&self_size).is_fixed() {
                *cross.dim_mut(&mut children_size) = cross.dim(&self_size).clone();
            }

            children_size
        };

        self.style_ref().padding().widen(None, Some(&mut self_size));

        axis.dim_mut(self.size_mut())
            .resolve_content(axis.size(&self_size));
        cross
            .dim_mut(self.size_mut())
            .resolve_content(cross.size(&self_size));
        if respect_baseline && self.size_ref().depth().is_none() {
            self.size_mut().set_depth(self_size.depth());
        }

        Ok(())
    }

    fn lay_out(
        &mut self,
        ctx: &mut dyn MeasureContext,
        mut offset: Offset,
        mut room: Size,
    ) -> Result<(), Error> {
        // resolve padding
        self.style_ref()
            .padding()
            .narrow(Some(&mut offset), Some(&mut room));

        let mut size = self.size.clone();
        self.style_ref().padding().narrow(None, Some(&mut size));

        // axes preparation
        let axis = self.axis;
        let cross = axis.cross();
        let cross_takes_native = cross.dim(&size).is_content_fixed();

        // dimensions preparation
        let axis_room = axis.size(&room);
        let axis_size = axis.dim(&size).size_available(axis_room);

        let cross_room = cross.size(&room);
        let cross_size = cross.dim(&size).size_available(cross_room);

        let wrap = self
            .style_ref()
            .wrap()
            .unwrap_or(matches!(axis, Axis::Horizontal));
        let align_items = self.style_ref().align_items();

        let axis_gap = axis.select(
            self.style_ref().horizontal_gap_size(),
            self.style_ref().vertical_gap_size(),
        );
        let cross_gap = cross.select(
            self.style_ref().horizontal_gap_size(),
            self.style_ref().vertical_gap_size(),
        );

        // Resolve relative positioning of request and self ascents, when aligning to baseline.
        let self_ascent = self.size_after_wrap_ref().ascent();
        if matches!(
            (align_items, room.depth(), self_ascent),
            (AlignItems::Baseline, Some(_), Some(_))
        ) {
            let self_to_parent_ascent = sub_unit(room.ascent(), self_ascent);
            offset.y_advance(self_to_parent_ascent.unwrap_or_default());
        }

        // wrap children using native size
        let lines = lay_out_native(
            self.axis,
            &mut self.children,
            axis_size,
            axis_gap,
            cross_gap,
            wrap,
            matches!(align_items, AlignItems::Baseline),
        );

        // prepare loop over lines
        let mut position = offset.clone();
        let mut content_size = Size::zero();
        let mut first_ascent = None;
        let mut first_line = Some(());
        let multi_line = lines.len() > 1;

        for mut line in lines {
            let native_line_size = line.size().clone();

            if first_line.take().is_some() {
                if matches!(axis, Axis::Horizontal) {
                    // if lines are horizontal, only the first line respects baseline
                    if matches!(
                        (align_items, self_ascent, native_line_size.ascent()),
                        (AlignItems::Baseline, Some(_), Some(_))
                    ) {
                        let first_line_to_self_ascent =
                            sub_unit(self_ascent, native_line_size.ascent());
                        offset.y_advance(first_line_to_self_ascent.unwrap_or_default());
                    }

                    first_ascent = self_ascent.or_else(|| native_line_size.ascent());
                }
            } else {
                position = cross.advance_dim(&position, cross_gap);
                content_size = cross.extend_dim(&content_size, cross_gap);
            }

            let room_to_distribute = axis_size - axis.size(&native_line_size);
            let sum_grow = axis.dim(&native_line_size).grow();
            let sum_shrink = axis.dim(&native_line_size).shrink();

            // If lines are more, or if cross axis has no dimension, use native size.
            // Otherwise (single line with cross axis known size) cross axis may stretch.
            let line_cross_room = if multi_line || cross_takes_native {
                cross.size(&native_line_size)
            } else {
                cross_size
            };

            // prepare loop over children in line
            axis.set_offset(&mut position, axis.offset(&offset)); // reset axis offset for new line
            let mut line_size = Size::zero();
            let mut first_child = Some(());

            for child in line.content_mut() {
                let first = first_child.take();
                if first.is_some() {
                    if matches!(axis, Axis::Vertical) {
                        first_ascent = first_ascent.max(child.size_after_wrap_ref().ascent());
                    }
                } else {
                    position = axis.advance_dim(&position, axis_gap);
                    line_size = axis.extend_dim(&line_size, axis_gap);
                }

                let child_size = child.size_after_wrap_ref();

                // Resolve axis streches.
                let child_axis_size =
                    axis.dim(child_size)
                        .size_distributed(room_to_distribute, sum_grow, sum_shrink);

                // Resolve cross stretches. only if both me and child has auto dimension, they stretch.
                // The behavior is the same as in FlexBox.
                let line_cross_grows = cross.dim(&size).is_dyn();
                let child_cross_grows = cross.dim(child_size).is_content_or_dyn();
                let child_cross_size = if child_cross_grows && line_cross_grows {
                    cross.dim(child_size).size_available(line_cross_room)
                } else {
                    cross
                        .dim(child_size)
                        .size_filled(cross.size(&native_line_size))
                };

                // Baseline alignment
                let child_ascent = child_size.ascent();

                // Baseline axis is main axis (therefore vertical).
                let child_axis_offset = match (align_items, first, axis, self_ascent, child_ascent)
                {
                    (
                        AlignItems::Baseline,
                        Some(_),
                        Axis::Vertical,
                        Some(self_ascent),
                        Some(child_ascent),
                    ) => self_ascent - child_ascent,
                    _ => Unit::zero(),
                };
                position = axis.advance_dim(&position, child_axis_offset);

                // Baseline axis is cross axis (therefore horizontal).
                let child_cross_offset = match (&self.style.align_items(), axis) {
                    (AlignItems::Start, _) => Unit::zero(),
                    (AlignItems::Center, _) => (line_cross_room - child_cross_size) * 0.5,
                    (AlignItems::End, _) => line_cross_room - child_cross_size,
                    (AlignItems::Baseline, Axis::Horizontal) => {
                        match (native_line_size.ascent(), child_ascent) {
                            (Some(line_ascent), Some(child_ascent)) => line_ascent - child_ascent,
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
                let child_depth = child.size_after_wrap_ref().depth();
                let child_size = match child_depth {
                    Some(depth) => Size::fixed_depth(width, height, depth),
                    None => Size::fixed(width, height),
                };

                // recurse into
                child.lay_out(ctx, cross_offsetted_position, child_size)?;

                // line_child_size incorporates bounding box of child offsetted in both axes.
                // line_child_size can be bigger than child_size.
                let line_child_size = child.size_after_lay_out();
                let line_child_size = axis.extend_dim(&line_child_size, child_axis_offset);
                let line_child_size = cross.extend_dim(&line_child_size, child_cross_offset);

                // move forward in main axis, gap is added at the loop begin
                position = axis.advance_dim(&position, child_axis_size);
                line_size = axis.extend_size(
                    &line_size,
                    &line_child_size,
                    matches!(align_items, AlignItems::Baseline),
                );
            }

            // Move forward in cross axis (over lines), gap is added at the loop begin.
            // Multiple lines never stretch (the same behavior as FlexBox has).
            position = cross.advance_dim(&position, cross.size(&line_size));
            content_size = cross.extend_size(
                &content_size,
                &line_size,
                matches!(align_items, AlignItems::Baseline),
            );
        }

        // Sets own axis/cross dimensions to native content size, if the dimension is auto_fixed.
        // Otherwise own axis/cross dimension can be stretchable, therefore size_filled is called to resolve it.
        axis.resolve_content_size(&mut size, &content_size, axis_room);
        cross.resolve_content_size(&mut size, &content_size, cross_room);

        self.content_size = Some(content_size);

        // Adopt final offset and size including padding
        self.style_ref()
            .padding()
            .widen(Some(&mut offset), Some(&mut size));

        self.offset = offset;
        self.size = size;

        // Adopt children depth, if I have not it set.
        if self.size.depth().is_none() {
            self.size
                .set_depth(first_ascent.map(|first_ascent| self.size.height() - first_ascent));
        }

        Ok(())
    }

    fn render(&self, ctx: &mut dyn RenderContext) -> Result<(), Error> {
        if !self.break_inside {
            ctx.new_page(Some(
                NewPageOptions::new().with_break_if_not_room(self.offset_ref(), self.size_ref()),
            ));
        }

        for child in self.iter() {
            child.render(ctx)?;
        }

        let style = self.style_ref();
        let top_left = self.offset_ref();
        let bottom_right = top_left + &self.size;

        if let Some(stroke) = style.border_top() {
            ctx.line(
                &self.offset,
                &Offset::new(bottom_right.x, top_left.y),
                stroke,
            );
        }

        if let Some(stroke) = style.border_right() {
            ctx.line(
                &Offset::new(bottom_right.x, top_left.y),
                &bottom_right,
                stroke,
            );
        }

        if let Some(stroke) = style.border_bottom() {
            ctx.line(
                &bottom_right,
                &Offset::new(top_left.x, bottom_right.y),
                stroke,
            );
        }

        if let Some(stroke) = style.border_left() {
            ctx.line(&Offset::new(top_left.x, bottom_right.y), top_left, stroke);
        }

        ctx.debug_frame(self.offset_ref(), self.size_ref());

        Ok(())
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &Box<dyn Layout>> + '_> {
        Box::new(self.children.iter())
    }
}
