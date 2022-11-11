use minidom::Element;
use odf_macros::define_element;

use crate::element::OpenDocumentElement;
use crate::ns::DRAWING_NS;

#[define_element(
    namespace = "DRAWING_NS",
    name = "a"
)]
#[derive(Default)]
pub struct A {}

#[define_element(
    namespace = "DRAWING_NS",
    name = "caption"
)]
#[derive(Default)]
pub struct Caption {}


#[define_element(
    namespace = "DRAWING_NS",
    name = "circle"
)]
#[derive(Default)]
pub struct Circle {}

#[define_element(
    namespace = "DRAWING_NS",
    name = "connector"
)]
#[derive(Default)]
pub struct Connector {}

#[define_element(
    namespace = "DRAWING_NS",
    name = "control"
)]
#[derive(Default)]
pub struct Control {}

#[define_element(
    namespace = "DRAWING_NS",
    name = "custom-shape"
)]
#[derive(Default)]
pub struct CustomShape {}

#[define_element(
    namespace = "DRAWING_NS",
    name = "ellipse"
)]
#[derive(Default)]
pub struct Ellipse {}

#[define_element(
    namespace = "DRAWING_NS",
    name = "frame"
)]
#[derive(Default)]
pub struct Frame {}

#[define_element(
    namespace = "DRAWING_NS",
    name = "g"
)]
#[derive(Default)]
pub struct G {}

#[define_element(
    namespace = "DRAWING_NS",
    name = "line"
)]
#[derive(Default)]
pub struct Line {}

#[define_element(
    namespace = "DRAWING_NS",
    name = "measure"
)]
#[derive(Default)]
pub struct Measure {}

#[define_element(
    namespace = "DRAWING_NS",
    name = "page-thumbnail"
)]
#[derive(Default)]
pub struct PageThumbnail {}

#[define_element(
    namespace = "DRAWING_NS",
    name = "path"
)]
#[derive(Default)]
pub struct Path {}

#[define_element(
    namespace = "DRAWING_NS",
    name = "polygon"
)]
#[derive(Default)]
pub struct Polygon {}

#[define_element(
    namespace = "DRAWING_NS",
    name = "polyline"
)]
#[derive(Default)]
pub struct Polyline {}

#[define_element(
    namespace = "DRAWING_NS",
    name = "rect"
)]
#[derive(Default)]
pub struct Rect {}

#[define_element(
    namespace = "DRAWING_NS",
    name = "regular-polygon"
)]
#[derive(Default)]
pub struct RegularPolygon {}