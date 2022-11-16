use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "notes-configuration"
)]
#[derive(Default)]
pub struct NotesConfiguration {}