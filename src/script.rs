use crate::{
    position::Quad,
    unit::{Fill, Pt, Unit},
    Border,
};

pub enum Element {
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
    Font(String),
    Points(Pt),
    Padding(Quad),
    Border(Border),
}
