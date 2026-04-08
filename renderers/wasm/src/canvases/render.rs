//! Canvas rendering utilities for Terra-Deck

use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::JsValue;

/// Draw a rectangle with fill and/or stroke
pub fn draw_rect(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    fill_color: &str,
    stroke_color: &str,
) {
    if fill_color != "none" {
        ctx.set_fill_style(&JsValue::from_str(fill_color));
        ctx.fill_rect(x, y, width, height);
    }
    if stroke_color != "none" {
        ctx.set_stroke_style(&JsValue::from_str(stroke_color));
        ctx.stroke_rect(x, y, width, height);
    }
}

/// Draw text at position
pub fn draw_text(
    ctx: &CanvasRenderingContext2d,
    text: &str,
    x: f64,
    y: f64,
    font_style: &str,
    color: &str,
) {
    ctx.set_font(font_style);
    ctx.set_fill_style(&JsValue::from_str(color));
    let _ = ctx.fill_text(text, x, y);
}

/// Draw text centered at position
pub fn draw_text_centered(
    ctx: &CanvasRenderingContext2d,
    text: &str,
    x: f64,
    y: f64,
    font_style: &str,
    color: &str,
) {
    ctx.set_font(font_style);
    ctx.set_text_align("center");
    ctx.set_fill_style(&JsValue::from_str(color));
    let _ = ctx.fill_text(text, x, y);
    ctx.set_text_align("start");
}
