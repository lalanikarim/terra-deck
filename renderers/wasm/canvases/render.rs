/// Canvas rendering utilities for Terra-Deck

use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

/// Draw a rectangle with optional stroke
pub fn draw_rectangle_with_style(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    fill_color: Option<&str>,
    stroke_color: Option<&str>,
    stroke_width: f64,
) -> Result<(), JsValue> {
    if let Some(color) = fill_color {
        ctx.set_fill_style_with_string(color)?;
        ctx.fill_rect(x, y, width, height);
    }

    if let Some(color) = stroke_color {
        ctx.set_stroke_style_with_string(color)?;
        ctx.set_line_width(stroke_width);
        ctx.stroke_rect(x, y, width, height);
    }

    Ok(())
}

/// Draw a circle with optional stroke
pub fn draw_circle_with_style(
    ctx: &CanvasRenderingContext2d,
    cx: f64,
    cy: f64,
    radius: f64,
    fill_color: Option<&str>,
    stroke_color: Option<&str>,
    stroke_width: f64,
) -> Result<(), JsValue> {
    ctx.begin_path();
    ctx.arc(cx, cy, radius, 0.0, std::f64::consts::PI * 2.0)?;

    if let Some(color) = fill_color {
        ctx.set_fill_style_with_string(color)?;
        ctx.fill();
    }

    if let Some(color) = stroke_color {
        ctx.stroke();
    }

    Ok(())
}

/// Draw a line with style
pub fn draw_line_with_style(
    ctx: &CanvasRenderingContext2d,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    color: &str,
    width: f64,
) -> Result<(), JsValue> {
    ctx.set_stroke_style_with_string(color)?;
    ctx.set_line_width(width);
    ctx.begin_path();
    ctx.line_to(x1, y1)?;
    ctx.line_to(x2, y2)?;
    ctx.stroke();
    
    Ok(())
}

/// Draw text with shadow
pub fn draw_text_with_shadow(
    ctx: &CanvasRenderingContext2d,
    text: &str,
    x: f64,
    y: f64,
    font: &str,
    color: &str,
    shadow_color: &str,
    shadow_blur: f64,
) -> Result<(), JsValue> {
    ctx.set_font(font);
    ctx.set_fill_style_with_string(color)?;
    ctx.set_shadow_color_with_str(shadow_color)?;
    ctx.set_shadow_blur(shadow_blur);
    ctx.fill_text(text, x, y)?;
    ctx.set_shadow_blur(0.0); // Reset shadow
    
    Ok(())
}

/// Draw a sprite (image) at position
pub fn draw_sprite(
    ctx: &CanvasRenderingContext2d,
    image: &web_sys::HtmlImageElement,
    dest_x: f64,
    dest_y: f64,
    dest_width: f64,
    dest_height: f64,
) -> Result<(), JsValue> {
    ctx.draw_image_with_html_image_element_and_sw_and_sh(
        image,
        0.0,
        0.0,
        dest_width,
        dest_height,
        dest_x,
        dest_y,
        dest_width,
        dest_height,
    )?;
    
    Ok(())
}

/// Create a gradient
pub fn create_linear_gradient(
    ctx: &CanvasRenderingContext2d,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
) -> Result<Option<web_sys::CanvasGradient>, JsValue> {
    let gradient = ctx.create_linear_gradient(x0, y0, x1, y1)?;
    Ok(Some(gradient))
}

/// Apply gradient to rectangle
pub fn fill_with_gradient(
    ctx: &CanvasRenderingContext2d,
    gradient: &web_sys::CanvasGradient,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> Result<(), JsValue> {
    ctx.set_fill_style(&gradient.as_ref().unchecked_into::<JsValue>());
    ctx.fill_rect(x, y, width, height);
    Ok(())
}

/// Save canvas state
pub fn save_context(ctx: &CanvasRenderingContext2d) {
    ctx.save();
}

/// Restore canvas state
pub fn restore_context(ctx: &CanvasRenderingContext2d) {
    ctx.restore();
}

/// Translate context
pub fn translate_context(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
) -> Result<(), JsValue> {
    ctx.translate(x, y)?;
    Ok(())
}

/// Rotate context
pub fn rotate_context(
    ctx: &CanvasRenderingContext2d,
    angle: f64,
) -> Result<(), JsValue> {
    ctx.rotate(angle)?;
    Ok(())
}

/// Scale context
pub fn scale_context(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
) -> Result<(), JsValue> {
    ctx.scale(x, y)?;
    Ok(())
}

/// Draw a particle
pub fn draw_particle(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    radius: f64,
    color: &str,
    alpha: f64,
) -> Result<(), JsValue> {
    ctx.begin_path();
    ctx.arc(x, y, radius, 0.0, std::f64::consts::PI * 2.0)?;
    ctx.set_fill_style_with_string(color)?;
    
    // Apply alpha
    let alpha_str = format!("{:.2}", alpha);
    ctx.global_alpha = alpha;
    
    ctx.fill();
    ctx.global_alpha = 1.0; // Reset alpha
    
    Ok(())
}

/// Draw terrain tile
pub fn draw_terrain_tile(
    ctx: &CanvasRenderingContext2d,
    tile: &web_sys::HtmlImageElement,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> Result<(), JsValue> {
    ctx.draw_image_with_html_image_element_and_sw_and_sh(
        tile,
        0.0,
        0.0,
        width,
        height,
        x,
        y,
        width,
        height,
    )?;
    
    Ok(())
}

/// Apply fog of war mask
pub fn apply_fog_mask(
    ctx: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
) -> Result<(), JsValue> {
    ctx.set_composite_operation("destination-in")?;
    ctx.set_fill_style_with_string("rgba(0, 0, 0, 0.7)")?;
    ctx.fill_rect(0.0, 0.0, width, height);
    ctx.set_composite_operation("source-over")?;
    
    Ok(())
}

/// Draw UI button
pub fn draw_button(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    text: &str,
    is_hovered: bool,
) -> Result<(), JsValue> {
    // Button background
    let bg_color = if is_hovered { "#5a5a8a" } else { "#4a4a6a" };
    ctx.set_fill_style_with_string(bg_color)?;
    
    // Draw rounded rectangle
    ctx.begin_path();
    ctx.round_rect(x, y, width, height, 8.0)?;
    ctx.fill();
    
    // Button text
    ctx.set_font("14px sans-serif");
    ctx.set_fill_style_with_string("#ffffff")?;
    let text_x = x + width / 2.0 - text.len() as f64 * 4.0;
    ctx.fill_text(text, text_x, y + height / 2.0 + 5.0)?;
    
    Ok(())
}

/// Check if point is inside rectangle
pub fn is_point_in_rect(px: f64, py: f64, rx: f64, ry: f64, rw: f64, rh: f64) -> bool {
    px > rx && px < rx + rw && py > ry && py < ry + rh
}

/// Check if point is inside circle
pub fn is_point_in_circle(px: f64, py: f64, cx: f64, cy: f64, radius: f64) -> bool {
    let dx = px - cx;
    let dy = py - cy;
    (dx * dx + dy * dy) < radius * radius
}

/// Clear canvas region
pub fn clear_rect(ctx: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64) {
    ctx.clear_rect(x, y, w, h);
}

/// Draw a progress bar
pub fn draw_progress_bar(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    progress: f64,
) -> Result<(), JsValue> {
    // Background
    ctx.set_fill_style_with_string("#333333")?;
    ctx.fill_rect(x, y, width, height);
    
    // Progress
    let progress_width = width * progress;
    ctx.set_fill_style_with_string("#4a4")?;
    ctx.fill_rect(x, y, progress_width, height);
    
    // Border
    ctx.set_stroke_style_with_string("#666666")?;
    ctx.set_line_width(1.0);
    ctx.stroke_rect(x, y, width, height);
    
    Ok(())
}

/// Draw health bar
pub fn draw_health_bar(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    health: f64,
    max_health: f64,
) -> Result<(), JsValue> {
    draw_progress_bar(ctx, x, y, width, height, health / max_health)?;
    Ok(())
}

/// Create pattern for fog or texture
pub fn create_fog_pattern(
    ctx: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
) -> Result<web_sys::CanvasPattern, JsValue> {
    // This is a placeholder - actual pattern creation would use an offscreen canvas
    Ok(web_sys::CanvasPattern::new())
}

/// Draw grid
pub fn draw_grid(
    ctx: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
    cell_size: f64,
    color: &str,
) -> Result<(), JsValue> {
    ctx.set_stroke_style_with_string(color)?;
    ctx.set_line_width(0.5);
    ctx.set_global_alpha(0.3);
    
    ctx.begin_path();
    
    // Vertical lines
    for x in (0.0..=width).step_by(cell_size as i32 as f64) {
        ctx.line_to(x, 0.0)?;
        ctx.line_to(x, height)?;
    }
    
    // Horizontal lines
    for y in (0.0..=height).step_by(cell_size as i32 as f64) {
        ctx.line_to(0.0, y)?;
        ctx.line_to(width, y)?;
    }
    
    ctx.stroke();
    ctx.set_global_alpha(1.0);
    
    Ok(())
}

/// Draw minimap frame
pub fn draw_minimap_frame(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> Result<(), JsValue> {
    // Background
    ctx.set_fill_style_with_string("rgba(0, 0, 0, 0.8)")?;
    ctx.set_shadow_color_with_str("rgba(0, 0, 0, 1.0)")?;
    ctx.set_shadow_blur(16.0);
    ctx.fill_rect(x, y, width, height);
    ctx.set_shadow_blur(0.0);
    
    // Border
    ctx.set_stroke_style_with_string("#888888")?;
    ctx.set_line_width(2.0);
    ctx.stroke_rect(x, y, width, height);
    
    Ok(())
}

/// Draw cursor/highlight effect
pub fn draw_cursor_effect(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
) -> Result<(), JsValue> {
    ctx.set_fill_style_with_string("#ffff00")?;
    ctx.set_global_alpha(0.5);
    ctx.set_line_width(2.0);
    ctx.set_stroke_style_with_string("#ffff00")?;
    
    ctx.begin_path();
    ctx.arc(x, y, 10.0, 0.0, std::f64::consts::PI * 2.0)?;
    ctx.fill();
    ctx.stroke();
    
    ctx.set_global_alpha(1.0);
    
    Ok(())
}

/// Smooth easing function for animations
pub fn ease_in_out(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).sqrt()
    }
}

/// Linear interpolation
pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

/// Clamping value
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.clamp(min, max)
}