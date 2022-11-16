use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "event-listeners",
)]
#[derive(Default)]
pub struct EventListeners {}