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