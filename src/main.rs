mod framebuffer;
mod linea;

use framebuffer::FrameBuffer;
use linea::Linea;
use raylib::prelude::*;

fn main() {
    let width = 256;
    let height = 256;

    let linea = Linea::new(17, 17, 212, 141);

    let mut fb_ecuacion = FrameBuffer::new(width, height, Color::BLACK);
    fb_ecuacion.set_current_color(Color::WHITE);
    linea.ecuacion(&mut fb_ecuacion);
    fb_ecuacion.render("line_ecuacion.png");

    let mut fb_dda = FrameBuffer::new(width, height, Color::BLACK);
    fb_dda.set_current_color(Color::WHITE);
    linea.dda(&mut fb_dda);
    fb_dda.render("line_dda.png");

    let mut fb_bresenham = FrameBuffer::new(width, height, Color::BLACK);
    fb_bresenham.set_current_color(Color::WHITE);
    linea.bresenham(&mut fb_bresenham);
    fb_bresenham.render("line_bresenham.png");

    println!("Imagenes guardadas correctamente.");
}
