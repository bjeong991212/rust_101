fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;
    let r2: &String = &s;
    println!("r1: {r1}, r2: {r2}");
    let r3 = &mut s;
    println!("r3: {r3}");
    // println!("r1: {r1}, r2: {r2}");
}
