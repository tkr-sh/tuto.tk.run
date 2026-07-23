use {
    serde::{de::DeserializeOwned, Deserialize},
    std::marker::PhantomData,
};

/// Core shared for wini
pub mod wini;

#[derive(Debug, Deserialize)]
pub struct DatastarQuery<T: DeserializeOwned> {
    #[serde(default = "default_datastar")]
    datastar: String,
    #[serde(skip_deserializing)]
    _phantom: PhantomData<T>,
}

fn default_datastar() -> String {
    String::from("{}")
}

impl<T: DeserializeOwned> DatastarQuery<T> {
    pub fn inner(self) -> serde_json::Result<T> {
        serde_json::from_str(&self.datastar)
    }
}
