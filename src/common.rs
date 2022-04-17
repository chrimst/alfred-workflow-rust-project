// trait for enum to get ident key
pub trait EnumIdent {
    fn name(&self) -> &'static str;
}


