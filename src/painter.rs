use crate::utils::wrap_point2_f32;
use nannou::math::clamp;
use nannou::{
    color::Hsv,
    draw::Draw,
    geom::{Point2, Rect, Vector2},
    math::map_range,
    rand::random_f32,
};
use std::collections::VecDeque;

pub struct Painter {
    positions: VecDeque<(Point2<f32>, Hsv)>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
}

impl Default for Painter {
    fn default() -> Self {
        let mut positions = VecDeque::with_capacity(1024);
        positions.push_back((Point2::new(0.0, 0.0), Hsv::new(0.0, 0.75, 0.60)));

        Self {
            positions,
            velocity: Vector2::new(5.0, 5.0),
            acceleration: Vector2::new(0.0, 0.0),
        }
    }
}

impl Painter {
    pub fn update(&mut self, window_rect: &Rect<f32>, mouse_xy: &Point2<f32>, color: &Hsv) {
        self.update_acceleration(mouse_xy);
        self.update_velocity();

        let (mut next_position, ..) = self.positions.back().unwrap().clone();
        next_position += self.velocity;
        next_position.x += (random_f32() - 0.5) * 10.0;
        next_position.y += (random_f32() - 0.5) * 10.0;
        wrap_point2_f32(&mut next_position, window_rect);

        if self.positions.len() > 1023 {
            self.positions.pop_front();
        }

        self.positions.push_back((next_position, color.clone()));
    }

    pub fn draw(&self, draw: &Draw) {
        // self.positions.iter().for_each(|position| {
        //     draw.ellipse().xy(*position).radius(5.0).color(LIGHTGREEN);
        // })
        let (a, b) = self.positions.as_slices();

        draw.polyline()
            .weight(20.0)
            .join_round()
            .points_colored([a, b].concat());
    }

    fn update_acceleration(&mut self, attract_position: &Point2<f32>) {
        let (current_position, ..) = self.positions.back().unwrap().clone();
        let mapped_accel_x =
            attract_position.x + (200.0 * (random_f32() - 0.5)) - current_position.x;
        let mapped_accel_y =
            attract_position.y + (200.0 * (random_f32() - 0.5)) - current_position.y;

        self.acceleration.x = map_range(mapped_accel_x, -512.0, 512.0, -1.0, 1.0);
        self.acceleration.y = map_range(mapped_accel_y, -512.0, 512.0, -1.0, 1.0);
    }

    fn update_velocity(&mut self) {
        self.velocity += self.acceleration;
        self.velocity.x = clamp(self.velocity.x, -10.0, 10.0);
        self.velocity.y = clamp(self.velocity.y, -10.0, 10.0);
    }
}
