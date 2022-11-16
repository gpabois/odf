use odf_macros::define_element;
use crate::draw;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "image",
    children = "draw::Frame"
)]
#[derive(Default)]
pub struct Image {}