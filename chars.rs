fn main() {
  let chars = "hi Ferris".chars().collect::<Vec<char>>();
  println!("{}", chars.len());

  println!("{}", chars[3] as u32);
}
