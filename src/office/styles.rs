use odf_macros::{define_element, define_child_elements};

use crate::draw;
use crate::number;
use crate::style;
use crate::svg;
use crate::table;
use crate::text;

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "styles",
    children = "StylesChildElement"
)]
#[derive(Default)]
pub struct Styles {}

#[define_child_elements(
    draw::FillImage,
    draw::Gradient,
    draw::Hatch,
    draw::Marker,
    draw::Opacity,
    draw::StrokeDash,
    number::BooleanStyle,
    number::CurrencyStyle,
    number::DateStyle,
    number::NumberStyle,
    number::PercentageStyle,
    number::TextStyle,
    number::TimeStyle,
    style::DefaultPageLayout,
    style::PresentationPageLayout,
    style::DefaultStyle,
    style::Style,
    svg::LinearGradient,
    svg::RadialGradient,
    table::TableTemplate,
    text::BibliographyConfiguration,
    text::LinenumberingConfiguration,
    text::ListStyle,
    text::NotesConfiguration,
    text::OutlineStyle
)]
pub enum StylesChildElement{}
