// A sv command
#[derive(Debug)]
pub struct Service {
    uri: String,
}

#[derive(Debug)]
pub enum Error {
    ServiceNotAccessable,
}

impl Service {
    /// Create a new SvCommand object
    pub fn new(uri: String) -> Result<Service, Error> {
        let service = Service { uri };
        service.check()?;

        Ok(service)
    }

    fn check(&self) -> Result<(), Error> {
        // TODO implement service checking
        Ok(())
    }

    /// Run a sv command
    fn run(&self) -> String {
        "".to_string()
    }
}
