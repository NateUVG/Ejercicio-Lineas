use crate::framebuffer::FrameBuffer;

pub struct Linea {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

impl Linea {
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        Linea { x0, y0, x1, y1 }
    }

    pub fn ecuacion(&self, fb: &mut FrameBuffer) {
        let m = (self.y1 - self.y0) as f32 / (self.x1 - self.x0) as f32;
        let b = self.y0 as f32 - m * self.x0 as f32;
        for x in self.x0..=self.x1 {
            let y = m * x as f32 + b;
            fb.point(x, y.round() as i32);
        }
    }

    pub fn dda(&self, fb: &mut FrameBuffer) {
        let dx = self.x1 - self.x0;
        let dy = self.y1 - self.y0;
        let steps = dx.abs().max(dy.abs());

        if steps == 0 {
            fb.point(self.x0, self.y0);
            return;
        }

        let x_inc = dx as f32 / steps as f32;
        let y_inc = dy as f32 / steps as f32;

        let mut x = self.x0 as f32;
        let mut y = self.y0 as f32;

        for _ in 0..=steps {
            fb.point(x.round() as i32, y.round() as i32);
            x += x_inc;
            y += y_inc;
        }
    }

    pub fn bresenham(&self, fb: &mut FrameBuffer) {
        let mut x = self.x0;
        let mut y = self.y0;

        let dx = (self.x1 - self.x0).abs();
        let dy = -(self.y1 - self.y0).abs();
        let sx = if self.x0 < self.x1 { 1 } else { -1 };
        let sy = if self.y0 < self.y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            fb.point(x, y);
            if x == self.x1 && y == self.y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}
