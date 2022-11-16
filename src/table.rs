use odf_macros::{ define_element};
use crate::ns::TABLE_NS;

#[define_element(
    namespace = "TABLE_NS",
    name = "table",
)]
#[derive(Default)]
pub struct Table {}

#[define_element(
    namespace = "TABLE_NS",
    name = "calculation-settings",
)]
#[derive(Default)]
pub struct CalculationSettings {}

#[define_element(
    namespace = "TABLE_NS",
    name = "consolidation",
)]
#[derive(Default)]
pub struct Consolidation {}

#[define_element(
    namespace = "TABLE_NS",
    name = "content-validations",
)]
#[derive(Default)]
pub struct ContentValidations {}


#[define_element(
    namespace = "TABLE_NS",
    name = "data-pilot-tables",
)]
#[derive(Default)]
pub struct DataPilotTables {}


#[define_element(
    namespace = "TABLE_NS",
    name = "database-ranges",
)]
#[derive(Default)]
pub struct DatabaseRanges {}

#[define_element(
    namespace = "TABLE_NS",
    name = "dde-links",
)]
#[derive(Default)]
pub struct DdeLinks {}

#[define_element(
    namespace = "TABLE_NS",
    name = "label-ranges",
)]
#[derive(Default)]
pub struct LabelRanges {}

#[define_element(
    namespace = "TABLE_NS",
    name = "named-expressions",
)]
#[derive(Default)]
pub struct NamedExpressions {}

#[define_element(
    namespace = "TABLE_NS",
    name = "tracked-changes",
)]
#[derive(Default)]
pub struct TrackedChanges {}

#[define_element(
    namespace = "TABLE_NS",
    name = "table-template",
)]
#[derive(Default)]
pub struct TableTemplate{}
