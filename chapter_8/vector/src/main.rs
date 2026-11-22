fn main() {
    let mut v = vec![1, 2, 3, 4, 6];

    let x = v[0];           // 1
    let x = v.get(1);       // Some(&2)
    v[4] = 5;               // v = [1, 2, 3, 4, 5]

    v.push(6);              // v = [1, 2, 3, 4, 5, 6]
    v.pop();                // v 의 끝 값 제거
    v.insert(2, 10);        // v = [1, 2, 10, 3, 4, 5]
    v.remove(2);            // v 의 2번째 index 제거

    println!("length of vector v: {}", v.len());      // 5
    println!("is vector v empty?: {}", v.is_empty()); // false

    v.resize(7, 0);         // v = [1, 2, 3, 4, 5, 0, 0]
    v.truncate(5);          // v = [1, 2, 3, 4, 5]

    for i in v.iter() { // &v 로 iterate
        println!("iter(): {i}");
    }    
    for i in v.iter_mut() { // &mut v 로 iterate
        *i += 10;
        println!("iter_mut(): {i}");
        *i -= 10;
    } 
    // for i in v.into_iter() {} // 소유권을 소비하는 iterate
    for (i, y) in v.iter().enumerate() { // (index, value) 페어로 iterate
        println!("index: {i}, val: {y}");
    } 
}
