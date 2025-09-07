use std::fs::File;
use std::io::{self};
use std::path::Path;

use zip::ZipArchive;

use crate::data::BinaryData;
use crate::decode::{Decode, DecodeState};

#[derive(Debug, Clone)]
#[allow(unused)]
struct Entry {
    id: i64,
    position: u32,
    size: u32,
    seed: i8,
}

impl Decode for Entry {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let position = state.decode::<i32>()? as u32;
        let size = state.decode::<i32>()? as u32;
        let seed = state.decode()?;
        Ok(Entry {
            id,
            position,
            size,
            seed,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Index {
    Unique(i64, i32),
    NonUnique(i64, Vec<i32>),
}

#[derive(Debug, Clone)]
pub struct Indexes {
    pub name: String,
    pub entries: Vec<Index>,
}

impl Decode for Indexes {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let unique = state.decode()?;
        let name = state.decode()?;
        let count: i32 = state.decode()?;
        let mut entries = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let id = state.decode()?;
            if unique {
                entries.push(Index::Unique(id, state.decode()?));
            } else {
                entries.push(Index::NonUnique(id, state.decode()?));
            }
        }
        Ok(Indexes { name, entries })
    }
}

#[derive(Debug, Clone)]
pub struct Document<A> {
    pub indexes: Vec<Indexes>,
    pub elements: Vec<A>,
}

impl<A: BinaryData> Document<A> {
    pub fn load(root: &Path) -> io::Result<Document<A>> {
        let path = root
            .join("contents")
            .join("bdata")
            .join(format!("{}.jar", A::TYPE_ID));
        Document::load_file(File::open(path)?)
    }

    fn load_file(file: File) -> io::Result<Document<A>> {
        let mut archive = ZipArchive::new(file).unwrap();
        let entry = archive.by_index(0).unwrap();
        let index = A::TYPE_ID.into();
        let mut state = DecodeState::new(entry, index)?;
        let entries: Vec<Entry> = state.decode()?;
        let index_count: i8 = state.decode()?;
        let mut indexes = Vec::with_capacity(index_count as usize);
        for _ in 0..index_count {
            indexes.push(state.decode()?);
        }
        state.reset(index)?;
        let entry_count = entries.len();
        let mut elements = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            elements.push(state.decode()?);
        }
        Ok(Document { indexes, elements })
    }
}
