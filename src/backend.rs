use crate::line::Line;
use crate::utils::{color_to_hsla, cood_to_point};
use gpui::{point, px, Bounds, Pixels, SharedString, TextRun};
use plotters_backend::{
    BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,
};

/// The embedded backend for plotters in gpui
pub struct GpuiBackend<'a, 'b> {
    bounds: Bounds<Pixels>,
    cx: &'a mut gpui::WindowContext<'b>,
}
impl<'a, 'b> GpuiBackend<'a, 'b> {
    /// Create a new embedded backend
    pub fn new(bounds: Bounds<Pixels>, cx: &'a mut gpui::WindowContext<'b>) -> Self {
        Self { bounds, cx }
    }
}
impl<'a, 'b> DrawingBackend for GpuiBackend<'a, 'b> {
    type ErrorType = crate::Error;

    fn get_size(&self) -> (u32, u32) {
        let size = self.bounds.size;
        (size.width.0 as u32, size.height.0 as u32)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.draw_path([point, point], &color)
    }
    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut line = Line::between_points(
            self.bounds.origin + cood_to_point(from).into(),
            self.bounds.origin + cood_to_point(to).into(),
        );
        line.color = color_to_hsla(style.color());
        line.width = px(style.stroke_width() as _);
        line.render_pixels(self.cx);
        Ok(())
    }
    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let upper_left = self.bounds.origin + cood_to_point(upper_left);
        let bottom_right = self.bounds.origin + cood_to_point(bottom_right);
        let color = color_to_hsla(style.color());
        if fill {
            let mut path = gpui::Path::new(upper_left);
            path.line_to(point(upper_left.x, bottom_right.y));
            path.line_to(bottom_right);
            path.line_to(point(bottom_right.x, upper_left.y));
            path.line_to(upper_left);
            self.cx.paint_path(path, color);
        } else {
            for (p1, p2) in [
                (upper_left, point(bottom_right.x, upper_left.y)),
                (point(bottom_right.x, upper_left.y), bottom_right),
                (bottom_right, point(upper_left.x, bottom_right.y)),
                (point(upper_left.x, bottom_right.y), upper_left),
            ] {
                Line::between_points(p1.into(), p2.into()).render_pixels(self.cx);
            }
        }

        Ok(())
    }
    // path in plotters does not close the shape
    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let iter = path.into_iter();
        let mut points = Vec::with_capacity(iter.size_hint().0 * 2);
        for point in iter {
            points.push(self.bounds.origin + cood_to_point(point).into());
        }

        if points.is_empty() {
            return Ok(());
        }

        let mut line = Line::new();
        line.points = points;
        line.width = px(style.stroke_width() as _);
        line.color = color_to_hsla(style.color());
        line.render_pixels(self.cx);

        Ok(())
    }
    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut iter = vert.into_iter();
        let start = match iter.next() {
            Some(start) => start,
            None => return Ok(()),
        };
        let mut path = gpui::Path::new(self.bounds.origin + cood_to_point(start));
        for point in iter {
            path.line_to(self.bounds.origin + cood_to_point(point));
        }
        let color = color_to_hsla(style.color());
        self.cx.paint_path(path, color);
        Ok(())
    }
    fn draw_text<TStyle: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &TStyle,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = color_to_hsla(style.color());
        let point = self.bounds.origin + cood_to_point(pos);
        let shared_string = SharedString::from(text.to_string());
        let size = px(style.size() as _);
        let ts = self.cx.text_system();
        let shaped_line = ts
            .shape_line(
                shared_string,
                size,
                &[TextRun {
                    len: text.len(),
                    font: self.cx.text_style().font(),
                    color,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                }],
            )
            .map_err(|err| DrawingErrorKind::FontError(err.to_string().into()))?;
        shaped_line.paint(point, size, self.cx).map_err(|err| {
            DrawingErrorKind::DrawingError(std::io::Error::new(
                std::io::ErrorKind::Other,
                err.to_string(),
            ))
        })?;
        Ok(())
    }
}
