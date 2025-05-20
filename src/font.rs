use allsorts::gsub;

use crate::unit::Em;

#[derive(Debug, Clone)]
pub struct Features {
    pnum: Option<bool>,
    lnum: Option<bool>,
    smcp: Option<()>,
}

impl Features {
    pub fn empty() -> Self {
        Self {
            pnum: None,
            lnum: None,
            smcp: None,
        }
    }

    pub fn pnum(mut self) -> Self {
        self.pnum = Some(true);
        self
    }

    pub fn tnum(mut self) -> Self {
        self.pnum = Some(false);
        self
    }

    pub fn lnum(mut self) -> Self {
        self.lnum = Some(true);
        self
    }

    pub fn onum(mut self) -> Self {
        self.lnum = Some(false);
        self
    }

    pub fn smcp(mut self) -> Self {
        self.smcp = Some(());
        self
    }
}

impl Default for Features {
    fn default() -> Self {
        Self {
            pnum: Some(true),
            lnum: Some(true),
            smcp: None,
        }
    }
}

#[derive(Debug)]
pub struct TextPosition {
    pub width: Em,
    pub height: Em,
    pub depth: Em,
    pub positions: Vec<GlyphPosition>,
}

impl TextPosition {
    pub fn ascent(&self) -> Em {
        self.height - self.depth
    }
}

#[derive(Debug)]
pub struct GlyphPosition {
    pub character: Option<char>,
    pub glyph_index: u16,
    pub h_advance: Em,
    pub v_advance: Em,
    pub h_offset: Em,
    pub v_offset: Em,
}

impl GlyphPosition {
    pub fn new(
        character: Option<char>,
        glyph_index: u16,
        h_advance: Em,
        v_advance: Em,
        h_offset: Em,
        v_offset: Em,
    ) -> Self {
        GlyphPosition {
            character,
            glyph_index,
            h_advance,
            v_advance,
            h_offset,
            v_offset,
        }
    }

    pub fn set_glyph_index(&mut self, index: u16) {
        self.glyph_index = index;
    }

    pub fn h_advance_rest(&self) -> Em {
        self.h_advance - self.h_offset
    }

    pub fn v_advance_rest(&self) -> Em {
        if self.v_advance.is_zero() && self.v_offset.is_zero() {
            Em(0.0)
        } else {
            self.v_advance - self.v_offset
        }
    }
}

impl From<&Features> for gsub::Features {
    fn from(features: &Features) -> Self {
        let mut mask = gsub::FeatureMask::default();
        if features.smcp.is_some() {
            mask |= gsub::FeatureMask::SMCP;
        }
        if let Some(lnum) = features.lnum {
            if lnum {
                mask |= gsub::FeatureMask::LNUM;
            } else {
                mask |= gsub::FeatureMask::ONUM;
            }
        }
        if let Some(pnum) = features.pnum {
            if pnum {
                mask |= gsub::FeatureMask::PNUM;
            } else {
                mask |= gsub::FeatureMask::TNUM;
            }
        }
        gsub::Features::Mask(mask)
    }
}
