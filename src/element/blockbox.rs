use std::sync::Arc;

use crate::{
    AlignItems, Axis, Error, Layout, MeasureContext, Position, RenderContext, Style, Styled,
    dimension::{Dim, MaybeDim},
    position::{Offset, Size},
    unit::{Fill, Unit, sub_unit},
};

pub struct BlockBox {
    mark: Option<&'static str>,
    offset: Offset,
    size: Size,
    style: Arc<Style>,
    children: Vec<Box<dyn Layout>>,
}

impl BlockBox {
    pub fn new() -> Self {
        Self {
            mark: None,
            offset: Offset::zero(),
            size: Size::content(),
            style: Style::new(),
            children: vec![],
        }
    }
    pub fn mark(mut self, mark: &'static str) -> Self {
        self.mark = Some(mark);
        self
    }

    pub fn axis_size(mut self, size: impl Into<Dim>) -> Self {
        Axis::Horizontal.dim_mut(&mut self.size).set_base(size);
        self
    }

    pub fn axis_min(mut self, size: impl Into<MaybeDim>) -> Self {
        Axis::Horizontal.dim_mut(&mut self.size).set_min(size);
        self
    }

    pub fn axis_max(mut self, size: impl Into<MaybeDim>) -> Self {
        Axis::Horizontal.dim_mut(&mut self.size).set_max(size);
        self
    }

    pub fn axis_grow(mut self, weight: impl Into<Fill>) -> Self {
        Axis::Horizontal.dim_mut(&mut self.size).set_grow(weight);
        self
    }

    pub fn axis_shrink(mut self, weight: impl Into<Fill>) -> Self {
        Axis::Horizontal.dim_mut(&mut self.size).set_shrink(weight);
        self
    }

    pub fn axis_depth(mut self, depth: impl Into<Unit>) -> Self {
        if !matches!(self.style.align_items(), AlignItems::Baseline) {
            tracing::warn!("Depth set for a box having items not aligned on a baseline");
        }
        self.size.set_depth(Some(depth));
        self
    }

    pub fn cross_size(mut self, size: impl Into<Dim>) -> Self {
        Axis::Horizontal
            .cross()
            .dim_mut(&mut self.size)
            .set_base(size);
        self
    }

    pub fn cross_min(mut self, size: impl Into<MaybeDim>) -> Self {
        Axis::Horizontal
            .cross()
            .dim_mut(&mut self.size)
            .set_min(size);
        self
    }

    pub fn cross_max(mut self, size: impl Into<MaybeDim>) -> Self {
        Axis::Horizontal
            .cross()
            .dim_mut(&mut self.size)
            .set_max(size);
        self
    }

    pub fn cross_grow(mut self, weight: impl Into<Fill>) -> Self {
        Axis::Horizontal
            .cross()
            .dim_mut(&mut self.size)
            .set_grow(weight);
        self
    }

    pub fn cross_shrink(mut self, weight: impl Into<Fill>) -> Self {
        Axis::Horizontal
            .cross()
            .dim_mut(&mut self.size)
            .set_shrink(weight);
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
        let style = child.style_ref().inherit(self.style_ref());
        child.size_mut().apply_style(Axis::Horizontal, &style);
        child.set_style(style);
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
                let style = child.style_ref().inherit(self.style_ref());
                child.size_mut().apply_style(Axis::Horizontal, &style);
                child.set_style(style);
                child
            })
            .collect::<Vec<_>>();
        self.children.extend(children);
        self
    }
}

impl Position for BlockBox {
    fn element(&self) -> &str {
        "BlockBox"
    }

    fn mark(&self) -> &str {
        self.mark.unwrap_or_default()
    }

    fn offset(&self) -> &Offset {
        &self.offset
    }

    fn offset_mut(&mut self) -> &mut Offset {
        &mut self.offset
    }

    fn size(&self) -> &Size {
        &self.size
    }

    fn size_mut(&mut self) -> &mut Size {
        &mut self.size
    }
}

impl Styled for BlockBox {
    fn style_ref(&self) -> &Style {
        self.style.as_ref()
    }

    fn set_style(&mut self, style: Arc<Style>) {
        self.size.apply_style(Axis::Horizontal, &style);
        self.style = style;
    }
}

impl Layout for BlockBox {
    fn measure(&mut self, ctx: &mut dyn MeasureContext, mut room: Size) -> Result<(), Error> {
        let axis_room = Axis::Horizontal.base_size(&room);
        Axis::Horizontal
            .dim_mut(self.size_mut())
            .resolve_parented(axis_room);

        let cross_room = Axis::Vertical.base_size(&room);
        Axis::Vertical
            .dim_mut(self.size_mut())
            .resolve_parented(cross_room);

        let ascent = if self.children.is_empty() {
            self.size.ascent()
        } else {
            self.style_ref().padding().narrow(None, Some(&mut room));
            for child in self.children.iter_mut() {
                child.measure(ctx, room.clone())?;
            }
            self.children
                .first()
                .unwrap()
                .size_after_wrap_ref()
                .ascent()
        };

        let respect_baseline = matches!(self.style_ref().align_items(), AlignItems::Baseline);
        if respect_baseline && self.size().depth().is_none() {
            self.size
                .set_depth(ascent.map(|ascent| self.size.base_height() - ascent));
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

        // Resolve relative positioning of request and self ascents, when aligning to baseline.
        let align_items = self.style_ref().align_items();
        let self_ascent = self.size_after_wrap_ref().ascent();
        if matches!(
            (align_items, room.depth(), self_ascent),
            (AlignItems::Baseline, Some(_), Some(_))
        ) {
            let self_to_parent_ascent = sub_unit(room.ascent(), self_ascent);
            offset.y_advance(self_to_parent_ascent.unwrap_or_default());
        }

        let ascent = if self.children.is_empty() {
            self.size.ascent()
        } else {
            for child in self.children.iter_mut() {
                let child_offset = child.offset() + &offset;
                child.lay_out(ctx, child_offset, child.size().clone())?;
            }
            self.children
                .first()
                .unwrap()
                .size_after_wrap_ref()
                .ascent()
        };

        // Adopt final offset and size including padding
        self.style_ref().padding().widen(Some(&mut offset), None);
        self.offset = offset;

        let respect_baseline = matches!(self.style_ref().align_items(), AlignItems::Baseline);
        if respect_baseline && self.size().depth().is_none() {
            self.size
                .set_depth(ascent.map(|ascent| self.size.base_height() - ascent));
        }

        Ok(())
    }

    fn render(&self, ctx: &mut dyn RenderContext) -> Result<(), Error> {
        ctx.check_page_break(self.offset.y, self.size.height.base_size(), true);

        for child in self.iter() {
            child.render(ctx)?;
        }

        let style = self.style_ref();
        let top_left = self.offset();
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

        ctx.debug_frame(self.offset(), self.size());

        ctx.release_page_break_reservation();

        Ok(())
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &Box<dyn Layout>> + '_> {
        Box::new(self.children.iter())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Layout, Position, Style, bbox, hbox,
        position::{Offset, Size},
        text, vbox,
    };

    #[test]
    fn text_in_bbox_from_zero() {
        let t1 = text("a");

        let mut t2 = text("b");
        *t2.offset_mut() = Offset::new(156000, 256000);

        let t3 = text("c");

        let mut bbox = bbox()
            .style(Style::new_default())
            .child(t1)
            .child(t2)
            .child(t3);

        bbox.measure(&mut 0_usize, Size::fixed(1000000, 2000000))
            .unwrap();
        bbox.lay_out(
            &mut 0_usize,
            Offset::new(0, 0),
            Size::fixed(1000000, 2000000),
        )
        .unwrap();

        let mut children = bbox.iter();

        let t1 = children.next().unwrap();
        assert_eq!(0, t1.offset().x.0);
        assert_eq!(0, t1.offset().y.0);

        let t2 = children.next().unwrap();
        assert_eq!(156000, t2.offset().x.0);
        assert_eq!(256000, t2.offset().y.0);

        let t3 = children.next().unwrap();
        assert_eq!(0, t3.offset().x.0);
        assert_eq!(0, t3.offset().y.0);
    }

    #[test]
    fn text_in_bbox_from_offset() {
        let t1 = text("a");

        let mut t2 = text("b");
        *t2.offset_mut() = Offset::new(156000, 256000);

        let t3 = text("c");

        let mut bbox = bbox()
            .style(Style::new_default())
            .child(t1)
            .child(t2)
            .child(t3);

        bbox.measure(&mut 0_usize, Size::fixed(1000000, 2000000))
            .unwrap();
        bbox.lay_out(
            &mut 0_usize,
            Offset::new(100000, 100000),
            Size::fixed(1000000, 2000000),
        )
        .unwrap();

        let mut children = bbox.iter();

        let t1 = children.next().unwrap();
        assert_eq!(100000, t1.offset().x.0);
        assert_eq!(100000, t1.offset().y.0);

        let t2 = children.next().unwrap();
        assert_eq!(256000, t2.offset().x.0);
        assert_eq!(356000, t2.offset().y.0);

        let t3 = children.next().unwrap();
        assert_eq!(100000, t3.offset().x.0);
        assert_eq!(100000, t3.offset().y.0);
    }

    #[test]
    fn text_in_hbox_in_bbox_from_zero() {
        let t1 = text("a").mark("t1");
        let t2 = text("b").mark("t2");
        let t3 = text("c").mark("t3");

        let mut bbox = bbox()
            .style(Style::new_default())
            .mark("h1")
            .child(hbox().mark("h2").child(t1).child(t2).child(t3));

        bbox.measure(&mut 0_usize, Size::fixed(1000000, 2000000))
            .unwrap();
        bbox.lay_out(
            &mut 0_usize,
            Offset::new(0, 0),
            Size::fixed(1000000, 2000000),
        )
        .unwrap();

        let mut children = bbox.iter();
        let mut children = children.next().unwrap().iter();

        let t1 = children.next().unwrap();
        assert_eq!(0, t1.offset().x.0);
        assert_eq!(0, t1.offset().y.0);

        let t2 = children.next().unwrap();
        assert_eq!(105833, t2.offset().x.0);
        assert_eq!(0, t2.offset().y.0);

        let t3 = children.next().unwrap();
        assert_eq!(2 * 105833, t3.offset().x.0);
        assert_eq!(0, t3.offset().y.0);
    }

    #[test]
    fn text_in_vbox_in_bbox() {
        let t1 = text("a");
        let t2 = text("b");
        let t3 = text("c");

        let mut bbox = bbox()
            .style(Style::new_default())
            .child(vbox().child(t1).child(t2).child(t3));

        bbox.measure(&mut 0_usize, Size::fixed(1000000, 2000000))
            .unwrap();
        bbox.lay_out(
            &mut 0_usize,
            Offset::new(0, 0),
            Size::fixed(1000000, 2000000),
        )
        .unwrap();

        let mut children = bbox.iter();
        let mut children = children.next().unwrap().iter();

        let t1 = children.next().unwrap();
        assert_eq!(0, t1.offset().x.0);
        assert_eq!(0, t1.offset().y.0);

        let t2 = children.next().unwrap();
        assert_eq!(0, t2.offset().x.0);
        assert_eq!(35278, t2.offset().y.0);

        let t3 = children.next().unwrap();
        assert_eq!(0, t3.offset().x.0);
        assert_eq!(2 * 35278, t3.offset().y.0);
    }
}
