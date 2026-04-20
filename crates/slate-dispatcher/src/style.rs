//! Minimal style-decl parser.
//!
//! Supports the narrow subset the Phase 1 kernel actually understands:
//! `width`, `height`, `background`, `border-width`. The point of this
//! module is to show that the translation from dirty CSS strings to
//! a fully-typed [`ResolvedStyle`] happens *at the bridge*, not in
//! the hot kernel path.

use slate_ais::{Rgba8, Size};

use crate::ir::ResolvedStyle;
use crate::DispatchError;

/// Parse a full `style="..."` declaration into a [`ResolvedStyle`].
/// Unknown properties are silently dropped — they are not erroneous,
/// they are just not part of the AIS surface.
pub fn parse(css: &str) -> Result<ResolvedStyle, DispatchError> {
    let mut out = ResolvedStyle::default();
    let mut width: Option<f32> = None;
    let mut height: Option<f32> = None;

    for decl in css.split(';') {
        let decl = decl.trim();
        if decl.is_empty() {
            continue;
        }
        let (prop, val) = decl
            .split_once(':')
            .ok_or_else(|| DispatchError::BadStyle(decl.to_string()))?;
        let prop = prop.trim();
        let val = val.trim();

        match prop {
            "width"  => width  = Some(parse_px(val)?),
            "height" => height = Some(parse_px(val)?),
            "background" | "background-color" => {
                out.background = Some(parse_color(val)?);
            }
            "border-color" => {
                let c = parse_color(val)?;
                let (_, w) = out.border.unwrap_or((c, 1.0));
                out.border = Some((c, w));
            }
            "border-width" => {
                let w = parse_px(val)?;
                let (c, _) = out.border.unwrap_or((Rgba8::BLACK, w));
                out.border = Some((c, w));
            }
            _ => { /* silently unsupported by the AIS surface */ }
        }
    }

    if width.is_some() || height.is_some() {
        out.size = Some(Size::new(width.unwrap_or(0.0), height.unwrap_or(0.0)));
    }
    Ok(out)
}

fn parse_px(v: &str) -> Result<f32, DispatchError> {
    let stripped = v.strip_suffix("px").unwrap_or(v);
    stripped
        .trim()
        .parse::<f32>()
        .map_err(|_| DispatchError::BadStyle(format!("not a length: {v}")))
}

fn parse_color(v: &str) -> Result<Rgba8, DispatchError> {
    // Named colors — tiny on purpose. The Dispatcher is the place to
    // enumerate them; the kernel only ever sees raw RGBA.
    match v {
        "black"       => return Ok(Rgba8::BLACK),
        "white"       => return Ok(Rgba8::WHITE),
        "red"         => return Ok(Rgba8::rgb(255, 0, 0)),
        "green"       => return Ok(Rgba8::rgb(0, 128, 0)),
        "blue"        => return Ok(Rgba8::rgb(0, 0, 255)),
        "transparent" => return Ok(Rgba8::TRANSPARENT),
        _ => {}
    }
    // #rgb / #rrggbb
    if let Some(hex) = v.strip_prefix('#') {
        return hex_color(hex).ok_or_else(|| DispatchError::BadStyle(v.to_string()));
    }
    Err(DispatchError::BadStyle(format!("unsupported color: {v}")))
}

fn hex_color(hex: &str) -> Option<Rgba8> {
    match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
            Some(Rgba8::rgb(r, g, b))
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Rgba8::rgb(r, g, b))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_size_and_color() {
        let r = parse("width:200px; height:100; background:red").unwrap();
        assert_eq!(r.size, Some(Size::new(200.0, 100.0)));
        assert_eq!(r.background, Some(Rgba8::rgb(255, 0, 0)));
    }

    #[test]
    fn parses_hex() {
        let r = parse("background:#1a2b3c").unwrap();
        assert_eq!(r.background, Some(Rgba8::rgb(0x1a, 0x2b, 0x3c)));
    }

    #[test]
    fn unknown_props_are_dropped_silently() {
        let r = parse("will-change: transform; width: 10").unwrap();
        assert_eq!(r.size, Some(Size::new(10.0, 0.0)));
    }
}
