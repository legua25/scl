// blobs.rs

use std::fmt::{ Display, Formatter, Result as FmtResult };
use std::iter::{ FromIterator, IntoIterator, Extend };
use std::io::{ Read, Write, Result as IoResult };
use base64::prelude::{ BASE64_STANDARD };
use std::borrow::{ Borrow, BorrowMut };
use base64::{ Engine, DecodeError };
use std::ops::{ Deref, DerefMut };
use std::str::{ FromStr };


#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Blob {
    metadata: Option<String>,
    data: Vec<u8>,
}

impl Blob {
    pub fn new() -> Self { Self::default() }
    pub fn with_metadata(metadata: impl AsRef<str>) -> Self {
        let metadata = Some(metadata.as_ref().to_string());
        Self { metadata, ..Default::default() }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }
    pub fn with_capacity_and_metadata(capacity: usize, metadata: impl AsRef<str>) -> Self {
        Self::from_vec_with_metadata(Vec::with_capacity(capacity), metadata)
    }
    pub fn from_vec(data: Vec<u8>) -> Self {
        Self { data, metadata: None }
    }
    pub fn from_vec_with_metadata(data: Vec<u8>, metadata: impl AsRef<str>) -> Self {
        let metadata = Some(metadata.as_ref().to_string());
        Self { data, metadata }
    }
    pub fn decode_base64(encoded: impl AsRef<[u8]>) -> Result<Self, DecodeError> {
        let data = BASE64_STANDARD.decode(encoded)?;
        Ok(Self::from_vec(data))
    }
    pub fn decode_base64_with_metadata(encoded: impl AsRef<[u8]>, metadata: impl AsRef<str>) -> Result<Self, DecodeError> {
        let data = BASE64_STANDARD.decode(encoded)?;
        Ok(Self::from_vec_with_metadata(data, metadata))
    }

    #[inline] pub fn capacity(&self) -> usize { self.data.capacity() }
    #[inline] pub fn metadata(&self) -> Option<&String> { self.metadata.as_ref() }

    pub fn encode_base64(&self) -> String {
        BASE64_STANDARD.encode(&self.data)
    }
    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }
}
impl FromStr for Blob {
    type Err = DecodeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::decode_base64(s)
    }
}

impl AsRef<[u8]> for Blob {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] { &self.data }
}
impl AsRef<Vec<u8>> for Blob {
    #[inline(always)]
    fn as_ref(&self) -> &Vec<u8> { &self.data }
}
impl AsMut<[u8]> for Blob {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u8] { &mut self.data }
}
impl AsMut<Vec<u8>> for Blob {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Vec<u8> { &mut self.data }
}
impl Deref for Blob {
    type Target = [u8];

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.data }
}
impl DerefMut for Blob {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.data }
}
impl Borrow<Vec<u8>> for Blob {
    fn borrow(&self) -> &Vec<u8> { &self.data }
}
impl BorrowMut<Vec<u8>> for Blob {
    fn borrow_mut(&mut self) -> &mut Vec<u8> { &mut self.data }
}

impl Display for Blob {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let data = self.encode_base64();
        match self.metadata {
            None => write!(f, "{data}"),
            Some(ref meta) => write!(f, "{meta}:{data}")
        }
    }
}

impl Read for Blob {
    #[inline(always)]
    fn read(&mut self, mut buf: &mut [u8]) -> IoResult<usize> { buf.write(&self.data) }
    #[inline(always)]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> IoResult<usize> {
        buf.write_all(&self.data)?;
        Ok(self.data.len())
    }
    #[inline(always)]
    fn read_to_string(&mut self, buf: &mut String) -> IoResult<usize> {
        let data = self.encode_base64();
        data.as_bytes().read_to_string(buf)
    }
}
impl Write for Blob {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> { self.data.write(buf) }
    #[inline(always)]
    fn flush(&mut self) -> IoResult<()> { self.data.flush() }
    #[inline(always)]
    fn write_all(&mut self, buf: &[u8]) -> IoResult<()> { self.data.write_all(buf) }
}
impl FromIterator<u8> for Blob {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Blob::from_vec(Vec::from_iter(iter))
    }
}
impl IntoIterator for Blob {
    type Item = u8;
    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
impl Extend<u8> for Blob {
    #[inline]
    fn extend<T: IntoIterator<Item=u8>>(&mut self, iter: T) { self.data.extend(iter) }
}
impl<'a> Extend<&'a u8> for Blob {
    #[inline]
    fn extend<T: IntoIterator<Item = &'a u8>>(&mut self, iter: T) { self.data.extend(iter) }
}
