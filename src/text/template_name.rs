use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "template-name"
)]
#[derive(Default)]
pub struct TemplateName {}