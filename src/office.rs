pub mod document_content;
pub mod document_styles;
pub mod document_meta;
pub mod document_settings;

pub mod body;
pub mod automatic_styles;
pub mod styles;
pub mod master_styles;
pub mod font_face_decls;
pub mod scripts;
pub mod meta;
pub mod settings;

pub mod text;
pub mod chart;
pub mod database;
pub mod drawing;
pub mod image;
pub mod presentation;
pub mod spreadsheet;

pub use document_content::DocumentContent;
pub use document_meta::DocumentMeta;
pub use document_settings::DocumentSettings;
pub use document_styles::DocumentStyles;