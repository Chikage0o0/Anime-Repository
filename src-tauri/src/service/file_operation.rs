use std::{fmt::Debug, fs::File, io::Write, path::Path};

use quick_xml::se::Serializer;

pub fn write_file<P, C>(path: P, data: &C) -> Result<(), FileOperationError>
where
    P: AsRef<Path>,
    C: serde::Serialize,
{
    let mut file = File::create(path)?;
    let mut ser = Serializer::new(String::new());
    ser.indent(' ', 4);
    let data = data.serialize(ser)?;

    file.write_all(data.as_bytes())?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum FileOperationError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),
    #[error(transparent)]
    SerializeError(#[from] quick_xml::DeError),
}
