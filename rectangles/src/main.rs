fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("The area of rectangle is {} square pixels.",
    area(width1, height1));
    let rectangle = (30, 40);
    println!("The area of rectangle with related parameters at one place is {} square pixels.", area_with_related_parameters(rectangle));
    println!("Lets see if I can access the rectangle variable");
    println!("{:?}", rectangle);

    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    println!("The area of rectangle with related parameters at one place is {} square pixels.", area_using_struct(&rect1));
    println!("The rectangle width is {} and height is {}", rect1.width, rect1.height);
}

fn area_using_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_related_parameters(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32
}

