use super::meta::Meta;
use odf_macros::define_element;
use crate::ns::OFFICE_NS;

#[define_element(
    namespace = "OFFICE_NS",
    name="meta",
    child(r#type="Meta")
)]
#[derive(Default)]
pub struct DocumentMeta
{
    //meta: Meta
}