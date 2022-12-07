use crate::{MeasureContext, Style, TextPosition};

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
