#[derive(Debug)]
struct Rectangle {
    width:  u32,
    height: u32,
}

fn create_rectangle(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
}

fn aire_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

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
    println!("aire:\n{}", aire_rectangle(rectangle));
    println!();
}

fn main() {
    let rectangle: Rectangle = create_rectangle(30, 50);

    display_rectangle(&rectangle);

    let rectangle: Rectangle = create_rectangle(10, 20);

    display_rectangle(&rectangle);

    for values in [[1,2], [2,4], [4,8], [8, 16]] {
        let rectangle: Rectangle = create_rectangle(values[0], values[1]);
        display_rectangle(&rectangle);
    }
}
