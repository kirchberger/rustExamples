struct Foo { 
  x:i32,
}

fn something(f:&mut Foo) {
  f.x += 1;
  println!("{}",f.x);
}

fn main() {
  let mut foo = Foo {x:42};
  something(&mut foo);
  something(&mut foo);
}
