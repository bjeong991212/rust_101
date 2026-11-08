enum PhoneType {
  nokia,
  iPhone(i32, String),
  galaxy(String, i32),
}

fn some_func(phone_type: PhoneType) {} // 위의 두 instance 모두 들어감

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {}
}

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let andy_phone = PhoneType::nokia;
    let brain_phone = PhoneType::iPhone(16, String::from("Pro")); // iPhone 을 constructor 처럼 함수로 선언
    let colin_phone = PhoneType::galaxy(String::from("ZFold"), 8);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_name = Some(String::from("byungho")); // some_name 은 Option<String> 타입이 된다.

}

