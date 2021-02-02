#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
    let can_hold_horizontally = self.height >= other_rectangle.height;
    let can_hold_vertically = self.width >= other_rectangle.width;
    can_hold_horizontally && can_hold_vertically
  }

  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
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

  println!("The area: {}", area(&rect1));
  println!("The area: {}", rect1.area());

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

  println!("rect1: {:#?}", &Rectangle::square(20));
}

fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
