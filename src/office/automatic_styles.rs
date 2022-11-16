use odf_macros::{define_element, define_child_elements};
use crate::{number, style, text};

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "automatic-styles"
)]
#[derive(Default)]
pub struct AutomaticStyles {}

#[define_child_elements(
    number::BooleanStyle,
    number::CurrencyStyle,
    number::DateStyle,
    number::NumberStyle,
    number::PercentageStyle,
    number::TextStyle,
    number::TimeStyle,
    style::PageLayout,
    style::Style,
    text::ListStyle
)]
pub enum AutomaticStylesChildElement{}