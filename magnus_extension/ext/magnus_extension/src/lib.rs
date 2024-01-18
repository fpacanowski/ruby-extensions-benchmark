use magnus::{function, prelude::*, Error, RHash, Ruby};

static PAYLOAD: &str = "ABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABC";

fn build_tree(ruby: &Ruby, depth: i32) -> RHash {
    let result = ruby.hash_new();
    result.aset(ruby.sym_new("label"), PAYLOAD).unwrap();
    let children = ruby.ary_new();
    if depth != 1 {
        children.push(build_tree(ruby, depth - 1)).unwrap();
        children.push(build_tree(ruby, depth - 1)).unwrap();
    }
    result.aset(ruby.sym_new("children"), children).unwrap();
    return result;
}

fn build_big_tree(ruby: &Ruby) -> RHash {
    return build_tree(ruby, 13);
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("MagnusExtension")?;
    module.define_singleton_method("build_big_tree", function!(build_big_tree, 0))?;
    Ok(())
}
