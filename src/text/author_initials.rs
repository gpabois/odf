use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "author-initials"
)]
#[derive(Default)]
pub struct AuthorInitials {}
