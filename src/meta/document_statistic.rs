use odf_macros::define_element;

#[define_element(
    namespace = "crate::ns::META_NS",
    name = "document-statistic",
    attribute(name = "cell-count", prefix="meta", r#type="String"),
    attribute(name = "character-count", prefix="meta", r#type="String"),
    attribute(name = "draw-count", prefix="meta", r#type="String"),
    attribute(name = "frame-count", prefix="meta", r#type="String"),
    attribute(name = "image-count", prefix="meta", r#type="String"),
    attribute(name = "non-whitespace-character-count", prefix="meta", r#type="String"),
    attribute(name = "object-count", prefix="meta", r#type="String"),
    attribute(name = "ole-object-count", prefix="meta", r#type="String"),
    attribute(name = "page-count", prefix="meta", r#type="String"),
    attribute(name = "paragraph-count", prefix="meta", r#type="String"),
    attribute(name = "sentence-count", prefix="meta", r#type="String"),
    attribute(name = "syllable-count", prefix="meta", r#type="String"),
    attribute(name = "table-count", prefix="meta", r#type="String"),
    attribute(name = "word-count", prefix = "meta", r#type="String")
)]
#[derive(Default)]
pub struct DocumentStatistic {}