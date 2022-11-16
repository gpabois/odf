use super::meta::Meta;
use odf_macros::define_element;
use crate::ns::OFFICE_NS;
use std::str::FromStr;

#[define_element(
    setup_prefixes="true",
    namespace = "OFFICE_NS",
    name="meta",
    attribute(name="transformation", r#type="String", prefix="grddl"),
    attribute(name="version", r#type="String", prefix="office"),
    child(r#type="Meta")
)]
#[derive(Default)]
pub struct DocumentMeta {}