use magnus::{define_module, function, prelude::*, Error, RHash, RArray, Symbol};

static PAYLOAD: &str = "ABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABC";

fn build_tree(depth: i32) -> RHash {
    let result = RHash::new();
    result.aset(Symbol::new("label"), PAYLOAD).unwrap();
    let children = RArray::new();
    if depth != 1 {
        children.push(build_tree(depth - 1)).unwrap();
        children.push(build_tree(depth - 1)).unwrap();
    }
    result.aset(Symbol::new("children"), children).unwrap();
    return result;
}

fn build_big_tree() -> RHash {
    return build_tree(13);
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("MagnusExtension")?;
    module.define_singleton_method("build_big_tree", function!(build_big_tree, 0))?;
    Ok(())
}
