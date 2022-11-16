use odf_macros::{ define_element};


#[define_element(
    namespace = "crate::ns::NUMBER_NS",
    name = "date-style",
)]
#[derive(Default)]
pub struct DateStyle {}