use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "styles"
)]
#[derive(Default)]
pub struct Styles {}