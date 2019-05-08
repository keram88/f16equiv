#ifdef SMACK_ENABLED
#include "smack.h"
#endif

typedef unsigned short f16;
typedef unsigned char RES;

#ifndef SMACK_ENABLED
RES __VERIFIER_nondet_unsigned_char() {
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

DEF_PRED(lt)
DEF_PRED(gt)
DEF_PRED(leq)
DEF_PRED(geq)

//int main(void) {
//  __VERIFIER_assert(f16_gt(2,1) != 0);
//  return 0;
//}
