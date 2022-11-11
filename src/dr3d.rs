use minidom::Element;
use odf_macros::define_element;

use crate::element::OpenDocumentElement;
use crate::ns::DR3D_NS;

#[define_element(
    namespace = "DR3D_NS",
    name = "scene"
)]
#[derive(Default)]
pub struct Scene {}
