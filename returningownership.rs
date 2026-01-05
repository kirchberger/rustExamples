struct Foo {
  x:i32,
}
fn something() -> Foo {
  Foo {x:42}
}
fn main() {
  // Ownership can be returned
  let foo = something();
  println!("{}",foo.x);
}
