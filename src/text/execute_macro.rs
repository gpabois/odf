use odf_macros::define_element;


#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "execute-macro"
)]
#[derive(Default)]
pub struct ExecuteMacro {}