use rtext::Apply;

use crate::{
    dimension::{DimAutoOrParent, DimOrParent},
    position::{Offset, Size},
    unit::{Fill, Unit},
    Axis, ChildrenIterator, Error, Layout, LayoutBox, MeasureContext, Position, RenderContext,
    Style, Styled,
};

pub struct DecoratedBox {
    mark: Option<&'static str>,
    offset: Offset,
    size: Size,
    native_size: Option<Size>,
    inner: LayoutBox,
}

impl DecoratedBox {
    pub fn new(axis: Axis) -> Self {
        Self {
            mark: None,
            offset: Offset::zero(),
            size: Size::new_auto(),
            native_size: None,
            inner: LayoutBox::new(axis),
        }
    }

    pub fn mark(mut self, mark: &'static str) -> Self {
        self.mark = Some(mark);
        self.inner = self.inner.mark("(i)");
        self
    }

    pub fn size(mut self, size: impl Into<DimAutoOrParent>) -> Self {
        self.inner.axis().dim_mut(&mut self.size).set_basis(size);
        self
    }

    pub fn min(mut self, size: impl Into<DimOrParent>) -> Self {
        self.inner.axis().dim_mut(&mut self.size).set_min(size);
        self
    }

    pub fn max(mut self, size: impl Into<DimOrParent>) -> Self {
        self.inner.axis().dim_mut(&mut self.size).set_max(size);
        self
    }

    pub fn grow(mut self, weight: impl Into<Fill>) -> Self {
        self.inner.axis().dim_mut(&mut self.size).set_grow(weight);
        self
    }

    pub fn shrink(mut self, weight: impl Into<Fill>) -> Self {
        self.inner.axis().dim_mut(&mut self.size).set_shrink(weight);
        self
    }

    pub fn depth(mut self, depth: impl Into<Unit>) -> Self {
        self.size.set_depth(Some(depth));
        self
    }

    pub fn cross_size(mut self, size: impl Into<DimAutoOrParent>) -> Self {
        self.inner
            .axis()
            .cross()
            .dim_mut(&mut self.size)
            .set_basis(size);
        self
    }

    pub fn cross_min(mut self, size: impl Into<DimOrParent>) -> Self {
        self.inner
            .axis()
            .cross()
            .dim_mut(&mut self.size)
            .set_min(size);
        self
    }

    pub fn cross_max(mut self, size: impl Into<DimOrParent>) -> Self {
        self.inner
            .axis()
            .cross()
            .dim_mut(&mut self.size)
            .set_max(size);
        self
    }

    pub fn cross_grow(mut self, weight: impl Into<Fill>) -> Self {
        self.inner
            .axis()
            .cross()
            .dim_mut(&mut self.size)
            .set_grow(weight);
        self
    }

    pub fn cross_shrink(mut self, weight: impl Into<Fill>) -> Self {
        self.inner
            .axis()
            .cross()
            .dim_mut(&mut self.size)
            .set_shrink(weight);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.inner = self.inner.style(style);
        self
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
        self.inner = self.inner.children_box(children);
        self
    }

    pub fn child_box(mut self, mut child: Box<dyn Layout>) -> Self {
        child.adopt_parent_style(self.style_ref());
        self.inner = self.inner.child_box(child);
        self
    }
}

impl Apply for DecoratedBox {}

impl Position for DecoratedBox {
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

    fn native_size(&self) -> Option<&Size> {
        self.native_size.as_ref()
    }

    fn content_size(&self) -> Option<&Size> {
        Some(self.inner.size_ref())
    }
}

impl Styled for DecoratedBox {
    fn style_ref(&self) -> &Style {
        self.inner.style_ref()
    }

    fn set_style(&mut self, style: Style) {
        self.inner.set_style(style);
    }

    fn adopt_parent_style(&mut self, parent: &Style) {
        self.inner.adopt_parent_style(parent);
    }

    fn override_style(&mut self, with: &Style) {
        self.inner.override_style(with);
    }
}

impl Layout for DecoratedBox {
    fn measure(&mut self, ctx: &mut dyn MeasureContext, available: Size) -> Result<(), Error> {
        self.inner.take_over_position(self.offset.clone(), self.size.clone());
        self.inner.measure(ctx, available)?;

        let mut native_size = self.inner.native_size().cloned().unwrap_or_else(Size::zero);
        self.inner
            .style_ref()
            .padding()
            .widen(None, Some(&mut native_size));
        self.native_size = Some(native_size);

        Ok(())
    }

    fn lay_out(
        &mut self,
        ctx: &mut dyn MeasureContext,
        mut outer_position: Offset,
        mut outer_size: Size,
    ) -> Result<(), Error> {
        let style = self.inner.style_ref();
        let axis = self.inner.axis();
        let cross = axis.cross();
        let outer_axis_size = axis.size(&outer_size);
        let outer_cross_size = cross.size(&outer_size);

        self.offset = outer_position.clone();

        style
            .padding()
            .narrow(Some(&mut outer_position), Some(&mut outer_size));

        let mut inner_position = self.offset.clone();
        let mut inner_size = self.size.clone();
        style
            .padding()
            .narrow(Some(&mut inner_position), Some(&mut inner_size));

        self.inner.take_over_position(inner_position, inner_size);
        self.inner.lay_out(ctx, outer_position, outer_size)?;

        let mut content_size = self.inner.content_size().unwrap().clone();
        self.inner
            .style_ref()
            .padding()
            .widen(None, Some(&mut content_size));

        axis.resolve_content_size(&mut self.size, &content_size, outer_axis_size);
        cross.resolve_content_size(&mut self.size, &content_size, outer_cross_size);
        if let (None, Some(depth)) = (self.size.depth(), self.inner.size_ref().depth()) {
            self.size
                .set_depth(Some(depth + self.inner.style_ref().padding().bottom_size()));
        }

        Ok(())
    }

    fn render(&self, context: &mut dyn RenderContext) -> Result<(), Error> {
        self.inner.render(context)?;

        let style = self.inner.style_ref();
        let top_left = self.offset_ref();
        let bottom_right = top_left + &self.size;

        if let Some(stroke) = style.border_top() {
            context.line(
                &self.offset,
                &Offset::new(bottom_right.x(), top_left.y()),
                stroke,
            );
        }

        if let Some(stroke) = style.border_right() {
            context.line(
                &Offset::new(bottom_right.x(), top_left.y()),
                &bottom_right,
                stroke,
            );
        }

        if let Some(stroke) = style.border_bottom() {
            context.line(
                &bottom_right,
                &Offset::new(top_left.x(), bottom_right.y()),
                stroke,
            );
        }

        if let Some(stroke) = style.border_left() {
            context.line(
                &Offset::new(top_left.x(), bottom_right.y()),
                top_left,
                stroke,
            );
        }

        context.debug_frame(self.offset_ref(), self.size_ref());

        Ok(())
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &Box<dyn Layout>> + '_> {
        Box::new(ChildrenIterator::new(&self.inner))
    }
}
