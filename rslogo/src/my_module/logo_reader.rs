use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use super::logo_parser::{parse_program, Program};

pub fn read_logo_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let path = path.as_ref();

    if let Some(extension) = path.extension() {
        if extension != "lg" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Please input right file format: xxx.lg. Got: {:?}", path.file_name().unwrap_or_default())
            ));
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("File has no extension. Please use .lg files. Got: {:?}", path.file_name().unwrap_or_default())
        ));
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse_logo_file<P: AsRef<Path>>(path: P) -> io::Result<Program> {
    let contents = read_logo_file(path)?;
    match parse_program(&contents) {
        Ok((_, program)) => Ok(program),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, format!("Parsing error: {:?}", e))),
    }
}
