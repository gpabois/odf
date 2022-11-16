use odf_macros::{ define_element};


#[define_element(
    namespace = "crate::ns::NUMBER_NS",
    name = "currency-style",
)]
#[derive(Default)]
pub struct CurrencyStyle {}