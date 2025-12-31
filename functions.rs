fn add(x: i32, y: i32) -> i32 {
  x + y
}

// returns tuple
fn swap(x: i32, y: i32) -> (i32, i32) {
  return (y, x);
}

fn main() {
  println!("30 + 40 = {}", add(30,40));
  let (a,b) = swap(30,40);
  println!("{}, {}", a, b);
  let c = swap(30,40);
  println!("{}, {}", c.0, c.1);
}
