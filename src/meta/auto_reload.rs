use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::META_NS",
    name = "auto-reload"
)]
#[derive(Default)]
pub struct AutoReload {}