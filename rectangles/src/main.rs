use std::fmt;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle {}x{}", self.width, self.height)
    }
}

fn main() {
    let mut sq = Rectangle::square(3);
    println!("The square {} has area {}", sq, sq.area());
    sq.double();
    println!("When sides are doubled, area becomes {}", sq.area());
    sq.height = 1;
    let rect = sq;
    println!("When height is 1, area becomes {}", rect.area())
}
