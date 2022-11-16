use odf_macros::define_element;
use crate::ns::TEXT_NS;

#[define_element(
    namespace = "TEXT_NS",
    name = "alphabetical-index"
)]
#[derive(Default)]
pub struct AlphabeticalIndex {}


#[define_element(
    namespace = "TEXT_NS",
    name = "alphabetical-index-auto-mark-file"
)]
#[derive(Default)]
pub struct AlphabeticalIndexAutoMarkFile {}

#[define_element(
    namespace = "TEXT_NS",
    name = "bibliography"
)]
#[derive(Default)]
pub struct Bibliography {}

#[define_element(
    namespace = "TEXT_NS",
    name = "bibliography-configuration"
)]
#[derive(Default)]
pub struct BibliographyConfiguration{}

#[define_element(
    namespace = "TEXT_NS",
    name = "change"
)]
#[derive(Default)]
pub struct Change {}

#[define_element(
    namespace = "TEXT_NS",
    name = "change-end"
)]
#[derive(Default)]
pub struct ChangeEnd {}

#[define_element(
    namespace = "TEXT_NS",
    name = "change-start"
)]
#[derive(Default)]
pub struct ChangeStart {}


#[define_element(
    namespace = "TEXT_NS",
    name = "dde-connection-decls"
)]
#[derive(Default)]
pub struct DdeConnectionDecls {}

#[define_element(
    namespace = "TEXT_NS",
    name = "h"
)]
#[derive(Default)]
pub struct H {}

#[define_element(
    namespace = "TEXT_NS",
    name = "illustration-index"
)]
#[derive(Default)]
pub struct IllustrationIndex {}

#[define_element(
    namespace = "TEXT_NS",
    name = "list"
)]
#[derive(Default)]
pub struct List {}

#[define_element(
    namespace = "TEXT_NS",
    name = "numbered-paragraph"
)]
#[derive(Default)]
pub struct NumberedParagraph {}

#[define_element(
    namespace = "TEXT_NS",
    name = "object-index"
)]

#[derive(Default)]
pub struct ObjectIndex {}

#[define_element(
    namespace = "TEXT_NS",
    name = "p"
)]
#[derive(Default)]
pub struct P {}

#[define_element(
    namespace = "TEXT_NS",
    name = "page-sequence"
)]
#[derive(Default)]
pub struct PageSequence {}

#[define_element(
    namespace = "TEXT_NS",
    name = "section"
)]
#[derive(Default)]
pub struct Section {}

#[define_element(
    namespace = "TEXT_NS",
    name = "sequence-decls"
)]

#[derive(Default)]
pub struct SequenceDecls {}

#[define_element(
    namespace = "TEXT_NS",
    name = "soft-page-break"
)]

#[derive(Default)]
pub struct SoftPageBreak {}

#[define_element(
    namespace = "TEXT_NS",
    name = "table-index"
)]
#[derive(Default)]
pub struct TableIndex {}

#[define_element(
    namespace = "TEXT_NS",
    name = "table-of-content"
)]
#[derive(Default)]
pub struct TableOfContent {}

#[define_element(
    namespace = "TEXT_NS",
    name = "table"
)]
#[derive(Default)]
pub struct Table {}

#[define_element(
    namespace = "TEXT_NS",
    name = "tracked-changes"
)]
#[derive(Default)]
pub struct TrackedChanges {}

#[define_element(
    namespace = "TEXT_NS",
    name = "user-field-decls"
)]
#[derive(Default)]
pub struct UserFieldDecls {}

#[define_element(
    namespace = "TEXT_NS",
    name = "user-index"
)]
#[derive(Default)]
pub struct UserIndex {}

#[define_element(
    namespace = "TEXT_NS",
    name = "variable-decls"
)]
#[derive(Default)]
pub struct VariableDecls {}

#[define_element(
    namespace = "TEXT_NS",
    name = "linenumbering-configuration"
)]
#[derive(Default)]
pub struct LinenumberingConfiguration {}

#[define_element(
    namespace = "TEXT_NS",
    name = "list-style"
)]
#[derive(Default)]
pub struct ListStyle {}

#[define_element(
    namespace = "TEXT_NS",
    name = "notes-configuration"
)]
#[derive(Default)]
pub struct NotesConfiguration {}

#[define_element(
    namespace = "TEXT_NS",
    name = "outline-style"
)]
#[derive(Default)]
pub struct OutlineStyle {}
