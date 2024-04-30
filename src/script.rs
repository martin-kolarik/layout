use smol_str::SmolStr;

use crate::{
    position::Quad,
    unit::{Fill, Pt, Unit},
    AlignItems, Border, DefaultFactory, Factory, Layout, Rgba, StyleBuilder,
};

pub enum Element {
    Bbox(Vec<Format>, Vec<Element>),

    Hbox(Vec<Format>, Vec<Element>),
    Hspace(Unit),
    Hfill(Fill),
    HfillShrink(Fill),

    Vbox(Vec<Format>, Vec<Element>),
    Vspace(Unit),
    Vfill(Fill),
    VfillShrink(Fill),

    Text(Vec<Format>, String),
}

#[derive(Clone)]
pub enum Format {
    Width(Unit),
    WidthParent(Fill),
    Height(Unit),
    HeightParent(Fill),
    Font(SmolStr),
    Points(Pt),
    Scaling(Fill),
    Padding(Quad),
    Border(Border),
    Grow(Fill),
    Shrink(Fill),
    Align(AlignItems),
    Color(Rgba),
    Background(Rgba),
}

impl From<Format> for Vec<Format> {
    fn from(format: Format) -> Self {
        vec![format]
    }
}

pub fn lay_out(element: &Element) -> Box<dyn Layout> {
    match element {
        Element::Bbox(format, children) => {
            let mut bbox = DefaultFactory::bbox();
            apply_format(&mut bbox, &format);
            Box::new(
                children
                    .iter()
                    .fold(bbox, |bbox, child| bbox.child_dyn(lay_out(child))),
            )
        }

        Element::Hbox(format, children) => {
            let mut hbox = DefaultFactory::hbox();
            apply_format(&mut hbox, &format);
            Box::new(
                children
                    .iter()
                    .fold(hbox, |hbox, child| hbox.child_dyn(lay_out(child))),
            )
        }
        Element::Hspace(space) => Box::new(DefaultFactory::hspace(space.clone())),
        Element::Hfill(fill) => Box::new(DefaultFactory::hfill(fill.clone())),
        Element::HfillShrink(fill) => Box::new(
            DefaultFactory::hfilling()
                .grow(fill.clone())
                .shrink(fill.clone()),
        ),

        Element::Vbox(format, children) => {
            let mut vbox = DefaultFactory::vbox();
            apply_format(&mut vbox, &format);
            Box::new(
                children
                    .iter()
                    .fold(vbox, |vbox, child| vbox.child_dyn(lay_out(child))),
            )
        }
        Element::Vspace(space) => Box::new(DefaultFactory::vspace(space.clone())),
        Element::Vfill(fill) => Box::new(DefaultFactory::vfill(fill.clone())),
        Element::VfillShrink(fill) => Box::new(
            DefaultFactory::vfilling()
                .grow(fill.clone())
                .shrink(fill.clone()),
        ),

        Element::Text(format, text) => {
            let mut text = DefaultFactory::text(text);
            apply_format(&mut text, &format);
            Box::new(text)
        }
    }
}

fn apply_format(layout: &mut dyn Layout, format: &[Format]) {
    if !format.is_empty() {
        let style = format
            .iter()
            .fold(StyleBuilder::new(), |style, format| match format {
                Format::Width(width) => style.with_width(width.clone()),
                Format::WidthParent(fill) => style.with_width_parent(fill.clone()),
                Format::Height(height) => style.with_height(height.clone()),
                Format::HeightParent(fill) => style.with_height_parent(fill.clone()),
                Format::Font(font) => style.with_font_name(font.clone()),
                Format::Points(points) => style.with_font_size(points.clone()),
                Format::Scaling(scaling) => style.with_font_scaling(scaling.clone()),
                Format::Padding(padding) => style.with_padding(padding.clone()),
                Format::Border(border) => style.with_border(border.clone()),
                Format::Grow(grow) => style.with_grow(grow.clone()),
                Format::Shrink(shrink) => style.with_shrink(shrink.clone()),
                Format::Align(align) => style.with_align_items(align.clone()),
                Format::Color(color) => style.with_color(color.clone()),
                Format::Background(color) => style.with_background_color(color.clone()),
            });
        layout.set_style(style.build());
    }
}
