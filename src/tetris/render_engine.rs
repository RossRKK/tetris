pub trait RenderEngine {
    fn clear_canvas(&self);
    fn draw_square(&self);
}

use flo_draw::*;
use flo_canvas::*;


pub struct FloDrawRenderEngine {
    pub canvas: DrawingTarget,
}

impl FloDrawRenderEngine {
    pub fn new(canvas: DrawingTarget) -> Self {
        Self { canvas: canvas }
    }
}

impl RenderEngine for FloDrawRenderEngine {

    fn clear_canvas(self: &Self) {

    }

    
    fn draw_square(self: &Self) {
        self.canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            gc.canvas_height(1000.0);
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);

            // Draw a rectangle...
            gc.new_path();
            gc.move_to(0.0, 0.0);
            gc.line_to(1000.0, 0.0);
            gc.line_to(1000.0, 1000.0);
            gc.line_to(0.0, 1000.0);
            gc.line_to(0.0, 0.0);

            gc.fill_color(Color::Rgba(1.0, 1.0, 0.8, 1.0));
            gc.fill();

            // Draw a triangle on top
            gc.new_path();
            gc.move_to(200.0, 200.0);
            gc.line_to(800.0, 200.0);
            gc.line_to(500.0, 800.0);
            gc.line_to(200.0, 200.0);

            gc.fill_color(Color::Rgba(0.0, 0.0, 0.8, 1.0));
            gc.fill();
        });
    }
}