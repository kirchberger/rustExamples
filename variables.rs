fn main() {
  // infers type
  let x = 13;
  println!("{}", x);

  // Can be explicit about type
  let x: f64 = 3.1419;
  println!("{}", x);

  let mut c = 42;
  println!("{}", c);
  c = 13;
  println!("{}", c);

  // defined types like such
  let a = 12u8;
  let sent = "hello world!";
  println!("{} {}",a, sent);

  // type conversion
  let x: i64 = 44;
  let b = a as i64 + x;
  println!("{}", b);
}
