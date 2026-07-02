use crate::domain::{UmlField, UmlItem, UmlItemType};
use anyhow::Result;
use std::{fs, path::Path};
use syn::{Field, Fields, FieldsNamed, File, ItemStruct, Visibility, parse_file, visit::Visit};
fn parse_struct_to_uml(item_struct: &ItemStruct) -> UmlItem {
    let name = item_struct.ident.to_string();
    let fields: Vec<UmlField> = parse_fields_to_uml(item_struct.fields.clone());
    UmlItem {
        name: name,
        is_public: match item_struct.vis {
            Visibility::Public(_) => true,
            _ => false,
        },
        item_type: UmlItemType::Struct,
        fields: fields,
    }
}
fn parse_fields_to_uml(item_fields: Fields) -> Vec<UmlField> {
    match item_fields {
        Fields::Named(FieldsNamed {
            named: named_fields,
            ..
        }) => named_fields.into_iter().map(parse_field_to_uml).collect(),
        // empty vector otherwise
        _ => vec![],
    }
}
fn parse_field_to_uml(field: Field) -> UmlField {
    UmlField {
        name: field
            .ident
            .map(|field_ident| field_ident.to_string())
            .unwrap_or_else(|| "".to_string()),
        is_public: match field.vis {
            Visibility::Public(_) => true,
            _ => false,
        },
        field_type: "TODO".to_string(),
    }
}

#[derive(Default)]
pub struct RustVisitor<'ast> {
    // pub enums: Vec<&'ast ItemEnum>,
    // pub impls: Vec<&'ast ItemImpl>,
    pub structs: Vec<&'ast ItemStruct>,
    pub uml_structs: Vec<UmlItem>,
    // pub traits: Vec<&'ast ItemTrait>,
}

impl<'ast> Visit<'ast> for RustVisitor<'ast> {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        println!("struct: {:?}", i.ident);
        self.structs.push(i);
        let uml_struct = parse_struct_to_uml(i);
        println!("uml_struct: {:?}", uml_struct);
        self.uml_structs.push(uml_struct);
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
