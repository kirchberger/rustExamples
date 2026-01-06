fn main() {
  let a = "hi Ferris";
  println!("{}", a.len());
  let first = &a[0..2];
  let second = &a[3..9];

  println!("{} {}", first, second);
}
