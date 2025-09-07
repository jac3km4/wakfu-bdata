use std::io;

use serde::Serialize;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone, Serialize)]
pub struct BackgroundFeedback {
    pub id: i32,
    pub kind: i16,
    pub pages: Vec<BackgroundFeedbackPages>,
}

impl Decode for BackgroundFeedback {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let kind = state.decode()?;
        let pages = state.decode()?;
        Ok(Self { id, kind, pages })
    }
}

impl BinaryData for BackgroundFeedback {
    const TYPE_ID: i16 = 6;
}

#[derive(Debug, Clone, Serialize)]
pub struct BackgroundFeedbackPages {
    pub id: i32,
    pub order: i16,
    pub template: i16,
    pub image_id: i32,
}

impl Decode for BackgroundFeedbackPages {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let order = state.decode()?;
        let template = state.decode()?;
        let image_id = state.decode()?;
        Ok(Self {
            id,
            order,
            template,
            image_id,
        })
    }
}
