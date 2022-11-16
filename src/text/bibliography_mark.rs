use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "bibliography-mark"
)]
#[derive(Default)]
pub struct BibliographyMark {}
