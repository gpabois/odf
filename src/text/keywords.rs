use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "keywords"
)]
#[derive(Default)]
pub struct Keywords {}