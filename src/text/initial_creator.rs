use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "initial-creator"
)]
#[derive(Default)]
pub struct InitialCreator {}