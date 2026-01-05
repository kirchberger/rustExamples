fn main() {
  let mut foo = 42;
  let f = &mut foo;
  let bar = *f;
  *f = 13;
  println!("{}",bar);
  println!("{}",foo);
}

