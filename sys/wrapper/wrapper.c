#include <caml/bigarray.h>
#include <caml/callback.h>
#include <caml/custom.h>
#include <caml/fail.h>
#include <caml/memory.h>
#include <caml/mlvalues.h>
#include <caml/threads.h>
#include <caml/alloc.h>

void caml_drop(struct caml__roots_block *caml__frame) { CAMLdrop; }

value caml_return(struct caml__roots_block *caml__frame, value v) {
  CAMLreturn(v);
}

void caml_return0(struct caml__roots_block *caml__frame) { CAMLreturn0; }

struct caml__roots_block *caml_param0() {
  CAMLparam0();
  return caml__frame;
}

struct caml__roots_block *caml_param1(value a) {
  CAMLparam1(a);
  return caml__frame;
}

struct caml__roots_block *caml_param2(value a, value b) {
  CAMLparam2(a, b);
  return caml__frame;
}

struct caml__roots_block *caml_param3(value a, value b, value c) {
  CAMLparam3(a, b, c);
  return caml__frame;
}

struct caml__roots_block *caml_param4(value a, value b, value c, value d) {
  CAMLparam4(a, b, c, d);
  return caml__frame;
}

struct caml__roots_block *caml_param5(value a, value b, value c, value d,
                                      value e) {
  CAMLparam5(a, b, c, d, e);
  return caml__frame;
}

void caml_xparam1(struct caml__roots_block *caml__frame, value a) {
  CAMLxparam1(a);
}

void caml_xparam2(struct caml__roots_block *caml__frame, value a, value b) {
  CAMLxparam2(a, b);
}

void caml_xparam3(struct caml__roots_block *caml__frame, value a, value b,
                  value c) {
  CAMLxparam3(a, b, c);
}

void caml_xparam4(struct caml__roots_block *caml__frame, value a, value b,
                  value c, value d) {
  CAMLxparam4(a, b, c, d);
}

void caml_xparam5(struct caml__roots_block *caml__frame, value a, value b,
                  value c, value d, value e) {
  CAMLxparam5(a, b, c, d, e);
}
