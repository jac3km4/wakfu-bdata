use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct Dialog {
    pub id: i32,
    pub criteria: String,
    pub _2: String,
    pub _3: String,
    pub answers: Vec<DialogAnswers>,
}

impl Decode for Dialog {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let criteria = state.decode()?;
        let _2 = state.decode()?;
        let _3 = state.decode()?;
        let answers = state.decode()?;
        Ok(Self {
            id,
            criteria,
            _2,
            _3,
            answers,
        })
    }
}

impl BinaryData for Dialog {
    const TYPE_ID: i16 = 27;
}

#[derive(Debug, Clone, Serialize)]
pub struct DialogAnswers {
    pub id: i32,
    pub criterion: String,
    pub kind: i32,
    pub client_only: bool,
}

impl Decode for DialogAnswers {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let criterion = state.decode()?;
        let kind = state.decode()?;
        let client_only = state.decode()?;
        Ok(Self {
            id,
            criterion,
            kind,
            client_only,
        })
    }
}
