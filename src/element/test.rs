use std::sync::{Arc, OnceLock};

use crate::{
    Error, GlyphPosition, MeasureContext, NewPageOptions, RenderContext, Stroke, Style,
    TextPosition,
    position::{Offset, Size},
    unit::Em,
};

pub(crate) mod baseline;
pub(crate) mod hbox_in_hbox;
pub(crate) mod hbox_in_vbox;
pub(crate) mod various;
pub(crate) mod vbox_in_hbox;
pub(crate) mod vbox_in_vbox;

pub(crate) struct Ctx;

impl MeasureContext for Ctx {
    fn style(&self) -> &Style {
        todo!()
    }

    fn typeset(&mut self, _: &Style, _: &str) -> Result<TextPosition, crate::Error> {
        todo!()
    }
}

static STYLE: OnceLock<Arc<Style>> = std::sync::OnceLock::new();

impl MeasureContext for usize {
    fn style(&self) -> &Style {
        STYLE.get_or_init(Style::new)
    }

    fn typeset(&mut self, _: &Style, _: &str) -> Result<TextPosition, Error> {
        Ok(TextPosition {
            width: Em(30.0),
            height: Em(10.0),
            depth: Em(2.0),
            positions: vec![GlyphPosition::new(
                None,
                1,
                Em(30.0),
                Em(0.0),
                Em(0.0),
                Em(0.0),
            )],
        })
    }
}

impl RenderContext for usize {
    fn new_page(&mut self, _: Option<NewPageOptions>) -> bool {
        todo!()
    }

    fn debug_frame(&mut self, _: &Offset, _: &Size) {
        todo!()
    }

    fn line(&mut self, _: &Offset, _: &Offset, _: &Stroke) {
        todo!()
    }

    fn text(&mut self, _: &Offset, _: &Style, _: &TextPosition, _: bool) {
        todo!()
    }
}
