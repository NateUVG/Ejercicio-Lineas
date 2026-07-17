use raylib::prelude::*;

struct FrameBuffer {
    image: Image,
    color: Color,
}

impl FrameBuffer {
    fn new(width: i32, height: i32, background: Color) -> Self {
        FrameBuffer {
            image: Image::gen_image_color(width, height, background),
            color: Color::WHITE,
        }
    }

    fn set_current_color(&mut self, color: Color) {
        self.color = color;
    }

    fn point(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.image.width() && y >= 0 && y < self.image.height() {
            self.image.draw_pixel(x, y, self.color);
        }
    }

    fn render(&self, filename: &str) {
        self.image.export_image(filename);
    }
}

fn linea_ecuacion(fb: &mut FrameBuffer, x0: i32, y0: i32, x1: i32, y1: i32) {
    let m = (y1 - y0) as f32 / (x1 - x0) as f32;
    let b = y0 as f32 - m * x0 as f32;
    for x in x0..=x1 {
        let y = m * x as f32 + b;
        fb.point(x, y.round() as i32);
    }
}

fn linea_dda(fb: &mut FrameBuffer, x0: i32, y0: i32, x1: i32, y1: i32) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let steps = dx.abs().max(dy.abs());

    if steps == 0 {
        fb.point(x0, y0);
        return;
    }

    let x_inc = dx as f32 / steps as f32;
    let y_inc = dy as f32 / steps as f32;

    let mut x = x0 as f32;
    let mut y = y0 as f32;

    for _ in 0..=steps {
        fb.point(x.round() as i32, y.round() as i32);
        x += x_inc;
        y += y_inc;
    }
}

fn linea_bresenham(fb: &mut FrameBuffer, x0: i32, y0: i32, x1: i32, y1: i32) {
    let mut x = x0;
    let mut y = y0;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        fb.point(x, y);
        if x == x1 && y == y1 {
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

fn main() {
    let width = 256;
    let height = 256;

    let (x0, y0, x1, y1) = (17, 17, 212, 141);

    let mut fb_ecuacion = FrameBuffer::new(width, height, Color::BLACK);
    fb_ecuacion.set_current_color(Color::WHITE);
    linea_ecuacion(&mut fb_ecuacion, x0, y0, x1, y1);
    fb_ecuacion.render("line_ecuacion.png");

    let mut fb_dda = FrameBuffer::new(width, height, Color::BLACK);
    fb_dda.set_current_color(Color::WHITE);
    linea_dda(&mut fb_dda, x0, y0, x1, y1);
    fb_dda.render("line_dda.png");

    let mut fb_bresenham = FrameBuffer::new(width, height, Color::BLACK);
    fb_bresenham.set_current_color(Color::WHITE);
    linea_bresenham(&mut fb_bresenham, x0, y0, x1, y1);
    fb_bresenham.render("line_bresenham.png");

    println!("Imagenes guardadas correctamente.");
}
