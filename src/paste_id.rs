use rocket::{
    http::{
        impl_from_uri_param_identity,
        uri::fmt::{Formatter, Path, UriDisplay},
    },
    request::FromParam,
};
use std::borrow::Cow;
use std::fmt;
use std::fmt::Display;
use std::path::{Path as FilePath, PathBuf};

use rand::{self, Rng};

/// Table to retrieve base62 values from.
const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// A _probably_ unique paste ID.
pub struct PasteId<'a>(Cow<'a, String>);

impl<'a> PasteId<'a> {
    /// Generate a _probably_ unique ID with `size` characters. For readability,
    /// the characters used are from the sets [0-9], [A-Z], [a-z]. The
    /// probability of a collision depends on the value of `size` and the number
    /// of IDs generated thus far.
    pub fn new(size: usize) -> PasteId<'static> {
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }

        PasteId(Cow::Owned(id))
    }

    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
        FilePath::new(root).join(self.0.as_ref())
    }
}

impl<'a> Display for PasteId<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Returns an instance of `PasteId` if the path segment is a valid ID.
/// Otherwise returns the invalid ID as the `Err` value.
impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match param.chars().all(|c| c.is_ascii_alphanumeric()) {
            true => Ok(PasteId(Cow::Owned(param.into()))),
            false => Err(param),
        }
    }
}

impl UriDisplay<Path> for PasteId<'_> {
    fn fmt(&self, f: &mut Formatter<'_, Path>) -> fmt::Result {
        UriDisplay::fmt(self.0.as_ref(), f)
    }
}

impl_from_uri_param_identity!([Path] ('a) PasteId<'a>);
