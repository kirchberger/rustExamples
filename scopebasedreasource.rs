struct Foo {
  x:i32,
}

fn main() {
  let fooa = Foo {x:42};
  let foob = Foo {x:13};

  println!("{}",fooa.x);
  println!("{}",foob.x);
  //foo a and b are dropped here
}
