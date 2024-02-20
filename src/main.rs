fn main() {
    
    // Tuple version, works, but harder to read from 
    let rect1 = (30,50);
    println!(
        "The area of the tuple rectangle is {} square pixels.",
        area(rect1)
    );

    // Struc version, works as well, but code is easier to work with, for you and others
    let rect_struct0 = Rectangle { width: 30, height: 50};
    println!("react struct is: {:?}", rect_struct0);
    println!(
        "The area of the struct rectangle is {} square pixels.",
        area_of_struct(&rect_struct0)
    );
    println!("{}", rect_struct0.area());

    let rect_struct1 = Rectangle { width: 20, height: 40};
    let rect_struct2 = Rectangle { width: 30, height: 50};
    println!("Can rect_struct0 hold rect_struct1? {}", rect_struct0.can_hold(&rect_struct1));
    println!("Can rect_struct1 hold rect_struct2? {}", rect_struct0.can_hold(&rect_struct2));
}

// Tuple version
fn area(dimnesions: (u32, u32)) -> u32 {
    dimnesions.0 * dimnesions.1
}

// Struct implementation
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
}


fn area_of_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
