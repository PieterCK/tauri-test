#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  BadRequest(#[from] reqwest::Error),
}

#[derive(serde::Serialize)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
enum ErrorKind {
    BadRequest(String),
}

impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    let error_message = self.to_string();
    let error_kind = match self {
      Self::BadRequest(_) => ErrorKind::BadRequest(error_message),
    };
    error_kind.serialize(serializer)
  }
}
