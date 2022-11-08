use minidom::Element;
use super::ns::OFFICE_NS;

#[derive(Default)]
pub struct FontFaceDecls
{

}

impl Into<Element> for &FontFaceDecls
{
    fn into(self) -> Element
    {
        Element::builder("font-face-decls", OFFICE_NS)
        .build()
    }
}
