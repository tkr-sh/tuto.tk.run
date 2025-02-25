use {
    crate::shared::wini::err::ServerError,
    axum::extract::Request,
    strum_macros::{EnumIter, EnumString},
};

#[derive(Debug, EnumIter, strum::Display)]
pub(crate) enum Language {
    #[strum(to_string = "en")]
    English,
    // #[strum(to_string = "es")]
    #[strum(to_string = "en")]
    Spanish,
    // #[strum(to_string = "pt")]
    #[strum(to_string = "en")]
    Portugese,
    // #[strum(to_string = "fr")]
    #[strum(to_string = "en")]
    French,
    // #[strum(to_string = "de")]
    #[strum(to_string = "en")]
    Deustch,
}

impl TryFrom<&Request> for Language {
    type Error = ServerError;

    fn try_from(req: &Request) -> Result<Self, Self::Error> {
        use std::str::FromStr;

        crate::utils::language::Language::from_str(match req.uri().authority() {
            Some(auth) => auth.host().split(".").next().unwrap_or(""),
            None => "",
        })
    }
}

impl std::str::FromStr for Language {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "" | "tuto" | "tk" | "en" => Language::English,
            "es" => Language::Spanish,
            "pt" => Language::Portugese,
            "fr" => Language::French,
            "de" => Language::Deustch,
            _ => return Err(ServerError::DebugedError(String::from("Unknown language."))),
        })
    }
}
