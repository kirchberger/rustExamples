struct Foo {
  x:i32,
}

fn something<'a>(foo: &'a Foo) -> &'a i32 {
  return &foo.x;
}

fn main() {
  let mut foo = Foo {x:42};
  let x = &mut foo.x;
  *x = 13;

  let y = something(&foo);
  println!("{}", y);
}
