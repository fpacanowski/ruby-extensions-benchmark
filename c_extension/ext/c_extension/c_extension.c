#include "c_extension.h"

char PAYLOAD[] = "ABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABC";

VALUE rb_mCext;

VALUE build_tree(int depth) {
  VALUE result = rb_hash_new();
  VALUE children = rb_ary_new();
  if(depth != 1) {
    rb_ary_push(children, build_tree(depth-1));
    rb_ary_push(children, build_tree(depth-1));
  }
  rb_hash_aset(result, ID2SYM(rb_intern("label")), rb_str_new_cstr(PAYLOAD));
  rb_hash_aset(result, ID2SYM(rb_intern("children")), children);
  return result;
}

VALUE build_big_tree() {
  return build_tree(13);
}

void
Init_c_extension(void)
{
  rb_mCext = rb_define_module("CExtension");
  rb_define_module_function(rb_mCext, "build_big_tree", build_big_tree, 0);
}
