use super::{text::Text, chart::Chart, database::Database, drawing::Drawing, presentation::Presentation, spreadsheet::Spreadsheet, image::Image};

use odf_macros::{define_child_elements, define_element};
use crate::ns::OFFICE_NS;

#[define_element(
    namespace = "OFFICE_NS",
    name = "body",
    children="BodyChildElement"
)]
#[derive(Default)]
pub struct Body {}

impl Body 
{
    pub fn add(&mut self, child: impl Into<BodyChildElement>)
    {
        self.children.push(child.into())
    }
}

#[define_child_elements(Text, Database, Chart, Drawing, Image, Presentation, Spreadsheet)]
pub enum BodyChildElement {}
