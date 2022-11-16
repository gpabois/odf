use odf_macros::{ define_element};


#[define_element(
    namespace = "crate::ns::STYLE_NS",
    name = "handout-master",
)]
#[derive(Default)]
pub struct HandoutMaster {}