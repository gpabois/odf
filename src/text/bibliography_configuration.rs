use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "bibliography-configuration"
)]
#[derive(Default)]
pub struct BibliographyConfiguration{}
