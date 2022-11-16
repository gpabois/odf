use crate::element::OpenDocumentElement;

pub fn find_child_element<'a, C: OpenDocumentElement>(parent: &'a minidom::Element) -> Option<&'a minidom::Element>
{
    parent.children().find(|cel| C::is_element(cel))
}

pub struct FromAttributeValue<T>(std::marker::PhantomData<T>);

impl FromAttributeValue<bool>
{
    pub fn from_attribute_value(value: impl Into<String>) -> bool
    {
        if value.into() == "true" {
            return true;
        }

        return false;
    }
}

impl FromAttributeValue<String>
{
    pub fn from_attribute_value(value: impl Into<String>) -> String
    {
        value.into()
    }
}


pub struct IntoAttributeValue<T>(std::marker::PhantomData<T>);

impl IntoAttributeValue<bool>
{
    pub fn into_attribute_value(value: bool) -> Option<String>
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
    pub fn into_attribute_value(value: String) -> Option<String>
    {
        Some(value)
    }
}

