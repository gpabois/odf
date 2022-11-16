use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "database-name"
)]
#[derive(Default)]
pub struct DatabaseName {}
