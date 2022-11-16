use odf_macros::{define_element, define_child_elements};
use crate::{draw, dr3d, office, presentation, text};

#[define_element(
    namespace = "crate::ns::TEXT_NS",
    name = "h",
    children="HChildElement",
    attribute(name="class-names", r#type="String", prefix = "text"),
    attribute(name="cond-style-name", r#type="String", prefix = "text"),
    attribute(name="id", r#type="String", prefix="text"),
    attribute(name="is-list-header", r#type="String", prefix="text"),
    attribute(name="outline-level", r#type="String", prefix="text"),
    attribute(name="restart-numbering", r#type="String", prefix="text"),
    attribute(name="start-value", r#type="String", prefix="text"),
    attribute(name="style-name", r#type="String", prefix="text"),
    attribute(name="about", r#type="String", prefix="xhtml"),
    attribute(name="content", r#type="String", prefix="xhtml"),
    attribute(name="datatype", r#type="String", prefix="xhtml"),
    attribute(name="property", r#type="String", prefix="xhtml"),
    attribute(name="id", r#type="String", prefix="xhtml")
)]
#[derive(Default)]
pub struct H {}

#[define_child_elements(
    dr3d::Scene,
    draw::A, draw::Caption, draw::Circle,
    draw::Connector, draw::Control, draw::CustomShape,
    draw::Ellipse, draw::Frame, draw::G,
    draw::Line, draw::Measure, draw::PageThumbnail,
    draw::Path, draw::Polygon, draw::Polyline, draw::Rect,
    draw::RegularPolygon, office::Annotation, office::AnnotationEnd,
    presentation::DateTime, presentation::Footer, presentation::Header,
    text::A, text::AlphabeticalIndexMark, text::AlphabeticalIndexMarkEnd,
    text::AuthorInitials, text::AuthorName, text::BibliographyMark,
    text::Bookmark, text::BookmarkEnd, text::BookmarkRef, text::BookmarkStart,
    text::Change, text::ChangeEnd, text::ChangeStart,
    text::Chapter, text::CharacterCount, text::ConditionalText,
    text::CreationDate, text::CreationTime, text::Creator,
    text::DatabaseDisplay, text::DatabaseName, text::DatabaseNext,
    text::DatabaseRowNumber, text::DatabaseRowSelect, text::Date,
    text::DdeConnection, text::Description
)]
pub enum HChildElement{}
