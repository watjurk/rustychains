use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum M {
    Error(Box<Error>),
    Debug(Box<Debug>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub err: String,
}

impl Into<M> for Error {
    fn into(self) -> M {
        M::Error(self.into())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Debug {
    pub message: String,
}

impl Into<M> for Debug {
    fn into(self) -> M {
        M::Debug(self.into())
    }
}

pub fn write_message(writer: impl Write, message: impl Into<M>) -> bincode::Result<()> {
    let bytes = bincode::serialize(&message.into())?;

    let mut writer = writer;
    writer.write(&bytes)?;
    Ok(())
}

pub fn read_message(reader: &mut impl Read) -> bincode::Result<M> {
    bincode::deserialize_from(reader)
}
