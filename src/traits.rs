pub trait Movable {
    fn perform_movement(&mut self);
}

pub trait Drawable {
    fn draw(&self, x: f32, y: f32);
}
