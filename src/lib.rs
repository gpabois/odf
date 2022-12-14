extern crate odf_macros;

pub mod element;
pub mod office;
pub mod manifest;
pub mod draw;
pub mod dr3d;
pub mod dc;
pub mod meta;
pub mod table;
pub mod text;
pub mod ns;
pub mod utils;
pub mod presentation;
pub mod chart;
pub mod config;
pub mod style;
pub mod number;
pub mod svg;
pub mod form;

use minidom::Element;

use office::DocumentContent;
use office::DocumentStyles;
use office::DocumentMeta;
use office::DocumentSettings;

#[derive(Default)]
pub struct OpenDocument {
    content: DocumentContent,
    styles: DocumentStyles,
    meta: DocumentMeta,
    settings: DocumentSettings
}

#[derive(Debug)]
pub enum Error
{
    IOError(std::io::Error),
    ZipError(zip::result::ZipError),
    XMLError(minidom::error::Error),
    ParsingError(ParsingError),
}

#[derive(Debug)]
pub enum ParsingError
{
    MissingChildElement(String)
}

impl ParsingError {
    pub fn missing_element(child_type: String) -> Self {
        Self::MissingChildElement(child_type)
    }
}

impl From<ParsingError> for Error
{
    fn from(e: ParsingError) -> Self {
        Error::ParsingError(e)
    }
}

impl From<std::io::Error> for Error
{
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<zip::result::ZipError> for Error
{
    fn from(error: zip::result::ZipError) -> Self {
        Self::ZipError(error)
    }
}

impl From<minidom::error::Error> for Error 
{
    fn from(error: minidom::error::Error) -> Self {
        Self::XMLError(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl OpenDocument {
    pub fn save(self, w: impl std::io::Write + std::io::Seek) -> Result<()>
    {
        let mut zip = zip::ZipWriter::new(w);
        
        // Write content.xml
        {
            let content_element: Element = self.content.into();
            zip.start_file("content.xml", Default::default()).map_err(Error::from)?;
            content_element.write_to(&mut zip).map_err(Error::from)?;
        }

        // Write styles.xml
        {
            let styles_element: Element = self.styles.into();
            zip.start_file("styles.xml", Default::default()).map_err(Error::from)?;
            styles_element.write_to(&mut zip).map_err(Error::from)?;
        }

        // Write meta.xml
        {
            let meta_element: Element = self.meta.into();
            zip.start_file("meta.xml", Default::default()).map_err(Error::from)?;
            meta_element.write_to(&mut zip).map_err(Error::from)?;

        }

        // Write settings.xml
        {
            let settings_element: Element = self.settings.into();
            zip.start_file("settings.xml", Default::default()).map_err(Error::from)?;
            settings_element.write_to(&mut zip).map_err(Error::from)?;
        }

        // Flush everything
        zip.finish().map_err(Error::from)?;

        Ok(())
    }

    pub fn save_to_file(
        self, path: impl Into<std::path::PathBuf>) -> Result<()>
    {
        let file = std::fs::File::create(path.into()).unwrap();   
        self.save(file)   
    }
}

#[cfg(test)]
mod tests {
    use super::{OpenDocument, Result};

    #[test]
    fn test_save_file() -> Result<()>
    {
        let doc: OpenDocument = Default::default();
        doc.save_to_file("tests/test_00.odt")
    }

    
}