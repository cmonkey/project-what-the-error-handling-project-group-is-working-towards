use std::fmt;

fn main() {
    let result = load_config();
    let _config = match result {
        Ok(config) => config,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };
}

fn load_config() -> Result<(), Error> {
    Err(Error(SourceError))
}

#[derive(Debug)]

struct Error(SourceError);
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        f.write_str("invalid config")
    }
}

impl std::error::Error for Error{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static) > {
        Some(&self.0)
    }
}

#[derive(Debug)]
struct SourceError;

impl fmt::Display for SourceError{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("config file does not exist")
    }
}
impl std::error::Error for SourceError{}
