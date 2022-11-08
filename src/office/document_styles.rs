use super::{automatic_styles::AutomaticStyles, master_styles::MasterStyles, styles::Styles};

#[derive(Default)]
pub struct DocumentStyles
{
    automatic_styles: AutomaticStyles,
    master_styles: MasterStyles,
    styles: Styles
}