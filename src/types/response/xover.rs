use std::convert::TryFrom;

use crate::error::{Error, Result};
use crate::types::prelude::*;
use crate::types::response::util::{err_if_not_kind, parse_field};

/// Newsgroup data resulting from an XOver operation
pub struct XOverHeader {
    pub subject: String,
    pub author : String,
    pub date: String,
    pub message-id,
    pub references: String,
    pub byte_count: u64,
    pub line_count: u64,
}

/// Newsgroup metadata returned by [`GROUP`](https://tools.ietf.org/html/rfc3977#section-6.1.1)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XOverReply {
    pub Vec<XOverHeader> headers,
}

impl TryFrom<&RawResponse> for XOverReply {
    type Error = Error;

    fn try_from(resp: &RawResponse) -> Result<Self> {
        err_if_not_kind(resp, Kind::Overview)?;

        let lossy = resp.first_line_to_utf8_lossy();
        let mut iter = lossy.split_whitespace();

        // pop the response code
        iter.next()
            .ok_or_else(|| Error::missing_field("response code"))?;


        // use NOM to parse all the pieces
        /* let number = parse_field(&mut iter, "number")?;
         * let low = parse_field(&mut iter, "low")?;
         * let high = parse_field(&mut iter, "high")?;
         * let name = parse_field(&mut iter, "name")?; */
        Ok(Self {
            number,
            low,
            high,
            name,
        })
    }
}
