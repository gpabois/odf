use crate::element::OpenDocumentElement;

pub fn find_child_element<'a, C: OpenDocumentElement>(parent: &'a minidom::Element) -> Option<&'a minidom::Element>
{
    parent.children().find(|cel| C::is_element(cel))
}

pub struct IntoAttributeValue<T>(std::marker::PhantomData<T>);

impl IntoAttributeValue<bool>
{
    fn into_attribute_value(value: bool) -> Option<String>
    {
        if value {
            Some("true".into())
        } else {
            Some("false".into())
        }
    }
}

impl IntoAttributeValue<String>
{
    fn into_attribute_value(value: String) -> Option<String>
    {
        Some(value)
    }
}

