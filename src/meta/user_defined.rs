use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::META_NS",
    name = "user-defined",
    attribute(name = "name", prefix = "meta", r#type="String"),
    attribute(name = "value-type", prefix = "meta", r#type="String"),
    content = "String"
)]
#[derive(Default)]
pub struct UserDefined {}