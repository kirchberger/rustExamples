struct Foo {
  x:i32,
}

fn something(a:&Foo) -> &i32 {
  return &a.x;
}

fn main() {
  let mut foo = Foo {x:42};
  let x = &mut foo.x;
  *x = 13;

  let y = something(&foo);
  println!("{}",y);
}
