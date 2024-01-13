#include "c_extension.h"

VALUE rb_mCExtension;

void
Init_c_extension(void)
{
  rb_mCExtension = rb_define_module("CExtension");
}
