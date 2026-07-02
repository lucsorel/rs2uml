#[derive(Debug)]
pub enum UmlItemType {
    Enum,
    Impl,
    Struct,
    Trait,
}

#[derive(Debug)]
pub struct UmlField {
    pub name: String,
    pub is_public: bool,
    pub field_type: String,
}

#[derive(Debug)]
pub struct UmlItem {
    pub name: String,
    pub is_public: bool,
    pub item_type: UmlItemType,
    // pub fqn: String,
    pub fields: Vec<UmlField>,
}
