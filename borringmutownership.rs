struct Foo {
  x:i32,
}

fn something(f: Foo) {
  println!("{}", f.x);
}

fn main () {
  let mut foo = Foo {x:42};
  let f = &mut foo;

  f.x = 13;
  // f is dropped after this point since it is no longer used

  println!("{}", foo.x);

  foo.x = 7;

  something(foo);

}
