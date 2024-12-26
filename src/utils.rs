use gpui::{point, px};

pub fn cood_to_point(cood: plotters_backend::BackendCoord) -> gpui::Point<gpui::Pixels> {
    point(px(cood.0 as f32), px(cood.1 as f32))
}
pub fn color_to_hsla(color: plotters_backend::BackendColor) -> gpui::Hsla {
    gpui::Rgba {
        r: color.rgb.0 as f32 / 255.0,
        g: color.rgb.1 as f32 / 255.0,
        b: color.rgb.2 as f32 / 255.0,
        a: color.alpha as f32,
    }
    .into()
}
