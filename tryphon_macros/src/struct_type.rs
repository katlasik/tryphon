use syn::Field;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum StructType {
    Unit,
    Named,
    Tuple,
}
impl StructType {
    pub(crate) fn from_fields(fields: &Vec<&Field>) -> StructType {
        match fields.iter().next().map(|f| &f.ident) {
            None => StructType::Unit,
            Some(Some(_)) => StructType::Named,
            Some(None) => StructType::Tuple,
        }
    }
}
