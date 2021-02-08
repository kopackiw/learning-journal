#[derive(Debug)]
enum IpAddress {
  V4(u8, u8, u8, u8),
  V6(String),
}

impl IpAddress {
  fn method_here_is_too_much(&self) {
    println!("Here I am: {:?}", &self)
  }
}

fn main() {
  let _home = IpAddress::V4(127, 0, 0, 1);

  let loopback = IpAddress::V6(String::from("::1"));

  loopback.method_here_is_too_much();

  let a = Some(5);

  let _q = match a {
    Some(1) => 1,
    Some(_) => 5,
    None => 0,
  };

  let b = a.map(|v| v * 2);
  println!("b: {:?}", b);
}
