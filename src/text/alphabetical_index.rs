use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "alphabetical-index"
)]
#[derive(Default)]
pub struct AlphabeticalIndex {}