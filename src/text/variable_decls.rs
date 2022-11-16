use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "variable-decls"
)]
#[derive(Default)]
pub struct VariableDecls {}
