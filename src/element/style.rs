use std::sync::Arc;

use smol_str::{SmolStr, ToSmolStr};

use crate::{
    Features, Rgba, Styled,
    dimension::{Dim, MaybeDim},
    position::Quad,
    unit::{Fill, FillPerMille, Pt, Unit},
};

#[derive(Default, Debug, Clone, Copy)]
pub enum AlignItems {
    #[default]
    Start,
    Baseline,
    Center,
    End,
}

#[derive(Debug, Clone)]
pub struct Font {
    name: Option<SmolStr>,
    size: Option<Pt>,
    features: Option<Features>,
    scaling: Option<FillPerMille>,
}

impl Font {
    pub fn new(name: impl ToSmolStr, size: impl Into<Pt>, features: Option<Features>) -> Self {
        Self {
            name: Some(name.to_smolstr()),
            size: Some(size.into()),
            features,
            scaling: None,
        }
    }

    const fn __internal_new() -> Self {
        Self {
            name: None,
            size: None,
            features: None,
            scaling: None,
        }
    }

    pub fn merge(&self, parent: &Self) -> Self {
        Self {
            name: self.name.as_ref().or(parent.name.as_ref()).cloned(),
            size: self.size.or(parent.size),
            features: self.features.as_ref().or(parent.features.as_ref()).cloned(),
            scaling: self.scaling.or(parent.scaling),
        }
    }

    pub fn set_name(&mut self, name: impl ToSmolStr) {
        self.name = Some(name.to_smolstr());
    }

    pub fn set_size(&mut self, size: Pt) {
        self.size = Some(size);
    }

    pub fn set_features(&mut self, features: Features) {
        self.features = Some(features);
    }

    pub fn set_scaling(&mut self, scaling: FillPerMille) {
        self.scaling = Some(scaling);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn features(&self) -> Option<&Features> {
        self.features.as_ref()
    }

    pub fn size(&self) -> Option<Pt> {
        self.size
    }

    pub fn scaling(&self) -> Option<FillPerMille> {
        self.scaling
    }
}

impl From<&Font> for Font {
    fn from(font: &Font) -> Self {
        font.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Stroke {
    color: Rgba,
    thickness: Pt,
}

impl Stroke {
    pub fn new(color: impl Into<Rgba>, thickness: impl Into<Pt>) -> Self {
        Self {
            color: color.into(),
            thickness: thickness.into(),
        }
    }

    pub fn with_color(mut self, color: impl Into<Rgba>) -> Self {
        self.color = color.into();
        self
    }

    pub fn with_thickness(mut self, thickness: impl Into<Pt>) -> Self {
        self.thickness = thickness.into();
        self
    }

    pub fn color(&self) -> &Rgba {
        &self.color
    }

    pub fn thickness(&self) -> Pt {
        self.thickness
    }
}

impl From<&Stroke> for Stroke {
    fn from(stroke: &Stroke) -> Self {
        stroke.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Border {
    top: Option<Stroke>,
    left: Option<Stroke>,
    bottom: Option<Stroke>,
    right: Option<Stroke>,
}

impl Border {
    pub const fn none() -> Self {
        Self {
            top: None,
            left: None,
            bottom: None,
            right: None,
        }
    }

    pub fn merge(&self, parent: &Self) -> Self {
        Self {
            top: self.top.as_ref().or(parent.top.as_ref()).cloned(),
            left: self.left.as_ref().or(parent.left.as_ref()).cloned(),
            bottom: self.bottom.as_ref().or(parent.bottom.as_ref()).cloned(),
            right: self.right.as_ref().or(parent.right.as_ref()).cloned(),
        }
    }

    pub fn square(stroke: impl Into<Stroke>) -> Self {
        let stroke = stroke.into();
        Self {
            top: Some(stroke.clone()),
            left: Some(stroke.clone()),
            bottom: Some(stroke.clone()),
            right: Some(stroke),
        }
    }

    pub fn h_v(horizontal: impl Into<Stroke>, vertical: impl Into<Stroke>) -> Self {
        let horizontal = horizontal.into();
        let vertical = vertical.into();
        Self {
            top: Some(horizontal.clone()),
            left: Some(vertical.clone()),
            bottom: Some(horizontal),
            right: Some(vertical),
        }
    }

    pub fn h(horizontal: impl Into<Stroke>) -> Self {
        let horizontal = horizontal.into();
        Self {
            top: Some(horizontal.clone()),
            left: None,
            bottom: Some(horizontal),
            right: None,
        }
    }

    pub fn v(vertical: impl Into<Stroke>) -> Self {
        let vertical = vertical.into();
        Self {
            top: None,
            left: Some(vertical.clone()),
            bottom: None,
            right: Some(vertical),
        }
    }

    pub fn with_top(mut self, top: impl Into<Stroke>) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn with_left(mut self, left: impl Into<Stroke>) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn with_bottom(mut self, bottom: impl Into<Stroke>) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn with_right(mut self, right: impl Into<Stroke>) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn top(&self) -> Option<&Stroke> {
        self.top.as_ref()
    }

    pub fn left(&self) -> Option<&Stroke> {
        self.left.as_ref()
    }

    pub fn bottom(&self) -> Option<&Stroke> {
        self.bottom.as_ref()
    }

    pub fn right(&self) -> Option<&Stroke> {
        self.right.as_ref()
    }
}

impl From<&Border> for Border {
    fn from(border: &Border) -> Self {
        border.clone()
    }
}

#[derive(Debug)]
pub struct Style {
    font: Font,
    color: Option<Rgba>,
    background_color: Option<Rgba>,
    width: Dim,
    min_width: MaybeDim,
    max_width: MaybeDim,
    height: Dim,
    min_height: MaybeDim,
    max_height: MaybeDim,
    grow: Option<Fill>,
    shrink: Option<Fill>,
    wrap: Option<bool>,
    align_items: Option<AlignItems>,
    horizontal_gap: Option<Unit>,
    vertical_gap: Option<Unit>,
    border: Border,
    padding: Quad,
}

impl Styled for Arc<Style> {
    fn style_ref(&self) -> &Style {
        self.as_ref()
    }

    fn set_style(&mut self, style: Arc<Style>) {
        *self = style;
    }
}

impl Style {
    const fn __internal_new() -> Self {
        Self {
            font: Font::__internal_new(),
            color: None,
            background_color: None,
            width: Dim::content(),
            min_width: MaybeDim::None,
            max_width: MaybeDim::None,
            height: Dim::content(),
            min_height: MaybeDim::None,
            max_height: MaybeDim::None,
            grow: None,
            shrink: None,
            wrap: None,
            align_items: None,
            horizontal_gap: None,
            vertical_gap: None,
            border: Border::none(),
            padding: Quad::empty(),
        }
    }

    fn __internal_default() -> Style {
        Style {
            font: Font::new("default", Pt(10.0), None),
            color: Some(Rgba::black().clone()),
            width: Dim::Content(None),
            height: Dim::Content(None),
            ..Self::__internal_new()
        }
    }

    pub fn new() -> Arc<Self> {
        Arc::new(Self::__internal_new())
    }

    pub fn new_default() -> Arc<Style> {
        Arc::new(Self::__internal_default())
    }

    pub fn inherit(&self, parent: &Style) -> Arc<Self> {
        let align_items = if matches!(self.align_items, None)
            && matches!(parent.align_items, Some(AlignItems::Baseline))
        {
            Some(AlignItems::Baseline)
        } else {
            self.align_items
        };

        Arc::new(Self {
            font: self.font.merge(&parent.font),
            color: self.color.as_ref().or(parent.color.as_ref()).cloned(),
            background_color: self
                .background_color
                .as_ref()
                .or(parent.background_color.as_ref())
                .cloned(),
            width: self.width.clone(),
            min_width: self.min_width.clone(),
            max_width: self.max_width.clone(),
            height: self.height.clone(),
            min_height: self.min_height.clone(),
            max_height: self.max_height.clone(),
            grow: self.grow,
            shrink: self.shrink,
            wrap: self.wrap,
            align_items,
            horizontal_gap: self.horizontal_gap,
            vertical_gap: self.vertical_gap,
            border: self.border.clone(),
            padding: self.padding.clone(),
        })
    }

    pub fn merge(&self, parent: &Style) -> Arc<Self> {
        Arc::new(Self {
            font: self.font.merge(&parent.font),
            color: self.color.as_ref().or(parent.color.as_ref()).cloned(),
            background_color: self
                .background_color
                .as_ref()
                .or(parent.background_color.as_ref())
                .cloned(),
            width: self.width.or(parent.width),
            min_width: self.min_width.or(parent.min_width),
            max_width: self.max_width.or(parent.max_width),
            height: self.height.or(parent.height),
            min_height: self.min_height.or(parent.min_height),
            max_height: self.max_height.or(parent.max_height),
            grow: self.grow.as_ref().or(parent.grow.as_ref()).cloned(),
            shrink: self.shrink.as_ref().or(parent.shrink.as_ref()).cloned(),
            wrap: self.wrap.as_ref().or(parent.wrap.as_ref()).cloned(),
            align_items: self.align_items.or(parent.align_items),
            horizontal_gap: self.horizontal_gap.or(parent.horizontal_gap),
            vertical_gap: self.vertical_gap.or(parent.vertical_gap),
            border: self.border.merge(&parent.border),
            padding: self.padding.merge(&parent.padding),
        })
    }

    pub fn font(&self) -> &Font {
        &self.font
    }

    pub fn color(&self) -> Option<&Rgba> {
        self.color.as_ref()
    }

    pub fn background_color(&self) -> Option<&Rgba> {
        self.background_color.as_ref()
    }

    pub fn width(&self) -> Dim {
        self.width
    }

    pub fn min_width(&self) -> MaybeDim {
        self.min_width
    }

    pub fn max_width(&self) -> MaybeDim {
        self.max_width
    }

    pub fn height(&self) -> Dim {
        self.height
    }

    pub fn min_height(&self) -> MaybeDim {
        self.min_height
    }

    pub fn max_height(&self) -> MaybeDim {
        self.max_height
    }

    pub fn grow(&self) -> Option<Fill> {
        self.grow
    }

    pub fn shrink(&self) -> Option<Fill> {
        self.shrink
    }

    pub fn wrap(&self) -> Option<bool> {
        self.wrap
    }

    pub fn border_top(&self) -> Option<&Stroke> {
        self.border.top()
    }

    pub fn border_left(&self) -> Option<&Stroke> {
        self.border.left()
    }

    pub fn border_bottom(&self) -> Option<&Stroke> {
        self.border.bottom()
    }

    pub fn border_right(&self) -> Option<&Stroke> {
        self.border.right()
    }

    pub fn padding(&self) -> &Quad {
        &self.padding
    }

    pub fn padding_top(&self) -> Option<Unit> {
        self.padding.top()
    }

    pub fn padding_top_size(&self) -> Unit {
        self.padding.top_size()
    }

    pub fn padding_left(&self) -> Option<Unit> {
        self.padding.left()
    }

    pub fn padding_left_size(&self) -> Unit {
        self.padding.left_size()
    }

    pub fn padding_bottom(&self) -> Option<Unit> {
        self.padding.bottom()
    }

    pub fn padding_bottom_size(&self) -> Unit {
        self.padding.bottom_size()
    }

    pub fn padding_right(&self) -> Option<Unit> {
        self.padding.right()
    }

    pub fn padding_right_size(&self) -> Unit {
        self.padding.right_size()
    }

    pub fn align_items(&self) -> AlignItems {
        self.align_items.unwrap_or_default()
    }

    pub fn horizontal_gap(&self) -> Option<Unit> {
        self.horizontal_gap
    }

    pub fn horizontal_gap_size(&self) -> Unit {
        self.horizontal_gap.unwrap_or_default()
    }

    pub fn vertical_gap(&self) -> Option<Unit> {
        self.vertical_gap
    }

    pub fn vertical_gap_size(&self) -> Unit {
        self.vertical_gap.unwrap_or_default()
    }
}

#[derive(Debug)]
pub struct StyleBuilder {
    style: Style,
}

impl Default for StyleBuilder {
    fn default() -> Self {
        Self {
            style: Style::__internal_default(),
        }
    }
}

impl From<StyleBuilder> for Arc<Style> {
    fn from(builder: StyleBuilder) -> Self {
        builder.build()
    }
}

impl StyleBuilder {
    pub fn new() -> Self {
        Self {
            style: Style::__internal_new(),
        }
    }

    pub fn build(self) -> Arc<Style> {
        Arc::new(self.style)
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn with_font(mut self, font: impl Into<Font>) -> Self {
        self.style.font = font.into();
        self
    }

    pub fn with_font_name(mut self, name: impl ToSmolStr) -> Self {
        self.style.font.set_name(name);
        self
    }

    pub fn with_font_size(mut self, size: Pt) -> Self {
        self.style.font.set_size(size);
        self
    }

    pub fn with_font_features(mut self, features: Features) -> Self {
        self.style.font.set_features(features);
        self
    }

    pub fn with_font_scaling(mut self, scaling: FillPerMille) -> Self {
        self.style.font.set_scaling(scaling);
        self
    }

    pub fn with_color(mut self, color: impl Into<Rgba>) -> Self {
        self.style.color = Some(color.into());
        self
    }

    pub fn with_background_color(mut self, background_color: impl Into<Rgba>) -> Self {
        self.style.background_color = Some(background_color.into());
        self
    }

    pub fn with_width(mut self, width: impl Into<Unit>) -> Self {
        self.style.width = width.into().into();
        self
    }

    pub fn with_width_parent(mut self, fill: impl Into<FillPerMille>) -> Self {
        self.style.width = fill.into().into();
        self
    }

    pub fn with_max_width(mut self, max: impl Into<Unit>) -> Self {
        self.style.max_width = max.into().into();
        self
    }

    pub fn with_max_width_parent(mut self, max: impl Into<FillPerMille>) -> Self {
        self.style.max_width = max.into().into();
        self
    }

    pub fn with_min_width(mut self, min: impl Into<Unit>) -> Self {
        self.style.min_width = min.into().into();
        self
    }

    pub fn with_min_width_parent(mut self, min: impl Into<FillPerMille>) -> Self {
        self.style.min_width = min.into().into();
        self
    }

    pub fn with_height(mut self, height: impl Into<Unit>) -> Self {
        self.style.height = height.into().into();
        self
    }

    pub fn with_height_parent(mut self, fill: impl Into<FillPerMille>) -> Self {
        self.style.height = fill.into().into();
        self
    }

    pub fn with_max_height(mut self, max: impl Into<Unit>) -> Self {
        self.style.max_height = max.into().into();
        self
    }

    pub fn with_max_height_parent(mut self, max: impl Into<FillPerMille>) -> Self {
        self.style.max_height = max.into().into();
        self
    }

    pub fn with_min_height(mut self, min: impl Into<Unit>) -> Self {
        self.style.min_height = min.into().into();
        self
    }

    pub fn with_min_height_parent(mut self, min: impl Into<FillPerMille>) -> Self {
        self.style.min_height = min.into().into();
        self
    }

    pub fn with_grow(mut self, grow: impl Into<Fill>) -> Self {
        self.style.grow = Some(grow.into());
        self
    }

    pub fn with_shrink(mut self, shrink: impl Into<Fill>) -> Self {
        self.style.shrink = Some(shrink.into());
        self
    }

    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.style.wrap = Some(wrap);
        self
    }

    pub fn with_border(mut self, border: impl Into<Border>) -> Self {
        self.style.border = border.into();
        self
    }

    pub fn with_padding(mut self, padding: impl Into<Quad>) -> Self {
        self.style.padding = padding.into();
        self
    }

    pub fn with_align_items(mut self, align_items: AlignItems) -> Self {
        self.style.align_items = Some(align_items);
        self
    }

    pub fn with_horizontal_gap(mut self, gap: impl Into<Unit>) -> Self {
        self.style.horizontal_gap = Some(gap.into());
        self
    }

    pub fn with_vertical_gap(mut self, gap: impl Into<Unit>) -> Self {
        self.style.vertical_gap = Some(gap.into());
        self
    }
}
