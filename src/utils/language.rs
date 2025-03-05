use {
    crate::shared::wini::err::ServerError,
    axum::extract::Request,
    strum_macros::{EnumIter, EnumString},
};

#[derive(Debug, EnumIter, strum::Display, Hash, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Language {
    #[strum(to_string = "en")]
    English,
    #[strum(to_string = "es")]
    Spanish,
    #[strum(to_string = "pt")]
    Portugese,
    #[strum(to_string = "fr")]
    French,
    // #[strum(to_string = "de")]
    #[strum(to_string = "en")]
    Deustch,
}

impl TryFrom<&Request> for Language {
    type Error = ServerError;

    fn try_from(req: &Request) -> Result<Self, Self::Error> {
        use std::str::FromStr;

        crate::utils::language::Language::from_str(match req.headers().get("host") {
            Some(host) => {
                host.to_str()
                    .ok()
                    .and_then(|str| str.split(".").next())
                    .unwrap_or("")
            },
            None => "",
        })
    }
}

impl std::str::FromStr for Language {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "" | "tuto" | "tk" | "en" | "localhost:10010" => Language::English,
            "es" => Language::Spanish,
            "pt" => Language::Portugese,
            "fr" => Language::French,
            "de" => Language::Deustch,
            _ => return Err(ServerError::DebugedError(String::from("Unknown language."))),
        })
    }
}
