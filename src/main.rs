#[macro_use]
mod smack;
use smack::*;

extern crate half;
use half::*;

use std::mem::transmute;

#[link(name = "chalf")]
extern "C" {
  fn f16_eq(_: u16, _:u16) -> u8;
  fn f16_gt(_: u16, _:u16) -> u8;
  fn f16_lt(_: u16, _:u16) -> u8;
  fn f16_le(_: u16, _:u16) -> u8;
  fn f16_ge(_: u16, _:u16) -> u8;
  fn f16_to_f32(_: u16) -> f32;
  fn f16_to_f64(_: u16) -> f64;
  fn f32_to_u16(_: f32) -> u16;
  fn f64_to_u16(_: f32) -> u16;
}

fn main() {
  let x: u16 = 3.nondet();
  let y: u16 = 7.nondet();
  let a: u32 = 990.nondet();
  let f: f32 = unsafe { transmute::<u32, f32>(a) };//f32::from_bits(a);
  let b: u64 = 990.nondet();
  let f2: f64 = unsafe { transmute::<u64, f64>(b) };//f32::from_bits(a);
  let z = f16::from_bits(x);

  assume!(!z.is_nan());
  assume!(!f.is_nan());
  assume!(!f2.is_nan());
  // assert!(unsafe { f16_to_f32(x) } == z.to_f32());
  // assert!(unsafe { f16_to_f64(x) } == z.to_f64());
  // assert!(unsafe { f32_to_u16(f) } == f16::from_f32(f).to_bits());
  // assert!(unsafe { f64_to_u16(f) } == f16::from_f64(f2).to_bits());
  assert!(unsafe { f16_eq(x, y) } == (f16::from_bits(x) == f16::from_bits(y)) as u8);
  // assert!(unsafe { f16_gt(x, y) } == (f16::from_bits(x) > f16::from_bits(y)) as u8);
  // assert!(unsafe { f16_lt(x, y) } == (f16::from_bits(x) < f16::from_bits(y)) as u8);
  // assert!(unsafe { f16_le(x, y) } == (f16::from_bits(x) <= f16::from_bits(y)) as u8);
  // assert!(unsafe { f16_ge(x, y) } == (f16::from_bits(x) >= f16::from_bits(y)) as u8);
}
