pub trait OpenDocumentElement: Into<minidom::Element>
{
    fn is_element(el: &minidom::Element) -> bool;
    fn from_element(el: &minidom::Element) -> crate::Result<Self>;
}

pub trait OpenDocumentElementWithChildren<T> {
    fn add_child(&mut self, child: impl Into<T>);
}