use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "note-ref"
)]
#[derive(Default)]
pub struct NoteRef {}