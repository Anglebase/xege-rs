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
        let facename = self
            .facename
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>();
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

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AlphaType {
    #[default]
    Straight = ege_alpha_type_ALPHATYPE_STRAIGHT,
    PreMultiplied = ege_alpha_type_ALPHATYPE_PREMULTIPLIED,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// A
    A,
    /// B
    B,
    /// C
    C,
    /// D
    D,
    /// E
    E,
    /// F
    F,
    /// G
    G,
    /// H
    H,
    /// I
    I,
    /// J
    J,
    /// K
    K,
    /// L
    L,
    /// M
    M,
    /// N
    N,
    /// O
    O,
    /// P
    P,
    /// Q
    Q,
    /// R
    R,
    /// S
    S,
    /// T
    T,
    /// U
    U,
    /// V
    V,
    /// W
    W,
    /// X
    X,
    /// Y
    Y,
    /// Z
    Z,

    /// 1
    Num1,
    /// 2
    Num2,
    /// 3
    Num3,
    /// 4
    Num4,
    /// 5
    Num5,
    /// 6
    Num6,
    /// 7
    Num7,
    /// 8
    Num8,
    /// 9
    Num9,
    /// 0
    Num0,

    /// NumPad 1
    NumPad1,
    /// NumPad 2
    NumPad2,
    /// NumPad 3
    NumPad3,
    /// NumPad 4
    NumPad4,
    /// NumPad 5
    NumPad5,
    /// NumPad 6
    NumPad6,
    /// NumPad 7
    NumPad7,
    /// NumPad 8
    NumPad8,
    /// NumPad 9
    NumPad9,
    /// NumPad 0
    NumPad0,

    /// F1
    F1,
    /// F2
    F2,
    /// F3
    F3,
    /// F4
    F4,
    /// F5
    F5,
    /// F6
    F6,
    /// F7
    F7,
    /// F8
    F8,
    /// F9
    F9,
    /// F10
    F10,
    /// F11
    F11,
    /// F12
    F12,

    /// Shift
    Shift,
    /// Ctrl
    Ctrl,
    /// Alt
    Alt,

    // Special symbols for American keyboards
    /// `
    Backtick,
    /// ,
    Comma,
    /// .
    Dot,
    /// /
    Slash,
    /// ;
    Semicolon,
    /// '
    Quote,
    /// [
    LeftBracket,
    /// ]
    RightBracket,
    /// \
    Backslash,
    /// -
    Minus,
    /// =
    Equals,

    /// NumPad +
    NumAdd,
    /// NumPad -
    NumSub,
    /// NumPad *
    NumMul,
    /// NumPad /
    NumDiv,
    /// NumPad .
    NumDot,

    /// Tab
    Tab,
    /// Space
    Space,
    /// Enter
    Enter,
    /// Backspace
    Backspace,

    /// Esc
    Esc,
    /// CapsLock
    CapsLock,
    /// Left Ctrl
    LeftCtrl,
    /// Left Shift
    LeftShift,
    /// Left Alt
    LeftAlt,
    /// Right Ctrl
    RightCtrl,
    /// Right Shift
    RightShift,
    /// Right Alt
    RightAlt,
    /// ScrollLock
    ScrollLock,
    /// NumLock
    NumLock,
    /// Delete
    Delete,
    /// Insert
    Insert,
    /// Home
    Home,
    /// End
    End,
    /// PageUp
    PageUp,
    /// PageDown
    PageDown,
    /// Clear
    Clear,

    /// Left mouse button
    LeftButton,
    /// Right mouse button
    RightButton,
    /// Middle mouse button
    MiddleButton,
    /// Mouse extension button 1
    X1Button,
    /// Mouse extension button 2
    X2Button,

    /// Left arrow
    Left,
    /// Right arrow
    Right,
    /// Up arrow
    Up,
    /// Down arrow
    Down,

    /// Other key
    Other(u32),
}

impl PartialEq<char> for Key {
    #[inline(never)]
    fn eq(&self, other: &char) -> bool {
        match (self, other) {
            // 字母键 (A-Z)
            (Key::A, 'a' | 'A') => true,
            (Key::B, 'b' | 'B') => true,
            (Key::C, 'c' | 'C') => true,
            (Key::D, 'd' | 'D') => true,
            (Key::E, 'e' | 'E') => true,
            (Key::F, 'f' | 'F') => true,
            (Key::G, 'g' | 'G') => true,
            (Key::H, 'h' | 'H') => true,
            (Key::I, 'i' | 'I') => true,
            (Key::J, 'j' | 'J') => true,
            (Key::K, 'k' | 'K') => true,
            (Key::L, 'l' | 'L') => true,
            (Key::M, 'm' | 'M') => true,
            (Key::N, 'n' | 'N') => true,
            (Key::O, 'o' | 'O') => true,
            (Key::P, 'p' | 'P') => true,
            (Key::Q, 'q' | 'Q') => true,
            (Key::R, 'r' | 'R') => true,
            (Key::S, 's' | 'S') => true,
            (Key::T, 't' | 'T') => true,
            (Key::U, 'u' | 'U') => true,
            (Key::V, 'v' | 'V') => true,
            (Key::W, 'w' | 'W') => true,
            (Key::X, 'x' | 'X') => true,
            (Key::Y, 'y' | 'Y') => true,
            (Key::Z, 'z' | 'Z') => true,

            // 数字键 (0-9)
            (Key::Num0 | Key::NumPad0, '0') => true,
            (Key::Num1 | Key::NumPad1, '1') => true,
            (Key::Num2 | Key::NumPad2, '2') => true,
            (Key::Num3 | Key::NumPad3, '3') => true,
            (Key::Num4 | Key::NumPad4, '4') => true,
            (Key::Num5 | Key::NumPad5, '5') => true,
            (Key::Num6 | Key::NumPad6, '6') => true,
            (Key::Num7 | Key::NumPad7, '7') => true,
            (Key::Num8 | Key::NumPad8, '8') => true,
            (Key::Num9 | Key::NumPad9, '9') => true,

            // Shift + 数字键
            (Key::Num0, ')') => true,
            (Key::Num1, '!') => true,
            (Key::Num2, '@') => true,
            (Key::Num3, '#') => true,
            (Key::Num4, '$') => true,
            (Key::Num5, '%') => true,
            (Key::Num6, '^') => true,
            (Key::Num7, '&') => true,
            (Key::Num8, '*') => true,
            (Key::Num9, '(') => true,

            // 符号键
            (Key::Backtick, '`' | '~') => true,
            (Key::Comma, ',' | '<') => true,
            (Key::Dot, '.' | '>') => true,
            (Key::Slash, '/' | '?') => true,
            (Key::Semicolon, ';' | ':') => true,
            (Key::Quote, '\'' | '"') => true,
            (Key::LeftBracket, '[' | '{') => true,
            (Key::RightBracket, ']' | '}') => true,
            (Key::Backslash, '\\' | '|') => true,
            (Key::Minus, '-' | '_') => true,
            (Key::Equals, '=' | '+') => true,

            // 小键盘符号
            (Key::NumAdd, '+') => true,
            (Key::NumSub, '-') => true,
            (Key::NumMul, '*') => true,
            (Key::NumDiv, '/') => true,
            (Key::NumDot, '.') => true,

            // 控制字符
            (Key::Space, ' ') => true,
            (Key::Tab, '\t') => true,
            (Key::Enter, '\n' | '\r') => true, // 同时支持 LF 和 CR
            (Key::Backspace, '\x08') => true,  // ASCII 退格符

            _ => false,
        }
    }
}

const fn vk_to_key(vk: u32) -> Key {
    match vk {
        0x30 => Key::Num0,
        0x31 => Key::Num1,
        0x32 => Key::Num2,
        0x33 => Key::Num3,
        0x34 => Key::Num4,
        0x35 => Key::Num5,
        0x36 => Key::Num6,
        0x37 => Key::Num7,
        0x38 => Key::Num8,
        0x39 => Key::Num9,

        0x41 => Key::A,
        0x42 => Key::B,
        0x43 => Key::C,
        0x44 => Key::D,
        0x45 => Key::E,
        0x46 => Key::F,
        0x47 => Key::G,
        0x48 => Key::H,
        0x49 => Key::I,
        0x4A => Key::J,
        0x4B => Key::K,
        0x4C => Key::L,
        0x4D => Key::M,
        0x4E => Key::N,
        0x4F => Key::O,
        0x50 => Key::P,
        0x51 => Key::Q,
        0x52 => Key::R,
        0x53 => Key::S,
        0x54 => Key::T,
        0x55 => Key::U,
        0x56 => Key::V,
        0x57 => Key::W,
        0x58 => Key::X,
        0x59 => Key::Y,
        0x5A => Key::Z,

        VK_F1 => Key::F1,
        VK_F2 => Key::F2,
        VK_F3 => Key::F3,
        VK_F4 => Key::F4,
        VK_F5 => Key::F5,
        VK_F6 => Key::F6,
        VK_F7 => Key::F7,
        VK_F8 => Key::F8,
        VK_F9 => Key::F9,
        VK_F10 => Key::F10,
        VK_F11 => Key::F11,
        VK_F12 => Key::F12,

        VK_NUMPAD0 => Key::NumPad0,
        VK_NUMPAD1 => Key::NumPad1,
        VK_NUMPAD2 => Key::NumPad2,
        VK_NUMPAD3 => Key::NumPad3,
        VK_NUMPAD4 => Key::NumPad4,
        VK_NUMPAD5 => Key::NumPad5,
        VK_NUMPAD6 => Key::NumPad6,
        VK_NUMPAD7 => Key::NumPad7,
        VK_NUMPAD8 => Key::NumPad8,
        VK_NUMPAD9 => Key::NumPad9,

        VK_SHIFT => Key::Shift,
        VK_CONTROL => Key::Ctrl,
        VK_MENU => Key::Alt,

        VK_OEM_1 => Key::Semicolon,
        VK_OEM_2 => Key::Slash,
        VK_OEM_3 => Key::Backtick,
        VK_OEM_4 => Key::LeftBracket,
        VK_OEM_5 => Key::Backslash,
        VK_OEM_6 => Key::RightBracket,
        VK_OEM_PLUS => Key::Equals,
        VK_OEM_COMMA => Key::Comma,
        VK_OEM_MINUS => Key::Minus,
        VK_OEM_PERIOD => Key::Dot,

        VK_ADD => Key::NumAdd,
        VK_SUBTRACT => Key::NumSub,
        VK_MULTIPLY => Key::NumMul,
        VK_DIVIDE => Key::NumDiv,
        VK_DECIMAL => Key::NumDot,

        VK_BACK => Key::Backspace,
        VK_TAB => Key::Tab,
        VK_RETURN => Key::Enter,
        VK_SPACE => Key::Space,

        VK_ESCAPE => Key::Esc,
        VK_CAPITAL => Key::CapsLock,
        VK_LCONTROL => Key::LeftCtrl,
        VK_LSHIFT => Key::LeftShift,
        VK_LMENU => Key::LeftAlt,
        VK_RCONTROL => Key::RightCtrl,
        VK_RSHIFT => Key::RightShift,
        VK_RMENU => Key::RightAlt,
        VK_SCROLL => Key::ScrollLock,
        VK_NUMLOCK => Key::NumLock,
        VK_DELETE => Key::Delete,
        VK_INSERT => Key::Insert,
        VK_HOME => Key::Home,
        VK_END => Key::End,
        VK_PRIOR => Key::PageUp,
        VK_NEXT => Key::PageDown,
        VK_CLEAR => Key::Clear,

        VK_LBUTTON => Key::LeftButton,
        VK_RBUTTON => Key::RightButton,
        VK_MBUTTON => Key::MiddleButton,
        VK_XBUTTON1 => Key::X1Button,
        VK_XBUTTON2 => Key::X2Button,

        VK_LEFT => Key::Left,
        VK_UP => Key::Up,
        VK_RIGHT => Key::Right,
        VK_DOWN => Key::Down,

        _ => Key::Other(vk),
    }
}

const fn key_to_vk(key: Key) -> u32 {
    match key {
        Key::Num0 => 0x30,
        Key::Num1 => 0x31,
        Key::Num2 => 0x32,
        Key::Num3 => 0x33,
        Key::Num4 => 0x34,
        Key::Num5 => 0x35,
        Key::Num6 => 0x36,
        Key::Num7 => 0x37,
        Key::Num8 => 0x38,
        Key::Num9 => 0x39,
        Key::A => 0x41,
        Key::B => 0x42,
        Key::C => 0x43,
        Key::D => 0x44,
        Key::E => 0x45,
        Key::F => 0x46,
        Key::G => 0x47,
        Key::H => 0x48,
        Key::I => 0x49,
        Key::J => 0x4A,
        Key::K => 0x4B,
        Key::L => 0x4C,
        Key::M => 0x4D,
        Key::N => 0x4E,
        Key::O => 0x4F,
        Key::P => 0x50,
        Key::Q => 0x51,
        Key::R => 0x52,
        Key::S => 0x53,
        Key::T => 0x54,
        Key::U => 0x55,
        Key::V => 0x56,
        Key::W => 0x57,
        Key::X => 0x58,
        Key::Y => 0x59,
        Key::Z => 0x5A,
        Key::F1 => VK_F1,
        Key::F2 => VK_F2,
        Key::F3 => VK_F3,
        Key::F4 => VK_F4,
        Key::F5 => VK_F5,
        Key::F6 => VK_F6,
        Key::F7 => VK_F7,
        Key::F8 => VK_F8,
        Key::F9 => VK_F9,
        Key::F10 => VK_F10,
        Key::F11 => VK_F11,
        Key::F12 => VK_F12,
        Key::NumPad0 => VK_NUMPAD0,
        Key::NumPad1 => VK_NUMPAD1,
        Key::NumPad2 => VK_NUMPAD2,
        Key::NumPad3 => VK_NUMPAD3,
        Key::NumPad4 => VK_NUMPAD4,
        Key::NumPad5 => VK_NUMPAD5,
        Key::NumPad6 => VK_NUMPAD6,
        Key::NumPad7 => VK_NUMPAD7,
        Key::NumPad8 => VK_NUMPAD8,
        Key::NumPad9 => VK_NUMPAD9,
        Key::Shift => VK_SHIFT,
        Key::Ctrl => VK_CONTROL,
        Key::Alt => VK_MENU,
        Key::Semicolon => VK_OEM_1,
        Key::Slash => VK_OEM_2,
        Key::Backtick => VK_OEM_3,
        Key::LeftBracket => VK_OEM_4,
        Key::Backslash => VK_OEM_5,
        Key::RightBracket => VK_OEM_6,
        Key::Equals => VK_OEM_PLUS,
        Key::Comma => VK_OEM_COMMA,
        Key::Minus => VK_OEM_MINUS,
        Key::Dot => VK_OEM_PERIOD,
        Key::NumAdd => VK_ADD,
        Key::NumSub => VK_SUBTRACT,
        Key::NumMul => VK_MULTIPLY,
        Key::NumDiv => VK_DIVIDE,
        Key::NumDot => VK_DECIMAL,
        Key::Backspace => VK_BACK,
        Key::Tab => VK_TAB,
        Key::Enter => VK_RETURN,
        Key::Space => VK_SPACE,
        Key::Esc => VK_ESCAPE,
        Key::CapsLock => VK_CAPITAL,
        Key::LeftCtrl => VK_LCONTROL,
        Key::LeftShift => VK_LSHIFT,
        Key::LeftAlt => VK_LMENU,
        Key::RightCtrl => VK_RCONTROL,
        Key::RightShift => VK_RSHIFT,
        Key::RightAlt => VK_RMENU,
        Key::ScrollLock => VK_SCROLL,
        Key::NumLock => VK_NUMLOCK,
        Key::Delete => VK_DELETE,
        Key::Insert => VK_INSERT,
        Key::Home => VK_HOME,
        Key::End => VK_END,
        Key::PageUp => VK_PRIOR,
        Key::PageDown => VK_NEXT,
        Key::Clear => VK_CLEAR,
        Key::LeftButton => VK_LBUTTON,
        Key::RightButton => VK_RBUTTON,
        Key::MiddleButton => VK_MBUTTON,
        Key::X1Button => VK_XBUTTON1,
        Key::X2Button => VK_XBUTTON2,
        Key::Left => VK_LEFT,
        Key::Up => VK_UP,
        Key::Right => VK_RIGHT,
        Key::Down => VK_DOWN,
        Key::Quote => VK_OEM_1,
        Key::Other(vk) => vk,
    }
}

impl From<u32> for Key {
    fn from(value: u32) -> Self {
        vk_to_key(value)
    }
}

impl Into<u32> for Key {
    fn into(self) -> u32 {
        key_to_vk(self)
    }
}
