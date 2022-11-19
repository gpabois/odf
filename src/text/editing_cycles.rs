use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "editing-cycles"
)]
#[derive(Default)]
pub struct EditingCycles {}