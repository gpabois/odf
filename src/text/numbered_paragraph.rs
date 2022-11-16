use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "numbered-paragraph"
)]
#[derive(Default)]
pub struct NumberedParagraph {}