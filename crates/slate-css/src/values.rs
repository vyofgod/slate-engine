//! CSS value types and parsing.

use slate_ais::{Rgba8, SubPixel};

/// A CSS value.
#[derive(Debug, Clone, PartialEq)]
pub enum CssValue {
    /// Length with unit
    Length(SubPixel, Unit),

    /// Percentage
    Percentage(f32),

    /// Color
    Color(Rgba8),

    /// Keyword
    Keyword(Keyword),

    /// Number
    Number(f32),

    /// String
    String(String),

    /// Multiple values
    List(Vec<CssValue>),

    /// Initial value
    Initial,

    /// Inherit from parent
    Inherit,

    /// Unset (inherit if inherited, initial otherwise)
    Unset,
}

/// CSS length units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    /// Pixels
    Px,

    /// Ems (relative to font size)
    Em,

    /// Rems (relative to root font size)
    Rem,

    /// Viewport width percentage
    Vw,

    /// Viewport height percentage
    Vh,

    /// Viewport minimum
    Vmin,

    /// Viewport maximum
    Vmax,

    /// Points (1/72 inch)
    Pt,

    /// Picas (12 points)
    Pc,

    /// Inches
    In,

    /// Centimeters
    Cm,

    /// Millimeters
    Mm,

    /// Percentage of parent
    Percent,
}

/// CSS keywords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    Auto,
    None,
    Normal,
    Inherit,
    Initial,
    Unset,
}

/// Display property values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    None,
    Block,
    Inline,
    InlineBlock,
    Flex,
    InlineFlex,
    Grid,
    InlineGrid,
    Table,
    TableRow,
    TableCell,
    ListItem,
    Contents,
}

/// Position property values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

/// Float property values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Float {
    None,
    Left,
    Right,
}

/// Overflow property values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
    Auto,
}

/// Flexbox direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

/// Flexbox justify-content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Flexbox align-items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl CssValue {
    /// Parse a color from a hex string.
    pub fn parse_color(s: &str) -> Option<Self> {
        if let Some(hex) = s.strip_prefix('#') {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = if hex.len() == 8 {
                u8::from_str_radix(&hex[6..8], 16).ok()?
            } else {
                255
            };
            Some(CssValue::Color(Rgba8 { r, g, b, a }))
        } else {
            None
        }
    }

    /// Parse a length value.
    pub fn parse_length(s: &str) -> Option<Self> {
        if let Some(px) = s.strip_suffix("px") {
            let value: f32 = px.parse().ok()?;
            Some(CssValue::Length(SubPixel(value), Unit::Px))
        } else if let Some(em) = s.strip_suffix("em") {
            let value: f32 = em.parse().ok()?;
            Some(CssValue::Length(SubPixel(value), Unit::Em))
        } else if let Some(rem) = s.strip_suffix("rem") {
            let value: f32 = rem.parse().ok()?;
            Some(CssValue::Length(SubPixel(value), Unit::Rem))
        } else if let Some(pct) = s.strip_suffix('%') {
            let value: f32 = pct.parse().ok()?;
            Some(CssValue::Percentage(value))
        } else {
            None
        }
    }
}
