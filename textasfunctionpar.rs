fn loud(msg:&str){
  println!("{}!!!",msg.to_uppercase());
}

fn main() {
  loud("hello");
  loud(&String::from("hello"));
}
