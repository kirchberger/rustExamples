static PI: f64 = 3.1415;

fn main() {
  static mut SECRET: &'static str = "swordfish";

  let msg: &'static str = "Hello World";
  let p: &'static f64 = &PI;
  println!("{} {}",msg, p);

  unsafe {
    SECRET = "abracadabra";
    println!("{}", SECRET);
  }
}
