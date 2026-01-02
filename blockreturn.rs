fn example() -> i32 {
  let x = 42;

  let v = if x < 42 { -1 } else { 1 };
  println!("from if: {}", v);

  let food = "hamburger";
  let result = match food {
    "hotdog" => "is hotdog",

    _ => "is not hotdog"
  };
  println!("identifying food: {}", result);

  let v = {
    let a = 1;
    let b = 2;
    a + b
  };
  println!("from block {}", v);

  v + 4
}

fn main() {
  println!("from function {}", example());
}
