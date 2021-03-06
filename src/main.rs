#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(length_of_sides: u32) -> Rectangle {
        Rectangle{
            width: length_of_sides,
            height: length_of_sides,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let rect4 = Rectangle::square(10);

    println!(
        "The area of the rectangles are {}, {}, {}, and {} square pixels.",
        rect1.area(),
        rect2.area(),
        rect3.area(),
        rect4.area(),
    );

    println!(
        "Does rect1 fit inside of rect3? {}",
        rect3.can_hold(&rect1)
    );

    println!("{:#?}", rect1);
    println!("{:#?}", rect2);
    println!("{:#?}", rect3);
    println!("{:#?}", rect4);
}