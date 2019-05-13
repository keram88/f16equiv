//#define SMACK_ENABLED

#ifdef SMACK_ENABLED
#include "smack.h"
#endif

typedef unsigned short f16;
typedef unsigned char RES;

#ifndef SMACK_ENABLED
RES __VERIFIER_nondet_unsigned_char() {
  return 0;
}

float __VERIFIER_nondet_float() {
  return 0;
}

double __VERIFIER_nondet_double() {
  return 0;
}

unsigned short __VERIFIER_nondet_unsigned_short() {
  return 0;
}

void __SMACK_code(const char* fmt, ...) {
  return;
}
#endif

#define Q(x) #x
#define QUOTE(x) Q(x)

// why am I doing this?
#define DEF_PRED(pred) \
RES f16_ ## pred(f16 x, f16 y) {\
  RES ret = __VERIFIER_nondet_unsigned_char();\
  __SMACK_code(QUOTE(@c := if $fo ## pred.bvhalf.bool($bitcast.bv16.bvhalf(@h), $bitcast.bv16.bvhalf(@h)) then 1bv8 else 0bv8;), ret, x, y);\
  return ret;\
}

f16 f32_to_u16(float x) {
  f16 ret = __VERIFIER_nondet_unsigned_short();
  __SMACK_code("assume $bitcast.bv16.bvhalf(@H) == $fptrunc.bvfloat.bvhalf($rmode, @f);", ret, x);
  return ret;
}

f16 f64_to_u16(float x) {
  f16 ret = __VERIFIER_nondet_unsigned_short();
  __SMACK_code("assume $bitcast.bv16.bvhalf(@H) == $fptrunc.bvdouble.bvhalf($rmode, @);", ret, x);
  return ret;
}

float f16_to_f32(f16 x) {
  float ret = __VERIFIER_nondet_float();
  __SMACK_code("@f := $fpext.bvhalf.bvfloat($rmode, $bitcast.bv16.bvhalf(@h));", ret, x);
  return ret;
}

double f16_to_f64(f16 x) {
  double ret = __VERIFIER_nondet_double();
  __SMACK_code("@ := $fpext.bvhalf.bvdouble($rmode, $bitcast.bv16.bvhalf(@h));", ret, x);
  return ret;
}


DEF_PRED(lt)
DEF_PRED(gt)
DEF_PRED(le)
DEF_PRED(ge)
DEF_PRED(eq)

/*int main(void) {
  __VERIFIER_assert(f16_lt(0,16384) == 0);
  return 0;
  }*/
