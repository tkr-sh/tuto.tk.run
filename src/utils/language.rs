use {crate::shared::wini::err::ServerError, axum::extract::Request};

pub(crate) enum Language {
    English,
    Spanish,
    Portugese,
    French,
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
