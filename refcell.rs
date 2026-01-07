use std::cell::RefCell;

struct Pie {
  slices: u8
}

impl Pie {
  fn eat(&mut self) {
    println!("tastes better on the heap");
    self.slices -=1;
  }
}

fn main() {
  let pie_cell = RefCell::new(Pie{slices:8});

  {
    let mut mut_ref_pie = pie_cell.borrow_mut();
    mut_ref_pie.eat();
    mut_ref_pie.eat();
  }

  let ref_pie = pie_cell.borrow();
  println!("{} slices left", ref_pie.slices);
}
