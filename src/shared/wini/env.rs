use {
    serde::Deserialize,
    strum::{EnumIter, EnumString},
};


/// The type of environment that the server is running on
#[derive(
    Debug, Clone, Copy, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, EnumString, EnumIter,
)]
#[serde(rename_all = "lowercase")]
#[strum(ascii_case_insensitive)]
pub enum EnvType {
    Prod,
    Staging,
    Dev,
    Local,
}
