#[derive(Debug)]
pub enum Kind {
    Request,
    Parse,
    Query,
}

#[derive(Debug)]
pub struct Error {
    kind: Kind,
    code: Option<u16>,
    text: Option<String>,
}

impl Error {
    pub(crate) fn new(kind: Kind) -> Error {
        Error {
            kind,
            code: None,
            text: None,
        }
    }

    pub(crate) fn with_detail(kind: Kind, code: Option<u16>, text: Option<String>) -> Error {
        Error {
            kind,
            code,
            text,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ctbox]{:?} with code {} and text {}",
            self.kind,
            self.code.map_or("None".to_owned(), |c| c.to_string()),
            self.text
                .as_ref()
                .map_or("None".to_owned(), |c| c.to_string())
        )
    }
}
impl std::error::Error for Error {}
