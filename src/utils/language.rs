use {
    crate::shared::wini::err::{ServerError, ServerErrorKind},
    axum::extract::Request,
    strum::{EnumIter, EnumString},
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
    #[strum(to_string = "zh")]
    Mandarin,
    #[strum(to_string = "ja")]
    Japanese,
    #[strum(to_string = "ko")]
    Korean,
    #[strum(to_string = "eo")]
    Esperanto,
    #[strum(to_string = "ar")]
    Arabic,
    #[strum(to_string = "ru")]
    Russian,
}

impl Language {
    pub(crate) fn from_path(path: &str) -> Option<Self> {
        let segments = path
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect::<Vec<_>>();
        let lua_index = segments.iter().position(|segment| *segment == "lua")?;
        let language = segments.get(lua_index.checked_sub(1)?)?;

        Some(match *language {
            "en" => Language::English,
            "es" => Language::Spanish,
            "pt" => Language::Portugese,
            "fr" => Language::French,
            "de" => Language::Deustch,
            "zh" => Language::Mandarin,
            "ja" => Language::Japanese,
            "ko" => Language::Korean,
            "eo" => Language::Esperanto,
            "ar" => Language::Arabic,
            "ru" => Language::Russian,
            _ => return None,
        })
    }
}

impl TryFrom<&Request> for Language {
    type Error = ServerError;

    fn try_from(req: &Request) -> Result<Self, Self::Error> {
        use std::str::FromStr;

        if let Some(language) = Self::from_path(req.uri().path()) {
            return Ok(language);
        }

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
            "zh" => Language::Mandarin,
            "ja" => Language::Japanese,
            "ko" => Language::Korean,
            "eo" => Language::Esperanto,
            "ar" => Language::Arabic,
            "ru" => Language::Russian,
            _ => {
                return Err(ServerErrorKind::DebugedError(String::from("Unknown language.")).into())
            },
        })
    }
}
