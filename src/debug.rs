use std::fmt::Debug;

use crate::Layout;

impl Debug for dyn Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if self.mark() == "" {
            format!("{}({})", self.element(), self.mark())
        } else {
            self.element().to_owned()
        };

        f.debug_struct(&name)
            .field("offset", self.offset_ref())
            .field("size", self.size_ref())
            .field("content_size", &self.content_size())
            .finish()?;

        f.debug_list().entries(self.iter()).finish()
    }
}
