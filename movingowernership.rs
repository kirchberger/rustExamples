struct Foo {
  x:i32,
}
fn something(f:Foo) {
  println!("{}", f.x);
}
fn main() {
  let foo = Foo {x:42};
  // foo is moved to something
  something(foo);
  // foo can nolonger be used
}
