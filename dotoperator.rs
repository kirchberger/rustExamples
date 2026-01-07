struct Foo {
  value: i32
}

fn main() {
  let f = Foo {value: 42 };
  let refrefref_f = &&&f;
  println!("{}", refrefref_f.value);
  // dots automatically dereference
}
