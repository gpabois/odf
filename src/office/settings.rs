
use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name="settings"
)]
#[derive(Default)]
pub struct Settings {}