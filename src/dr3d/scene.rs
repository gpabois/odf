use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::DR3D_NS",
    name = "scene"
)]
#[derive(Default)]
pub struct Scene {}
