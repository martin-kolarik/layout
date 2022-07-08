use crate::{
    dimension::{DimAutoOrParent, DimOrParent},
    position::Quad,
    unit::{Fill, FillPerMille, Pt, Unit},
    Features, Rgba, Styled,
};

#[derive(Debug, Clone, Copy)]
pub enum AlignItems {
    Start,
    Baseline,
    Center,
    End,
}

impl Default for AlignItems {
    fn default() -> Self {
        Self::Start
    }
}

#[derive(Debug, Clone)]
pub struct Font {
    name: &'static str,
    size: Pt,
    features: Features,
}

impl Font {
    pub fn new(name: &'static str, size: impl Into<Pt>, features: Option<Features>) -> Self {
        Self {
            name,
            size: size.into(),
            features: features.unwrap_or_default(),
        }
    }

    pub fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    pub fn set_size(&mut self, size: Pt) {
        self.size = size;
    }

    pub fn set_features(&mut self, features: Features) {
        self.features = features;
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn features(&self) -> &Features {
        &self.features
    }

    pub fn size(&self) -> Pt {
        self.size
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

#[derive(Debug, Clone)]
pub struct Style {
    font: Option<Font>,
    color: Option<Rgba>,
    background_color: Option<Rgba>,
    width: DimAutoOrParent,
    min_width: DimOrParent,
    max_width: DimOrParent,
    height: DimAutoOrParent,
    min_height: DimOrParent,
    max_height: DimOrParent,
    grow: Option<Fill>,
    shrink: Option<Fill>,
    border: Border,
    padding: Quad,
    align_items: Option<AlignItems>,
    gap: Option<Unit>,
}

impl Style {
    pub const fn new() -> Self {
        Self {
            font: None,
            color: None,
            background_color: None,
            width: DimAutoOrParent::None,
            min_width: DimOrParent::None,
            max_width: DimOrParent::None,
            height: DimAutoOrParent::None,
            min_height: DimOrParent::None,
            max_height: DimOrParent::None,
            grow: None,
            shrink: None,
            border: Border::none(),
            padding: Quad::empty(),
            align_items: None,
            gap: None,
        }
    }

    pub fn merge(&self, parent: &Style) -> Self {
        Self {
            font: self.font.as_ref().or(parent.font.as_ref()).cloned(),
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
            border: self.border.clone(),
            padding: self.padding.clone(),
            align_items: self.align_items,
            gap: self.gap,
        }
    }

    pub fn with_font(mut self, font: impl Into<Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    pub fn with_color(mut self, color: impl Into<Rgba>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn with_background_color(mut self, background_color: impl Into<Rgba>) -> Self {
        self.background_color = Some(background_color.into());
        self
    }

    pub fn with_width(mut self, width: impl Into<Unit>) -> Self {
        self.width = width.into().into();
        self
    }

    pub fn with_width_parent(mut self, fill: impl Into<FillPerMille>) -> Self {
        self.width = fill.into().into();
        self
    }

    pub fn with_max_width(mut self, max: impl Into<Unit>) -> Self {
        self.max_width = max.into().into();
        self
    }

    pub fn with_max_width_parent(mut self, max: impl Into<FillPerMille>) -> Self {
        self.max_width = max.into().into();
        self
    }

    pub fn with_min_width(mut self, min: impl Into<Unit>) -> Self {
        self.min_width = min.into().into();
        self
    }

    pub fn with_min_width_parent(mut self, min: impl Into<FillPerMille>) -> Self {
        self.min_width = min.into().into();
        self
    }

    pub fn with_height(mut self, height: impl Into<Unit>) -> Self {
        self.height = height.into().into();
        self
    }

    pub fn with_height_parent(mut self, fill: impl Into<FillPerMille>) -> Self {
        self.height = fill.into().into();
        self
    }

    pub fn with_max_height(mut self, max: impl Into<Unit>) -> Self {
        self.max_height = max.into().into();
        self
    }

    pub fn with_max_height_parent(mut self, max: impl Into<FillPerMille>) -> Self {
        self.max_height = max.into().into();
        self
    }

    pub fn with_min_height(mut self, min: impl Into<Unit>) -> Self {
        self.min_height = min.into().into();
        self
    }

    pub fn with_min_height_parent(mut self, min: impl Into<FillPerMille>) -> Self {
        self.min_height = min.into().into();
        self
    }

    pub fn with_grow(mut self, grow: impl Into<Fill>) -> Self {
        self.grow = Some(grow.into());
        self
    }

    pub fn with_shrink(mut self, shrink: impl Into<Fill>) -> Self {
        self.shrink = Some(shrink.into());
        self
    }

    pub fn with_border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    pub fn with_padding(mut self, padding: Quad) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_align_items(mut self, align_items: AlignItems) -> Self {
        self.align_items = Some(align_items);
        self
    }

    pub fn with_gap(mut self, gap: impl Into<Unit>) -> Self {
        self.gap = Some(gap.into());
        self
    }

    pub fn font(&self) -> Option<&Font> {
        self.font.as_ref()
    }

    pub fn font_mut(&mut self) -> Option<&mut Font> {
        self.font.as_mut()
    }

    pub fn color(&self) -> Option<&Rgba> {
        self.color.as_ref()
    }

    pub fn background_color(&self) -> Option<&Rgba> {
        self.background_color.as_ref()
    }

    pub fn width(&self) -> &DimAutoOrParent {
        &self.width
    }

    pub fn min_width(&self) -> &DimOrParent {
        &self.min_width
    }

    pub fn max_width(&self) -> &DimOrParent {
        &self.max_width
    }

    pub fn height(&self) -> &DimAutoOrParent {
        &self.height
    }

    pub fn min_height(&self) -> &DimOrParent {
        &self.min_height
    }

    pub fn max_height(&self) -> &DimOrParent {
        &self.max_height
    }

    pub fn grow(&self) -> Option<Fill> {
        self.grow
    }

    pub fn shrink(&self) -> Option<Fill> {
        self.shrink
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

    pub fn gap(&self) -> Option<Unit> {
        self.gap
    }

    pub fn gap_size(&self) -> Unit {
        self.gap.unwrap_or_default()
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            font: Some(Font::new("default", Pt(10.0), None)),
            color: Some(Rgba::black().clone()),
            background_color: None,
            width: DimAutoOrParent::Content(None),
            min_width: DimOrParent::None,
            max_width: DimOrParent::None,
            height: DimAutoOrParent::Content(None),
            min_height: DimOrParent::None,
            max_height: DimOrParent::None,
            grow: None,
            shrink: None,
            border: Border::none(),
            padding: Quad::empty(),
            align_items: None,
            gap: None,
        }
    }
}

impl Styled for Style {
    fn style_ref(&self) -> &Style {
        self
    }

    fn set_style(&mut self, style: Style) {
        *self = style;
    }

    fn adopt_parent_style(&mut self, parent: &Style) {
        *self = self.merge(parent);
    }

    fn override_style(&mut self, with: &Style) {
        *self = with.merge(self);
    }
}
