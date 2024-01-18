use rb_sys::{
    rb_ary_new, rb_ary_push, rb_define_module, rb_define_singleton_method, rb_hash_aset,
    rb_hash_new, rb_id2sym, rb_intern, rb_str_new, ID, VALUE,
};
use std::{intrinsics::transmute, os::raw::c_char};

static PAYLOAD: &str = "ABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABC";

static mut LABEL_INTERN: ID = 0;
static mut CHILDREN_INTERN: ID = 0;

unsafe fn build_tree(depth: i32) -> VALUE {
    let result = rb_hash_new();
    let children = rb_ary_new();
    if depth != 1 {
        rb_ary_push(children, build_tree(depth - 1));
        rb_ary_push(children, build_tree(depth - 1));
    }
    rb_hash_aset(
        result,
        rb_id2sym(LABEL_INTERN),
        rb_str_new(PAYLOAD.as_ptr() as *mut _, PAYLOAD.len() as _),
    );
    rb_hash_aset(result, rb_id2sym(CHILDREN_INTERN), children);
    return result;
}

unsafe extern "C" fn build_big_tree(_: VALUE) -> VALUE {
    return build_tree(13);
}

#[no_mangle]
unsafe extern "C" fn Init_rb_sys_extension() {
    let module = rb_define_module("RbSysExtension\0".as_ptr() as *mut _);

    LABEL_INTERN = rb_intern("label\0".as_ptr() as *const c_char);
    CHILDREN_INTERN = rb_intern("children\0".as_ptr() as *const c_char);

    rb_define_singleton_method(
        module,
        "build_big_tree\0".as_ptr() as *mut _,
        Some(transmute::<unsafe extern "C" fn(VALUE) -> VALUE, _>(
            build_big_tree,
        )),
        0,
    );
}
