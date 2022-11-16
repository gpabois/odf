use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "linenumbering-configuration"
)]
#[derive(Default)]
pub struct LinenumberingConfiguration {}