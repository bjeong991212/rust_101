use crate::garden::vegetables::cabbage;

pub mod garden;

fn main() {
    let plant = cabbage {};
    println!("I'm growing {plant:?}!");
}