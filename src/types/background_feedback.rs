use crate::BinaryData;
use crate::decode::{Decode, DecodeState};
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BackgroundFeedback {
    pub id: i32,
    pub kind: i16,
    pub pages: Vec<BackgroundFeedbackPages>,
}

impl BinaryData for BackgroundFeedback {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        6
    }
}

impl Decode for BackgroundFeedback {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let kind = state.decode()?;
        let pages = state.decode()?;
        Ok(BackgroundFeedback { id, kind, pages })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
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
        Ok(BackgroundFeedbackPages {
            id,
            order,
            template,
            image_id,
        })
    }
}
