#[derive(Debug)]
struct Rectangle {
    width:  u32,
    height: u32,
}

impl Rectangle {
    fn aire(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_contain(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(side: u32) -> Rectangle {
        Rectangle { width: side, height: side }
    }
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn create_rectangle(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
}

// implementation de l'aire
//fn aire_rectangle(rectangle: &Rectangle) -> u32 {
//    rectangle.width * rectangle.height
//}

fn display_rectangle(rectangle: &Rectangle) {
    println!();

    // le formatage diffère selon le caractère

    // ici
    //  struct { attr }
    //println!("{:?}", rectangle);

    // ici
    //  struct {
    //      attr
    //  }
    println!("{:#?}", rectangle);
    println!("rectangle.is_square: {}", rectangle.is_square());

    // implementation de l'aire
    //println!("aire: {}", aire_rectangle(rectangle));
    println!("rectangle.aire: {}", rectangle.aire());

    println!();
}

fn main() {
    let rectangle: Rectangle = create_rectangle(30, 50);

    display_rectangle(&rectangle);

    let square: Rectangle = Rectangle::square(10);

    display_rectangle(&square);

    for values in [[1,2], [2,4], [4,8], [8, 8]] {
        let rectangle: Rectangle = create_rectangle(values[0], values[1]);
        display_rectangle(&rectangle);
    }

    println!("rectangle.can_contain square : {}", rectangle.can_contain(square));
}
