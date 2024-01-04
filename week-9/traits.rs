pub trait Drawable {
    fn draw(&self) {
        println!("Shape not drawable");
    }
}

struct AbstractShape;
impl Drawable for AbstractShape {}

struct Square {
    size: u64 
}
impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square of size {}", self.size);
    }
}

fn main() {
    let s = AbstractShape {};
    s.draw();

    let sq = Square { size: 1 };
    sq.draw();
}
