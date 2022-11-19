use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "line-break"
)]
#[derive(Default)]
pub struct LineBreak {}