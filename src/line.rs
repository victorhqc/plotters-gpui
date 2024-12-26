use gpui::{Bounds, Hsla, Path, Pixels, Point, WindowContext};

#[derive(Clone, Debug)]
pub struct Line {
    pub points: Vec<Point<Pixels>>,
    pub width: Pixels,
    pub color: Hsla,
}

impl Line {
    pub fn new() -> Self {
        Self {
            points: vec![],
            width: 1.0.into(),
            color: gpui::black(),
        }
    }
    pub fn between_points(start: Point<Pixels>, end: Point<Pixels>) -> Self {
        let mut line = Self::new();
        line.add_point(start);
        line.add_point(end);
        line
    }
    pub fn width(mut self, width: f64) -> Self {
        self.width = width.into();
        self
    }
    pub fn add_point(&mut self, point: Point<Pixels>) {
        self.points.push(point);
    }

    pub fn render_pixels(&mut self, cx: &mut WindowContext) {
        let mut path = Path::new(self.points[0].into());
        for point in self.points.iter().cloned().skip(1) {
            path.line_to(point.into());
        }
        // now do the reverse to close the path
        for point in self.points.iter().cloned().rev() {
            let mut p: Point<Pixels> = point.into();
            // TODO: compute new points based on the slope
            p.x.0 -= self.width.0;
            p.y.0 += self.width.0;
            path.line_to(p);
        }

        cx.paint_path(path, self.color);
    }
}
