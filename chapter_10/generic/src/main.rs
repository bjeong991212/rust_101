fn largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    return largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &U {
     &self.y
    }
    fn mixup<T2, U2>(self, other: Point<T2, U2>) -> Point<T, U2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<i32, i32> {
    fn get_i32_x(&self) -> &i32 {
        &self.x
    }
    fn get_i32_y(&self) -> &i32 {
        &self.y
    }
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_generic(&number_list);
    println!("The largest number is {result}");

    let point_a = Point{
        x: 1,
        y: 2.0
    };


}