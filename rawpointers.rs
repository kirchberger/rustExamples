fn main() {
  let a = 42;
  let memory_location = &a as *const i32;
  println!("Data is here {}", memory_location as usize);
}
