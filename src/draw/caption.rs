use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::DRAWING_NS",
    name = "caption"
)]
#[derive(Default)]
pub struct Caption {}