use core::fmt::{Display, Formatter};
use wgpu_glyph::ab_glyph::FontArc;

#[derive(Debug, Copy, Clone)]
pub enum FontStyle {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

impl Display for FontStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            FontStyle::Regular => write!(f, "Regular"),
            FontStyle::Bold => {
                write!(f, "Bold")
            }
            FontStyle::Italic => {
                write!(f, "Italic")
            }
            FontStyle::BoldItalic => {
                write!(f, "BoldItalic")
            }
        }
    }
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

impl Display for FontFamily {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            FontFamily::SansSerif => {
                write!(f, "Sans")
            }
            FontFamily::Serif => {
                write!(f, "Serif")
            }
            FontFamily::Monospace => {
                write!(f, "Mono")
            }
        }
    }
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
