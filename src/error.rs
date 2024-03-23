#[derive(Debug)]
pub struct Error {
    kind: Kind,
    code: Option<u16>,
    text: Option<String>,
}

impl Error {
    pub(crate) fn new(kind: Kind) -> Error {
        Error {
            kind: kind,
            code: None,
            text: None,
        }
    }

    pub(crate) fn with_detail(kind: Kind, code: Option<u16>, text: Option<String>) -> Error {
        Error {
            kind: kind,
            code: code,
            text: text,
        }
    }
}


impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }

}
#[derive(Debug)]
pub enum Kind {
    Request,
    Parse,
    Query,
}
