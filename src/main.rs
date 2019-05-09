#[macro_use]
mod smack;
use smack::*;

extern crate half;
use half::*;

#[link(name = "chalf")]
extern "C" {
  fn f16_gt(_: u16, _:u16) -> u8;
  fn f16_lt(_: u16, _:u16) -> u8;
  fn f16_le(_: u16, _:u16) -> u8;
  fn f16_ge(_: u16, _:u16) -> u8;
}

fn main() {
  let x: u16 = 3.nondet();
  let y: u16 = 7.nondet();

  assert!(unsafe { f16_gt(x, y) } == (f16::from_bits(x) > f16::from_bits(y)) as u8);
  assert!(unsafe { f16_lt(x, y) } == (f16::from_bits(x) < f16::from_bits(y)) as u8);
  assert!(unsafe { f16_le(x, y) } == (f16::from_bits(x) <= f16::from_bits(y)) as u8);
  assert!(unsafe { f16_ge(x, y) } == (f16::from_bits(x) >= f16::from_bits(y)) as u8);
}
