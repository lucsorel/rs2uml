use anyhow::Result;
use std::{fs, path::Path};
use syn::{
    File,
    // ItemEnum,
    // ItemImpl,
    ItemStruct,
    // ItemTrait,
    parse_file,
    visit::Visit,
};

#[derive(Default)]
pub struct RustVisitor<'ast> {
    // pub enums: Vec<&'ast ItemEnum>,
    // pub impls: Vec<&'ast ItemImpl>,
    pub structs: Vec<&'ast ItemStruct>,
    // pub traits: Vec<&'ast ItemTrait>,
}

impl<'ast> Visit<'ast> for RustVisitor<'ast> {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        println!("struct: {:?}", i.ident);
        self.structs.push(i);
        // syn::visit::visit_item_struct(self,i);
    }
}

fn parse_rust_file(path: &Path) -> Result<File> {
    let source = fs::read_to_string(path)?;
    Ok(parse_file(&source)?)
}

pub fn analyze_rust_ast(path: &Path) -> Result<()> {
    let ast = parse_rust_file(path)?;
    let mut visitor = RustVisitor::default();
    visitor.visit_file(&ast);

    Ok(())
}
