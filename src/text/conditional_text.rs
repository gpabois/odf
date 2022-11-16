use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "conditional-text"
)]
#[derive(Default)]
pub struct ConditionalText {}
