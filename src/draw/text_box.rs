use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::DRAWING_NS",
    name = "text-box"
)]
#[derive(Default)]
pub struct TextBox {}