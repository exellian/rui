use wgpu_glyph::ab_glyph::FontArc;

#[derive(Debug, Copy, Clone)]
pub enum FontStyle {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

pub struct FallbackFonts {}

#[derive(RustEmbed)]
#[folder = "assets/liberation-fonts-ttf-2.1.5/"]
#[include = "*.ttf"]
struct FallbackFontArchive;

#[derive(Debug, Copy, Clone)]
pub enum FontFamily {
    SansSerif,
    Serif,
    Monospace,
}

impl FallbackFonts {
    const fn get_font_name(family: FontFamily, style: FontStyle) -> &'static str {
        match family {
            FontFamily::SansSerif => match style {
                FontStyle::Regular => "LiberationSans-Regular.ttf",
                FontStyle::Bold => "LiberationSans-Bold.ttf",
                FontStyle::Italic => "LiberationSans-Italic.ttf",
                FontStyle::BoldItalic => "LiberationSans-BoldItalic.ttf",
            },
            FontFamily::Serif => match style {
                FontStyle::Regular => "LiberationSerif-Regular.ttf",
                FontStyle::Bold => "LiberationSerif-Bold.ttf",
                FontStyle::Italic => "LiberationSerif-Italic.ttf",
                FontStyle::BoldItalic => "LiberationSerif-BoldItalic.ttf",
            },
            FontFamily::Monospace => match style {
                FontStyle::Regular => "LiberationMono-Regular.ttf",
                FontStyle::Bold => "LiberationMono-Bold.ttf",
                FontStyle::Italic => "LiberationMono-Italic.ttf",
                FontStyle::BoldItalic => "LiberationMono-BoldItalic.ttf",
            },
        }
    }

    pub fn get_font(family: FontFamily, style: FontStyle) -> FontArc {
        let data = Vec::from(
            FallbackFontArchive::get(Self::get_font_name(family, style))
                .unwrap()
                .data,
        );
        FontArc::try_from_vec(data).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::font::fallback_fonts::{FallbackFonts, FontFamily, FontStyle};

    #[test]
    fn test_that_all_fonts_are_loadable() {
        let _ = FallbackFonts::get_font(FontFamily::SansSerif, FontStyle::Regular);
        let _ = FallbackFonts::get_font(FontFamily::SansSerif, FontStyle::Italic);
        let _ = FallbackFonts::get_font(FontFamily::SansSerif, FontStyle::Bold);
        let _ = FallbackFonts::get_font(FontFamily::SansSerif, FontStyle::BoldItalic);

        let _ = FallbackFonts::get_font(FontFamily::Serif, FontStyle::Regular);
        let _ = FallbackFonts::get_font(FontFamily::Serif, FontStyle::Italic);
        let _ = FallbackFonts::get_font(FontFamily::Serif, FontStyle::Bold);
        let _ = FallbackFonts::get_font(FontFamily::Serif, FontStyle::BoldItalic);

        let _ = FallbackFonts::get_font(FontFamily::Monospace, FontStyle::Regular);
        let _ = FallbackFonts::get_font(FontFamily::Monospace, FontStyle::Italic);
        let _ = FallbackFonts::get_font(FontFamily::Monospace, FontStyle::Bold);
        let _ = FallbackFonts::get_font(FontFamily::Monospace, FontStyle::BoldItalic);

        assert_eq!(1, 1);
    }
}
