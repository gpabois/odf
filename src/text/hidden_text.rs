use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "hidden-text"
)]
#[derive(Default)]
pub struct HiddenText {}