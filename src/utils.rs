use gpui::{point, px};

pub fn coord_to_point(
    origin: gpui::Point<gpui::Pixels>,
    coord: plotters_backend::BackendCoord,
) -> gpui::Point<gpui::Pixels> {
    origin + point(px(coord.0 as f32), px(coord.1 as f32))
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
