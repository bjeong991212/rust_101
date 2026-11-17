use std::collections::HashMap;

fn main() {
    let text = "one ravioli two ravioli three ravioli";
    let mut map = HashMap::new();

    for word in text.split_whitespace() { // (one,ravioli,two,ravioli,three,ravioli)
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}"); // {"three": 1, "ravioli": 3, "two": 1, "one": 1}

}