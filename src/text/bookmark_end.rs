use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "bookmark-end"
)]
#[derive(Default)]
pub struct BookmarkEnd {}
