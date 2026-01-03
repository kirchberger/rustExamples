fn main() {
  // Using a static method to create an instance of a string
  let s = String::from("Hello World!");
  // Using a method on the instance
  println!("{} is {} characters long", s, s.len());
}
