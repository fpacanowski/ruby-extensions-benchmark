use magnus::{function, prelude::*, value::Lazy, Error, RHash, Ruby, StaticSymbol};

static PAYLOAD: &str = "ABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABCABC";

static LABEL: Lazy<StaticSymbol> = Lazy::new(|ruby| ruby.sym_new("label"));
static CHILDREN: Lazy<StaticSymbol> = Lazy::new(|ruby| ruby.sym_new("children"));

fn build_tree(ruby: &Ruby, depth: i32) -> Result<RHash, Error> {
    let result = ruby.hash_new();
    result.aset(ruby.get_inner(&LABEL), PAYLOAD)?;
    let children = ruby.ary_new();
    if depth != 1 {
        children.push(build_tree(ruby, depth - 1)?)?;
        children.push(build_tree(ruby, depth - 1)?)?;
    }
    result.aset(ruby.get_inner(&CHILDREN), children)?;
    return Ok(result);
}

fn build_big_tree(ruby: &Ruby) -> Result<RHash, Error> {
    return build_tree(ruby, 13);
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("MagnusExtension")?;
    module.define_singleton_method("build_big_tree", function!(build_big_tree, 0))?;
    Ok(())
}
