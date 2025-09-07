use std::collections::HashMap;
use std::hash::Hash;
use std::io;

use byteorder::{LittleEndian, ReadBytesExt};

pub struct DecodeState<R> {
    inner: R,
    position: usize,
    seed: i8,
    mul: i32,
    add: i32,
}

impl<R: io::Read> DecodeState<R> {
    #[inline]
    fn advance(&mut self, size: usize) {
        self.seed = self
            .seed
            .wrapping_add((self.mul * self.position as i32 + self.add) as i8);
        self.position += size;
    }

    #[inline]
    pub fn decode<A: Decode>(&mut self) -> io::Result<A> {
        Decode::decode(self)
    }

    pub fn reset(&mut self, mul: i32) -> io::Result<()> {
        self.mul = mul;
        self.add = self.inner.read_i32::<LittleEndian>()? + 756423;
        self.seed = (self.mul ^ self.add) as i8;
        self.position = 4;
        Ok(())
    }
}

impl<R: io::Read> DecodeState<R> {
    pub fn new(mut inner: R, mul: i32) -> io::Result<DecodeState<R>> {
        let add = inner.read_i32::<LittleEndian>()? + 756423;
        let state = DecodeState {
            inner,
            position: 4,
            mul,
            add,
            seed: (mul ^ add) as i8,
        };
        Ok(state)
    }
}

pub trait Decode
where
    Self: Sized,
{
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self>;
}

impl Decode for i8 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        state.advance(1);
        Ok(state.inner.read_i8()?.wrapping_sub(state.seed))
    }
}

impl Decode for i16 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        state.advance(2);
        Ok(state
            .inner
            .read_i16::<LittleEndian>()?
            .wrapping_sub(state.seed as i16))
    }
}

impl Decode for i32 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        state.advance(4);
        Ok(state
            .inner
            .read_i32::<LittleEndian>()?
            .wrapping_sub(state.seed as i32))
    }
}

impl Decode for i64 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        state.advance(8);
        Ok(state
            .inner
            .read_i64::<LittleEndian>()?
            .wrapping_sub(state.seed as i64))
    }
}

impl Decode for bool {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        state.advance(1);
        let val = state.inner.read_i8()?.wrapping_sub(state.seed);
        Ok(val != 0)
    }
}

impl Decode for f32 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        state.advance(4);
        state.inner.read_f32::<LittleEndian>()
    }
}

impl Decode for f64 {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        state.advance(8);
        state.inner.read_f64::<LittleEndian>()
    }
}

impl Decode for String {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let size = state.decode::<i32>()? as usize;
        let mut buf = vec![0u8; size];
        state.inner.read_exact(&mut buf)?;
        state.position += size;
        Ok(String::from_utf8(buf).unwrap())
    }
}

impl<A: Decode> Decode for Vec<A> {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let count: i32 = state.decode()?;
        let mut vec = Vec::with_capacity(count as usize);
        for _ in 0..count {
            vec.push(state.decode()?);
        }
        Ok(vec)
    }
}

impl<A: Decode> Decode for Option<A> {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let is_some = state.decode::<bool>()?;
        if is_some {
            Ok(Some(state.decode()?))
        } else {
            Ok(None)
        }
    }
}

impl<K: Decode + Hash + Eq, V: Decode> Decode for HashMap<K, V> {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let count: i32 = state.decode()?;
        let mut vec = HashMap::with_capacity(count as usize);
        for _ in 0..count {
            let k = state.decode()?;
            let v = state.decode()?;
            vec.insert(k, v);
        }
        Ok(vec)
    }
}
