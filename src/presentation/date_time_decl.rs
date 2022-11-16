use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::PRESENTATION_NS",
    name = "date-time-decl"
)]
#[derive(Default)]
pub struct DateTimeDecl {}
