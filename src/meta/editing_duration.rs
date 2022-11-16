use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::META_NS",
    name = "editing-duration"
)]
#[derive(Default)]
pub struct EditingDuration {}