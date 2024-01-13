use rb_sys::{
    rb_ary_new, rb_ary_push, rb_define_module, rb_define_singleton_method, rb_hash_aset,
    rb_hash_new, rb_id2sym, rb_intern, rb_str_new_cstr, VALUE,
};
use std::{ffi::CString, intrinsics::transmute, os::raw::c_char};

trait AsCStr {
    fn to_cstring(&self) -> *const c_char;
}

impl AsCStr for str {
    /// Convert a Rust string to a C string.
    fn to_cstring(&self) -> *const c_char {
        CString::new(self).unwrap().into_raw()
    }
}

static PAYLOAD: &str = "ABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABC";

unsafe fn build_tree(depth: i32) -> VALUE {
    let result = rb_hash_new();
    let children = rb_ary_new();
    if depth != 1 {
        rb_ary_push(children, build_tree(depth - 1));
        rb_ary_push(children, build_tree(depth - 1));
    }
    rb_hash_aset(
        result,
        rb_id2sym(rb_intern("label".to_cstring())),
        rb_str_new_cstr(PAYLOAD.to_cstring()),
    );
    rb_hash_aset(
        result,
        rb_id2sym(rb_intern("children".to_cstring())),
        children,
    );
    return result;
}

unsafe extern "C" fn build_big_tree(_: VALUE) -> VALUE {
    return build_tree(13);
}

#[no_mangle]
unsafe extern "C" fn Init_rb_sys_extension() {
    let module = rb_define_module("RbSysExtension".to_cstring());

    rb_define_singleton_method(
        module,
        "build_big_tree".to_cstring(),
        Some(transmute::<unsafe extern "C" fn(VALUE) -> VALUE, _>(
            build_big_tree,
        )),
        0,
    );
}
