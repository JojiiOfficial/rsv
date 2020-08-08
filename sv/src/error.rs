#[derive(Debug)]
pub enum Error {
    ServiceNotAccessable,
    ServiceNotFound,
    ServiceDirNotFound,
}

impl Error {
    pub fn string(self) -> String {
        match self {
            Error::ServiceDirNotFound => "Runsvdir not found".to_string(),
            Error::ServiceNotAccessable => "Not accessable".to_string(),
            Error::ServiceNotFound => "Service not found".to_string(),
        }
    }
}
