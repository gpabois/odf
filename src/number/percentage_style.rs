use odf_macros::{ define_element};


#[define_element(
    namespace = "crate::ns::NUMBER_NS",
    name = "percentage-style",
)]
#[derive(Default)]
pub struct PercentageStyle {}