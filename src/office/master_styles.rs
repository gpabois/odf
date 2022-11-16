use odf_macros::{define_element, define_child_elements};
use crate::{draw, style};

#[define_element(
    namespace = "crate::ns::OFFICE_NS",
    name = "master-styles"
)]
#[derive(Default)]
pub struct MasterStyles {}

#[define_child_elements(
    draw::LayerSet,
    style::HandoutMaster,
    style::MasterPage
)]
pub enum MasterStylesChildElement{}