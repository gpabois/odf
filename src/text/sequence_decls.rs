use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "sequence-decls"
)]

#[derive(Default)]
pub struct SequenceDecls {}