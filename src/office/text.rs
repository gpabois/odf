use crate::{dr3d::Scene, draw::{A, Caption, Circle, Connector, Control, CustomShape, Ellipse, Frame, G, Line, Measure, PageThumbnail, Path, Polygon, Polyline, Rect, RegularPolygon}, table::{CalculationSettings, Consolidation, ContentValidations, DataPilotTables, DatabasesRanges, DdeLinks, LabelRanges, NamedExpressions, Table}, text::{AlphabeticalIndex, AlphabeticalIndexAutoMarkFile, Bibliography, Change, ChangeEnd, ChangeStart, DdeConnectionDecls, H, IllustrationIndex, List, NumberedParagraph, ObjectIndex, P, PageSequence, Section, SequenceDecls, SoftPageBreak, TableIndex, TableOfContent, TrackedChanges, UserFieldDecls, VariableDecls, UserIndex}};

#[derive(Default)]
pub struct Text{
    children: Vec<TextChildElement>
}

pub enum TextChildElement
{
    Dr3dScene(Scene),
    DrawA(A),
    DrawCaption(Caption),
    DrawCircle(Circle),
    DrawConnector(Connector),
    DrawControl(Control),
    DrawCustomShape(CustomShape),
    DrawEllipse(Ellipse),
    DrawFrame(Frame),
    DrawG(G),
    DrawLine(Line),
    DrawMeasure(Measure),
    DrawPageThumbnail(PageThumbnail),
    DrawPath(Path),
    DrawPolygon(Polygon),
    DrawPolyline(Polyline),
    DrawRect(Rect),
    DrawRegularPolygon(RegularPolygon),
    // office:forms
    TableCalculationSettings(CalculationSettings),
    TableConsolidation(Consolidation),
    TableContentValidations(ContentValidations),
    TableDataPilotTables(DataPilotTables),
    TableDatabaseRanges(DatabasesRanges),
    TableDdeLinks(DdeLinks),
    TableLabelRanges(LabelRanges),
    TableNamedExpressions(NamedExpressions),
    Table(Table),
    AlphabeticalIndex(AlphabeticalIndex),
    AlphabeticalIndexAutoMarkFile(AlphabeticalIndexAutoMarkFile),
    Bibliography(Bibliography),
    Change(Change),
    ChangeEnd(ChangeEnd),
    ChangeStart(ChangeStart),
    DdeConnectionDecls(DdeConnectionDecls),
    H(H),
    IllustrationIndex(IllustrationIndex),
    List(List),
    NumberedParagraph(NumberedParagraph),
    ObjectIndex(ObjectIndex),
    P(P),
    PageSequence(PageSequence),
    Section(Section),
    SequenceDecls(SequenceDecls),
    SoftPageBreak(SoftPageBreak),
    TableIndex(TableIndex),
    TableOfContent(TableOfContent),
    TrackedChanges(TrackedChanges),
    UserFieldDecls(UserFieldDecls),
    UserIndex(UserIndex),
    VariableDecls(VariableDecls)
}