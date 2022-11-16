use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "tracked-changes"
)]
#[derive(Default)]
pub struct TrackedChanges {}