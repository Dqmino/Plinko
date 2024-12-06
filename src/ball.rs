use macroquad::color::YELLOW;
use macroquad::prelude::draw_circle;
use crate::{BALL_RADIUS, BOUNCE_DAMPING, FRICTION, GRAVITY, PEG_RADIUS, SCREEN_HEIGHT, SCREEN_WIDTH};

pub(crate) struct Ball {
    x: f32,
    pub(crate) y: f32,
    vx: f32,
    vy: f32,
}

impl Ball {
    pub(crate) fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0
        }
    }

    pub(crate) fn update(&mut self, dt: f32, pegs: &[(f32, f32)]) {
        self.vy += GRAVITY * dt * dt;

        self.vx *= FRICTION;
        self.vy *= FRICTION;

        self.x += self.vx * dt;
        self.y += self.vy * dt;

        for &(peg_x, peg_y) in pegs {
            let dx = self.x - peg_x;
            let dy = self.y - peg_y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance < BALL_RADIUS + PEG_RADIUS {
                let overlap = BALL_RADIUS + PEG_RADIUS - distance;
                let angle = dx.atan2(dy);

                self.x += overlap * angle.cos();
                self.y += overlap * angle.sin();
                let normal_vx = dx / distance;
                let normal_vy = dy / distance;
                let dot = self.vx * normal_vx + self.vy * normal_vy;

                self.vx -= 2.0 * dot * normal_vx * BOUNCE_DAMPING;
                self.vy -= 2.0 * dot * normal_vy * BOUNCE_DAMPING;
            }
        }
        if self.x < BALL_RADIUS || self.x > SCREEN_WIDTH - BALL_RADIUS {
            self.vx = -self.vx * BOUNCE_DAMPING;
        }
        if self.y > SCREEN_HEIGHT - BALL_RADIUS {
            self.vy = -self.vy * BOUNCE_DAMPING;
            self.y = SCREEN_HEIGHT - BALL_RADIUS;
        }
    }
    pub(crate) fn draw(&self) {
        draw_circle(self.x, self.y, BALL_RADIUS, YELLOW);
    }
}
