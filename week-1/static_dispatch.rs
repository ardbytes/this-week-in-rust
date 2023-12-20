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

fn draw_all<T: Drawable>(shapes: &[T]) {
    shapes.iter().for_each(|drawable| {
        drawable.draw();
    });
}

fn main() {
    let shapes1: Vec<Circle> = vec![Circle {}, Circle {}];
    let shapes2: Vec<Square> = vec![Square {}];
    draw_all(&shapes1);
    draw_all(&shapes2);
}
