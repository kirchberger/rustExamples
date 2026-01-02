fn main() {
  let x = 42;

  match x {
    0 => {
      println!("found zero");
    }

    1 | 2 => {
      println!("found 1 or 0");
    }

    3..=9 => {

