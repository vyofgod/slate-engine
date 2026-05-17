//! # Slate CSS Engine
//!
//! Full CSS3 selector matching, cascade, specificity, and computed styles.
//! Zero-allocation fast path for inline styles.

pub mod cascade;
pub mod parser;
pub mod selector;
pub mod values;
pub mod cssparser_integration;

pub use cascade::{CascadeEngine, ComputedStyle};
pub use parser::CssParser;
pub use selector::{Selector, SelectorMatcher, Specificity};
pub use values::{CssValue, Display, Position, Unit};
pub use cssparser_integration::CssParserReal;

/// A CSS rule: selector + declarations.
#[derive(Debug, Clone)]
pub struct CssRule {
    pub selector: Selector,
    pub declarations: Vec<Declaration>,
    pub specificity: Specificity,
}

/// A CSS declaration: property + value.
#[derive(Debug, Clone)]
pub struct Declaration {
    pub property: Property,
    pub value: CssValue,
    pub important: bool,
}

/// CSS property types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Property {
    // Box model
    Width,
    Height,
    MinWidth,
    MinHeight,
    MaxWidth,
    MaxHeight,
    Margin,
    MarginTop,
    MarginRight,
    MarginBottom,
    MarginLeft,
    Padding,
    PaddingTop,
    PaddingRight,
    PaddingBottom,
    PaddingLeft,
    Border,
    BorderWidth,
    BorderColor,
    BorderStyle,

    // Layout
    Display,
    Position,
    Top,
    Right,
    Bottom,
    Left,
    ZIndex,
    Float,
    Clear,
    Overflow,
    OverflowX,
    OverflowY,

    // Flexbox
    FlexDirection,
    FlexWrap,
    JustifyContent,
    AlignItems,
    AlignContent,
    Flex,
    FlexGrow,
    FlexShrink,
    FlexBasis,
    Order,

    // Grid
    GridTemplateColumns,
    GridTemplateRows,
    GridTemplateAreas,
    GridAutoColumns,
    GridAutoRows,
    GridAutoFlow,
    GridColumn,
    GridRow,
    GridArea,
    Gap,
    RowGap,
    ColumnGap,

    // Visual
    Color,
    BackgroundColor,
    Background,
    Opacity,
    Visibility,

    // Text
    FontFamily,
    FontSize,
    FontWeight,
    FontStyle,
    LineHeight,
    TextAlign,
    TextDecoration,
    TextTransform,
    LetterSpacing,
    WordSpacing,
    WhiteSpace,
    WordBreak,

    // Transform
    Transform,
    TransformOrigin,
    Rotate,
    Scale,
    Translate,

    // Animation
    Transition,
    TransitionProperty,
    TransitionDuration,
    TransitionTimingFunction,
    TransitionDelay,
    Animation,
    AnimationName,
    AnimationDuration,
    AnimationTimingFunction,
    AnimationDelay,
    AnimationIterationCount,
    AnimationDirection,
    AnimationFillMode,

    // Other
    Cursor,
    PointerEvents,
    UserSelect,

    Unknown(u32), // Hash of unknown property name
}

impl Property {
    /// Parse a property name.
    pub fn from_name(name: &str) -> Self {
        match name {
            "width" => Property::Width,
            "height" => Property::Height,
            "min-width" => Property::MinWidth,
            "min-height" => Property::MinHeight,
            "max-width" => Property::MaxWidth,
            "max-height" => Property::MaxHeight,
            "margin" => Property::Margin,
            "margin-top" => Property::MarginTop,
            "margin-right" => Property::MarginRight,
            "margin-bottom" => Property::MarginBottom,
            "margin-left" => Property::MarginLeft,
            "padding" => Property::Padding,
            "padding-top" => Property::PaddingTop,
            "padding-right" => Property::PaddingRight,
            "padding-bottom" => Property::PaddingBottom,
            "padding-left" => Property::PaddingLeft,
            "display" => Property::Display,
            "position" => Property::Position,
            "color" => Property::Color,
            "background-color" => Property::BackgroundColor,
            "font-size" => Property::FontSize,
            "font-weight" => Property::FontWeight,
            "font-family" => Property::FontFamily,
            "text-align" => Property::TextAlign,
            "flex-direction" => Property::FlexDirection,
            "justify-content" => Property::JustifyContent,
            "align-items" => Property::AlignItems,
            _ => {
                // Hash unknown properties for fast comparison
                let hash = name.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
                Property::Unknown(hash)
            }
        }
    }

    /// Check if this property is inherited.
    pub fn is_inherited(&self) -> bool {
        matches!(
            self,
            Property::Color
                | Property::FontFamily
                | Property::FontSize
                | Property::FontWeight
                | Property::FontStyle
                | Property::LineHeight
                | Property::TextAlign
                | Property::TextTransform
                | Property::LetterSpacing
                | Property::WordSpacing
                | Property::WhiteSpace
                | Property::WordBreak
                | Property::Cursor
        )
    }
}

/// A stylesheet containing multiple rules.
#[derive(Debug, Clone)]
pub struct Stylesheet {
    pub rules: Vec<CssRule>,
}

impl Stylesheet {
    /// Create an empty stylesheet.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add a rule to the stylesheet.
    pub fn add_rule(&mut self, rule: CssRule) {
        self.rules.push(rule);
    }
}

impl Default for Stylesheet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn property_inheritance() {
        assert!(Property::Color.is_inherited());
        assert!(Property::FontSize.is_inherited());
        assert!(!Property::Width.is_inherited());
        assert!(!Property::Display.is_inherited());
    }
}
