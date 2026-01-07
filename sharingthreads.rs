use std::sync::Mutex;

struct Pie;

impl Pie {
  fn eat(&self) {
    println!("only I eat the pie right now!");
  }
}

fn main(){
  let mutex_pie = Mutex::new(Pie);
  let ref_pie = mutex_pie.lock().unwrap();
  ref_pie.eat();
}
