use crate::{dc::{Creator, Date, Description, Subject, Language}, meta::{AutoReload, CreationDate, DocumentStatistic, EditingCycles, EditingDuration, Generator, HyperlinkBehaviour, InitialCreator, PrintDate, PrintedBy, Template, UserDefined, Keyword}};

#[derive(Default)]
pub struct Meta {
    children: Vec<MetaChildElement>
}

pub enum MetaChildElement {
    Creator(Creator),
    Date(Date),
    Description(Description),
    Subject(Subject),
    Language(Language),
    AutoReload(AutoReload),
    CreationDate(CreationDate),
    DocumentStatistic(DocumentStatistic),
    EditingCycles(EditingCycles),
    EditingDuration(EditingDuration),
    Generator(Generator),
    HyperlinkBehaviour(HyperlinkBehaviour),
    InitialCreator(InitialCreator),
    Keyword(Keyword),
    PrintDate(PrintDate),
    PrintedBy(PrintedBy),
    Template(Template),
    UserDefined(UserDefined)
}