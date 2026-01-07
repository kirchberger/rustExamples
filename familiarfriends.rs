use std::alloc::{alloc, Layout};
use std::ops::Deref;

struct Pie {
  secret_recipe: usize,
}

impl Pie {
  fn new() -> Self {
    
    let layout = Layout::from_size_align(4, 1).unwrap();

    unsafe {
      
      let ptr = alloc(layout) as *mut u8;
      ptr.write(86);
      ptr.add(1).write(14);
      ptr.add(2).write(73);
      ptr.add(3).write(64);

      Pie { secret_recipe: ptr as usize }

    }
  }
}

impl Deref for Pie {
  type Target = f32;
  fn deref(&self) -> &f32 {
    
    let pointer = self.secret_recipe as *const f32;

    unsafe { &*pointer }
  }
}

fn main() {
  let p = Pie::new();

  println!("{:?}", *p);
}
