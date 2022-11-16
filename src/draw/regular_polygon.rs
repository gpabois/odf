use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::DRAWING_NS",
    name = "regular-polygon"
)]
#[derive(Default)]
pub struct RegularPolygon {}