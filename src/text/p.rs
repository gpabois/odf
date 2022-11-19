use odf_macros::{define_element, define_child_elements};
use crate::{chart, draw, form, office, style, table, text};

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "p"
)]
#[derive(Default)]
pub struct P {}


#[define_child_elements(
    chart::DataLabel, chart::Equation, chart::Footer,
    chart::LabelSeperator, chart::Legend, chart::Subtitle, chart::Title,
    draw::Caption, draw::Circle, draw::Connector, draw::CustomShape,
    draw::Ellipse, draw::Image, draw::Line, draw::Measure, draw::Path, draw::Polygon,
    draw::Polyline, draw::Rect, draw::RegularPolygon, draw::TextBox, form::Textarea,
    office::Annotation, office::ChangeInfo, office::Text
)]
pub enum PChildElement{}