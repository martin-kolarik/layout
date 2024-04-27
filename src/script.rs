use smol_str::SmolStr;

use crate::{
    position::Quad,
    unit::{Fill, Pt, Unit},
    Border, DefaultFactory, Factory, Layout, StyleBuilder,
};

pub enum Element {
    Bbox(Option<Vec<Format>>, Vec<Element>),

    Hbox(Option<Vec<Format>>, Vec<Element>),
    Hspace(Unit),
    Hfill(Fill),

    Vbox(Option<Vec<Format>>, Vec<Element>),
    Vspace(Unit),
    Vfill(Fill),

    Text(Option<Vec<Format>>, String),
}

pub enum Format {
    Width(Unit),
    Height(Unit),
    Font(SmolStr),
    Points(Pt),
    Padding(Quad),
    Border(Border),
    Grow(Fill),
    Shrink(Fill),
}

pub fn lay_out(element: &Element) -> Box<dyn Layout> {
    match element {
        Element::Bbox(format, children) => {
            let mut bbox = DefaultFactory::bbox();
            apply_format(&mut bbox, format.as_deref());
            Box::new(
                children
                    .iter()
                    .fold(bbox, |bbox, child| bbox.child_dyn(lay_out(child))),
            )
        }

        Element::Hbox(format, children) => {
            let mut hbox = DefaultFactory::hbox();
            apply_format(&mut hbox, format.as_deref());
            Box::new(
                children
                    .iter()
                    .fold(hbox, |hbox, child| hbox.child_dyn(lay_out(child))),
            )
        }
        Element::Hspace(space) => Box::new(DefaultFactory::hspace(space.clone())),
        Element::Hfill(fill) => Box::new(DefaultFactory::hfill(fill.clone())),

        Element::Vbox(format, children) => {
            let mut vbox = DefaultFactory::vbox();
            apply_format(&mut vbox, format.as_deref());
            Box::new(
                children
                    .iter()
                    .fold(vbox, |vbox, child| vbox.child_dyn(lay_out(child))),
            )
        }
        Element::Vspace(space) => Box::new(DefaultFactory::vspace(space.clone())),
        Element::Vfill(fill) => Box::new(DefaultFactory::vfill(fill.clone())),

        Element::Text(format, text) => {
            let mut text = DefaultFactory::text(text);
            apply_format(&mut text, format.as_deref());
            Box::new(text)
        }
    }
}

fn apply_format(layout: &mut dyn Layout, format: Option<&[Format]>) {
    if let Some(formats) = format {
        let style = formats
            .iter()
            .fold(StyleBuilder::new(), |style, format| match format {
                Format::Width(width) => style.with_width(width.clone()),
                Format::Height(height) => style.with_height(height.clone()),
                Format::Font(font) => style.with_font_name(font.clone()),
                Format::Points(points) => style.with_font_size(points.clone()),
                Format::Padding(padding) => style.with_padding(padding.clone()),
                Format::Border(border) => style.with_border(border.clone()),
                Format::Grow(grow) => style.with_grow(grow.clone()),
                Format::Shrink(shrink) => style.with_shrink(shrink.clone()),
            });
        layout.set_style(style.build());
    }
}
