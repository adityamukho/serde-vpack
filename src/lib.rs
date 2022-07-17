// Copyright 2022 Aditya Mukhopadhyay
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>.

mod de;
mod error;
mod ser;

pub use crate::de::{from_vpack, Deserializer};
pub use crate::error::{Error, Result};
pub use crate::ser::{to_vpack, Serializer};
