use super::{text::Text, chart::Chart, database::Database, drawing::Drawing, presentation::Presentation, spreadsheet::Spreadsheet, image::Image};

#[derive(Default)]
pub struct Body {
    chidren: Vec<BodyChildElement>
}

impl Body 
{
    fn add(&mut self, child: impl Into<BodyChildElement>)
    {
        self.chidren.push(child.into())
    }
}

pub enum BodyChildElement
{
    Text(Text),
    Chart(Chart),
    Database(Database),
    Drawing(Drawing),
    Image(Image),
    Presentation(Presentation),
    Spreadsheet(Spreadsheet)
}

impl From<Text> for BodyChildElement
{
    fn from(t: Text) -> Self {
        Self::Text(t)
    }
}

impl From<Chart> for BodyChildElement
{
    fn from(c: Chart) -> Self {
        Self::Chart(c)
    }
}

impl From<Database> for BodyChildElement
{
    fn from(d: Database) -> Self {
        Self::Database(d)
    }
}

impl From<Drawing> for BodyChildElement
{
    fn from(d: Drawing) -> Self {
        Self::Drawing(d)
    }
}

impl From<Image> for BodyChildElement
{
    fn from(i: Image) -> Self {
        Self::Image(i)
    }
}

impl From<Presentation> for BodyChildElement 
{
    fn from(p: Presentation) -> Self {
        Self::Presentation(p)
    }
}