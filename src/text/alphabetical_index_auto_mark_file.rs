use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "alphabetical-index-auto-mark-file"
)]
#[derive(Default)]
pub struct AlphabeticalIndexAutoMarkFile {}