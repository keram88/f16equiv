#[macro_use]
mod smack;
use smack::*;

extern crate half;

#[link(name = "chalf")]
extern "C" {
  fn f16_gt(_: u16, _:u16) -> u8;
}

fn main() {
  let x: u16 = 3.nondet();
  let y: u16 = 7.nondet();
  let z = unsafe { f16_gt(x, y) };
  assert!(z == 2);
}
