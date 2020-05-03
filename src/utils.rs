use nannou::geom::{Point2, Range, Rect};

pub fn wrap_point2_f32(position: &mut Point2<f32>, window_rect: &Rect<f32>) {
    if !window_rect.x.contains(position.x) {
        position.x = wrap_f32(position.x, window_rect.x);
    }

    if !window_rect.y.contains(position.y) {
        position.y = wrap_f32(position.y, window_rect.y);
    }
}

pub fn wrap_f32(n: f32, range: Range<f32>) -> f32 {
    if range.contains(n) {
        return n;
    } else {
        let distance = range.end - range.start;
        let mut n = n;

        if n > range.end {
            while n > range.end {
                n = n - distance
            }
        } else if n < range.start {
            while n < range.start {
                n = n + distance
            }
        }

        n
    }
}
