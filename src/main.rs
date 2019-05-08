extern crate half;

#[link(name = "chalf")]
extern "C" {
  fn f16_gt(_: u16, _:u16) -> u8;
}

fn main() {
  let x = 3;
  let y = 7;
  let z = unsafe { f16_gt(x, y) };
  println!("Hello, world!");
}
