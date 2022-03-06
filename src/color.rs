use oklab::{oklab_to_srgb, srgb_to_oklab, Oklab, RGB};
use rgb::RGBA;

#[derive(Clone, PartialEq)]
pub struct Rgba(RGBA<u8, f32>);

impl Rgba {
    pub fn black() -> &'static Rgba {
        &BLACK
    }

    pub fn white() -> &'static Rgba {
        &WHITE
    }

    #[must_use]
    pub fn alpha(self, fraction: f32) -> Self {
        Self(self.0.alpha(fraction))
    }

    pub fn into_rgba(self) -> (f64, f64, f64, f64) {
        (
            self.0.r as f64 / 255.0,
            self.0.g as f64 / 255.0,
            self.0.b as f64 / 255.0,
            self.0.a as f64,
        )
    }

    fn into_lcha(self) -> (f32, f32, f32, f32) {
        let Oklab { l, a, b } = srgb_to_oklab(self.0.rgb());
        let c = (a.powi(2) + b.powi(2)).sqrt();
        let h = b.atan2(a);
        (l, c, h, self.0.a)
    }

    #[allow(clippy::many_single_char_names)]
    fn from_lcha((l, c, h, a): (f32, f32, f32, f32)) -> Self {
        let (y, x) = h.sin_cos();
        let oklab = Oklab {
            l,
            a: c * x,
            b: c * y,
        };
        let RGB { r, g, b } = oklab_to_srgb(oklab);
        Self(RGBA { r, g, b, a })
    }

    #[must_use]
    pub fn lighten(self, fraction: f32) -> Self {
        let mut lcha = self.into_lcha();
        lcha.0 = if lcha.0 == 0.0 {
            0.01
        } else {
            lcha.0 * (1.0 + fraction)
        }
        .clamp(0.0, 1.0);
        Self::from_lcha(lcha)
    }

    #[must_use]
    pub fn darken(self, fraction: f32) -> Self {
        let mut lcha = self.into_lcha();
        lcha.0 = if lcha.0 >= 1.0 {
            0.99
        } else {
            lcha.0 * (1.0 - fraction)
        }
        .clamp(0.0, 1.0);
        Self::from_lcha(lcha)
    }

    #[must_use]
    pub fn saturate(self, fraction: f32) -> Self {
        let mut lcha = self.into_lcha();
        lcha.1 *= 1.0 + fraction;
        lcha.1 = lcha.1.clamp(0.0, 1.0);
        Self::from_lcha(lcha)
    }
}

static BLACK: Rgba = Rgba(RGBA {
    r: 0,
    g: 0,
    b: 0,
    a: 1.0,
});

static WHITE: Rgba = Rgba(RGBA {
    r: 255,
    g: 255,
    b: 255,
    a: 1.0,
});

impl From<(u8, u8, u8, f32)> for Rgba {
    fn from((r, g, b, a): (u8, u8, u8, f32)) -> Self {
        Self(RGBA { r, g, b, a })
    }
}

impl From<u32> for Rgba {
    fn from(rgba: u32) -> Self {
        let [r, g, b, a] = rgba.to_be_bytes();
        Self(RGBA {
            r,
            g,
            b,
            a: a as f32 / 255.0,
        })
    }
}

impl ToString for Rgba {
    fn to_string(&self) -> String {
        macro_rules! format {
            ($($arg:tt)*) => {{
                use ufmt;
                let mut text = String::new();
                ufmt::uwrite!(&mut text, $($arg)*).unwrap();
                text
            }}
        }

        format!(
            "rgba({}, {}, {}, {}%)",
            self.0.r,
            self.0.g,
            self.0.b,
            (self.0.a * 100.0).round() as u8
        )
    }
}
