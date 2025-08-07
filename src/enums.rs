use std::ops::BitOr;

use xege_ffi::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FillPattern {
    Empty = ege_fill_patterns_EMPTY_FILL,
    Solid = ege_fill_patterns_SOLID_FILL,
    Line = ege_fill_patterns_LINE_FILL,
    LtSlash = ege_fill_patterns_LTSLASH_FILL,
    Slash = ege_fill_patterns_SLASH_FILL,
    BkSlash = ege_fill_patterns_BKSLASH_FILL,
    LtBkSlash = ege_fill_patterns_LTBKSLASH_FILL,
    Hatch = ege_fill_patterns_HATCH_FILL,
    XHatch = ege_fill_patterns_XHATCH_FILL,
    Interleave = ege_fill_patterns_INTERLEAVE_FILL,
    WideDot = ege_fill_patterns_WIDE_DOT_FILL,
    CloseDot = ege_fill_patterns_CLOSE_DOT_FILL,
    UserDefined = ege_fill_patterns_USER_FILL,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineStyle {
    Solid,
    Center,
    Dotted,
    Dashed,
    Null,
    UserDef(u16),
}

impl Into<i32> for LineStyle {
    fn into(self) -> i32 {
        match self {
            Self::Solid => ege_line_styles_SOLID_LINE,
            Self::Center => ege_line_styles_CENTER_LINE,
            Self::Dotted => ege_line_styles_DOTTED_LINE,
            Self::Dashed => ege_line_styles_DASHED_LINE,
            Self::Null => ege_line_styles_NULL_LINE,
            Self::UserDef(_) => ege_line_styles_USERBIT_LINE,
        }
    }
}

impl TryFrom<i32> for LineStyle {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            xege_ffi::ege_line_styles_SOLID_LINE => Ok(Self::Solid),
            xege_ffi::ege_line_styles_CENTER_LINE => Ok(Self::Center),
            xege_ffi::ege_line_styles_DOTTED_LINE => Ok(Self::Dotted),
            xege_ffi::ege_line_styles_DASHED_LINE => Ok(Self::Dashed),
            xege_ffi::ege_line_styles_NULL_LINE => Ok(Self::Null),
            xege_ffi::ege_line_styles_USERBIT_LINE => Ok(Self::UserDef(0)),
            _ => Err(()),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineCap {
    Flat = ege_line_cap_type_LINECAP_FLAT,
    Square = ege_line_cap_type_LINECAP_SQUARE,
    Round = ege_line_cap_type_LINECAP_ROUND,
}

impl TryFrom<i32> for LineCap {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            xege_ffi::ege_line_cap_type_LINECAP_FLAT => Ok(Self::Flat),
            xege_ffi::ege_line_cap_type_LINECAP_SQUARE => Ok(Self::Square),
            xege_ffi::ege_line_cap_type_LINECAP_ROUND => Ok(Self::Round),
            _ => Err(()),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineJoin {
    Miter = ege_line_join_type_LINEJOIN_MITER,
    Round = ege_line_join_type_LINEJOIN_ROUND,
    Bevel = ege_line_join_type_LINEJOIN_BEVEL,
}

impl TryFrom<i32> for LineJoin {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            xege_ffi::ege_line_join_type_LINEJOIN_MITER => Ok(Self::Miter),
            xege_ffi::ege_line_join_type_LINEJOIN_ROUND => Ok(Self::Round),
            xege_ffi::ege_line_join_type_LINEJOIN_BEVEL => Ok(Self::Bevel),
            _ => Err(()),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Weight {
    Dontcare = 0,
    Thin = 100,
    Extralight = 200,
    Light = 300,
    #[default]
    Normal = 400,
    Medium = 500,
    Semibold = 600,
    Bold = 700,
    Extrabold = 800,
    Black = 900,
}

impl TryFrom<i32> for Weight {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Dontcare),
            100 => Ok(Self::Thin),
            200 => Ok(Self::Extralight),
            300 => Ok(Self::Light),
            400 => Ok(Self::Normal),
            500 => Ok(Self::Medium),
            600 => Ok(Self::Semibold),
            700 => Ok(Self::Bold),
            800 => Ok(Self::Extrabold),
            900 => Ok(Self::Black),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CharSet {
    Ansi = ANSI_CHARSET,
    Baltic = BALTIC_CHARSET,
    ChineseBig5 = CHINESEBIG5_CHARSET,
    #[default]
    Default = DEFAULT_CHARSET,
    EastEurope = EASTEUROPE_CHARSET,
    GB2312 = GB2312_CHARSET,
    Greek = GREEK_CHARSET,
    Hangul = HANGUL_CHARSET,
    Mac = MAC_CHARSET,
    OEM = OEM_CHARSET,
    Russian = RUSSIAN_CHARSET,
    ShiftJis = SHIFTJIS_CHARSET,
    Symbol = SYMBOL_CHARSET,
    Turkish = TURKISH_CHARSET,
}

impl TryFrom<u32> for CharSet {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            ANSI_CHARSET => Ok(Self::Ansi),
            BALTIC_CHARSET => Ok(Self::Baltic),
            CHINESEBIG5_CHARSET => Ok(Self::ChineseBig5),
            DEFAULT_CHARSET => Ok(Self::Default),
            EASTEUROPE_CHARSET => Ok(Self::EastEurope),
            GB2312_CHARSET => Ok(Self::GB2312),
            GREEK_CHARSET => Ok(Self::Greek),
            HANGUL_CHARSET => Ok(Self::Hangul),
            MAC_CHARSET => Ok(Self::Mac),
            OEM_CHARSET => Ok(Self::OEM),
            RUSSIAN_CHARSET => Ok(Self::Russian),
            SHIFTJIS_CHARSET => Ok(Self::ShiftJis),
            SYMBOL_CHARSET => Ok(Self::Symbol),
            TURKISH_CHARSET => Ok(Self::Turkish),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum OutPrecision {
    #[default]
    Default = OUT_DEFAULT_PRECIS,
    Device = OUT_DEVICE_PRECIS,
    Outline = OUT_OUTLINE_PRECIS,
    Raster = OUT_RASTER_PRECIS,
    String = OUT_STRING_PRECIS,
    Stroke = OUT_STROKE_PRECIS,
    TureTypeOnly = OUT_TT_ONLY_PRECIS,
    TureType = OUT_TT_PRECIS,
}

impl TryFrom<u32> for OutPrecision {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            OUT_DEFAULT_PRECIS => Ok(Self::Default),
            OUT_DEVICE_PRECIS => Ok(Self::Device),
            OUT_OUTLINE_PRECIS => Ok(Self::Outline),
            OUT_RASTER_PRECIS => Ok(Self::Raster),
            OUT_STRING_PRECIS => Ok(Self::String),
            OUT_STROKE_PRECIS => Ok(Self::Stroke),
            OUT_TT_ONLY_PRECIS => Ok(Self::TureTypeOnly),
            OUT_TT_PRECIS => Ok(Self::TureType),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ClipPrecision {
    #[default]
    DefaultPrecis = CLIP_DEFAULT_PRECIS,
    StrokePrecis = CLIP_STROKE_PRECIS,
    Embedded = CLIP_EMBEDDED,
    LhAngles = CLIP_LH_ANGLES,
}

impl TryFrom<u32> for ClipPrecision {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            CLIP_DEFAULT_PRECIS => Ok(Self::DefaultPrecis),
            CLIP_STROKE_PRECIS => Ok(Self::StrokePrecis),
            CLIP_EMBEDDED => Ok(Self::Embedded),
            CLIP_LH_ANGLES => Ok(Self::LhAngles),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Quality {
    Antialiased = ANTIALIASED_QUALITY,
    #[default]
    Default = DEFAULT_QUALITY,
    Draft = DRAFT_QUALITY,
    NonAntialiased = NONANTIALIASED_QUALITY,
    Proof = PROOF_QUALITY,
}

impl TryFrom<u32> for Quality {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            ANTIALIASED_QUALITY => Ok(Self::Antialiased),
            DEFAULT_QUALITY => Ok(Self::Default),
            DRAFT_QUALITY => Ok(Self::Draft),
            NONANTIALIASED_QUALITY => Ok(Self::NonAntialiased),
            PROOF_QUALITY => Ok(Self::Proof),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Pitch {
    #[default]
    Default = DEFAULT_PITCH,
    Fixed = FIXED_PITCH,
    Variable = VARIABLE_PITCH,
}

impl TryFrom<u32> for Pitch {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            DEFAULT_PITCH => Ok(Self::Default),
            FIXED_PITCH => Ok(Self::Fixed),
            VARIABLE_PITCH => Ok(Self::Variable),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontFamily {
    Decorative = FF_DECORATIVE,
    Dontcare = FF_DONTCARE,
    Modern = FF_MODERN,
    Roman = FF_ROMAN,
    Script = FF_SCRIPT,
    Swiss = FF_SWISS,
}

impl TryFrom<u32> for FontFamily {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            FF_DECORATIVE => Ok(Self::Decorative),
            FF_DONTCARE => Ok(Self::Dontcare),
            FF_MODERN => Ok(Self::Modern),
            FF_ROMAN => Ok(Self::Roman),
            FF_SCRIPT => Ok(Self::Script),
            FF_SWISS => Ok(Self::Swiss),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PitchAndFamily {
    pub(crate) mask: u32,
}

impl Default for PitchAndFamily {
    fn default() -> Self {
        PitchAndFamily {
            mask: DEFAULT_PITCH | (FF_DONTCARE as u32),
        }
    }
}

impl BitOr<Pitch> for FontFamily {
    type Output = PitchAndFamily;

    fn bitor(self, rhs: Pitch) -> Self::Output {
        PitchAndFamily {
            mask: (self as u32) | (rhs as u32),
        }
    }
}

impl BitOr<FontFamily> for Pitch {
    type Output = PitchAndFamily;

    fn bitor(self, rhs: FontFamily) -> Self::Output {
        rhs | self
    }
}

impl TryFrom<u32> for PitchAndFamily {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let pitch = Pitch::try_from(value & 0x00000003)?;
        let family = FontFamily::try_from((value & 0x000000F0) >> 4)?;
        Ok(PitchAndFamily {
            mask: (pitch as u32) | ((family as u32) << 4),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Font {
    pub height: i32,
    pub width: i32,
    pub escapement: i32,
    pub orientation: i32,
    pub weight: Weight,
    pub italic: bool,
    pub underline: bool,
    pub strikeout: bool,
    pub charset: CharSet,
    pub outprecision: OutPrecision,
    pub clipprecision: ClipPrecision,
    pub quality: Quality,
    pub pitchandfamily: PitchAndFamily,
    pub facename: String,
}

impl Into<LOGFONTW> for Font {
    fn into(self) -> LOGFONTW {
        let mut buff = [0u16; 32];
        let facename = self.facename.encode_utf16().collect::<Vec<u16>>();
        for (i, &c) in facename.iter().enumerate() {
            buff[i] = c;
        }
        LOGFONTW {
            lfHeight: self.height,
            lfWidth: self.width,
            lfEscapement: self.escapement,
            lfOrientation: self.orientation,
            lfWeight: self.weight as i32,
            lfItalic: self.italic as u8,
            lfUnderline: self.underline as u8,
            lfStrikeOut: self.strikeout as u8,
            lfCharSet: self.charset as u8,
            lfOutPrecision: self.outprecision as u8,
            lfClipPrecision: self.clipprecision as u8,
            lfQuality: self.quality as u8,
            lfPitchAndFamily: self.pitchandfamily.mask as u8,
            lfFaceName: buff,
        }
    }
}

impl TryFrom<LOGFONTW> for Font {
    type Error = ();

    fn try_from(value: LOGFONTW) -> Result<Self, Self::Error> {
        let facename = String::from_utf16(&value.lfFaceName).map_err(|_| ())?;
        Ok(Font {
            height: value.lfHeight,
            width: value.lfWidth,
            escapement: value.lfEscapement,
            orientation: value.lfOrientation,
            weight: Weight::try_from(value.lfWeight)?,
            italic: value.lfItalic != 0,
            underline: value.lfUnderline != 0,
            strikeout: value.lfStrikeOut != 0,
            charset: CharSet::try_from(value.lfCharSet as u32)?,
            outprecision: OutPrecision::try_from(value.lfOutPrecision as u32)?,
            clipprecision: ClipPrecision::try_from(value.lfClipPrecision as u32)?,
            quality: Quality::try_from(value.lfQuality as u32)?,
            pitchandfamily: PitchAndFamily::try_from(value.lfPitchAndFamily as u32)?,
            facename,
        })
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TextHAlign {
    #[default]
    Left = ege_text_just_LEFT_TEXT,
    Center = ege_text_just_CENTER_TEXT,
    Right = ege_text_just_RIGHT_TEXT,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TextVAlign {
    #[default]
    Top = ege_text_just_TOP_TEXT,
    Center = ege_text_just_CENTER_TEXT,
    Bottom = ege_text_just_BOTTOM_TEXT,
}
