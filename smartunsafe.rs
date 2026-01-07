fn main() {
  let a: [u8; 4] = [86, 14, 73, 64];
  let pointer_a = &a as *const u8 as usize;
  println!("Data memory location: {}", pointer_a);

  let pointer_b = pointer_a as *const f32;
  let b = unsafe {
    *pointer_b
  };
  println!("I swear this is a pie {}", b);
}
