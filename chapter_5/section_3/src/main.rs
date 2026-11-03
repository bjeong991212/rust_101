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

  fn width(&self) -> u32 {
    self.width
  }

  fn height(&self) -> u32 {
    self.height
  }

  fn square(size: u32) -> Self {
    Self {
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
    println!(
        "rect1 width: {}, height: {}, area: {}",
        rect1.width, 
        rect1.height,
        rect1.area(),
    );

    let rect2 = Rectangle::square(40);
    println!(
        "rect2 width: {}, height: {}, area: {}",
        rect2.width(), 
        rect2.height(),
        rect2.area(),
    );

    println!(
        "Can rect2 hold rect1?: {}",
        rect2.can_hold(&rect1),
    )

}