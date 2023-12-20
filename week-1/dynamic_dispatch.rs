trait Drawable {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing Circle");
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing Square");
    }
}

fn draw_all<T: Drawable + ?Sized>(shapes: &[Box<T>]) {
    shapes.iter().for_each(|drawable| {
        drawable.draw();
    });
}

fn main() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle {}),
        Box::new(Circle {}),
        Box::new(Square {}),
    ];
    draw_all(&shapes);
}
