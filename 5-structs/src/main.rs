#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn double(&mut self) {
        self.width = self.width * 2;
        self.height = self.height * 2;
    }

    fn print(&self) {
        for _ in 0..self.width {
            print!("*");
        }
        println!("");

        for _ in 0..self.height - 2 {
            print!("*");
            for _ in 0..self.width - 2 {
                print!(" ")
            }
            print!("*");
            println!("");
        }

        for _ in 0..self.width {
            print!("*");
        }
        println!("");
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 10,
        height: 5,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {rect1:#?}");
    rect1.print();
    rect1.double();
    rect1.print();

    let rect2 = Rectangle {
        width: 11,
        height: 6,
    };

    let rect3 = Rectangle {
        width: 8,
        height: 4,
    };

    println!("Can rect2 hold rect3? : {}", rect2.can_hold(&rect3));
    println!("Can rect3 hold rect2? : {}", rect3.can_hold(&rect2));

    let rect4 = Rectangle::square(6);
    rect4.print();
}
