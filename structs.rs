#[derive(Debug)] // Give the trait debug to rectangle so it can be printed
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // WHAT IS rect1?
    println!("rect1 is {rect1:?}"); // print debug
    println!("rect1 is {rect1:#?}"); // pprint debug

    // We can also just use the macro dbg!
    // Which also prints file out line num and col.
    dbg!(&rect1);
    println!("Area: {}", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

/*

VER 2. With tuples

fn main() {
    let rect1 = (30, 50);
    println!("The area is {}", area(rect1));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


VER 1. Variables

fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/
