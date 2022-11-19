use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::FORM_NS",
    name = "textarea"
)]
#[derive(Default)]
pub struct Textarea {}
