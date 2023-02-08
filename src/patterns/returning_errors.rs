use std::fs;
use std::io::Error as IOError;

#[derive(Debug)]
enum ApplicationErrors {
    IO(IOError),
}

#[derive(Debug)]
pub struct ErrorWrapper {
    source: ApplicationErrors,
    message: String,
}

impl From<IOError> for ErrorWrapper {
    fn from(source: IOError) -> Self {
        Self {
            source: ApplicationErrors::IO(source),
            message: "IO Error".into(),
        }
    }
}

pub fn read_error() -> Result<(), ErrorWrapper> {
    fs::read("non_existing_file_path")?;
    Ok(())
}

pub fn run() {
    match read_error() {
        Err(err) => println!("{} : {:?}", err.message, err.source),
        _ => (),
    }
}
