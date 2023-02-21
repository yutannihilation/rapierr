use extendr_api::prelude::*;

use std::convert::TryFrom;

/// An intermediate form to convert to tibble.
pub struct ResultTibble {
    pub frame: Vec<i32>,
    // Unscaled position of x.
    pub x: Vec<f32>,
    // Unscaled position of y.
    pub y: Vec<f32>,
}

impl TryFrom<ResultTibble> for Robj {
    type Error = extendr_api::Error;

    fn try_from(value: ResultTibble) -> std::result::Result<Self, Self::Error> {
        // Find tibble
        let tibble_robj = R!("tibble::tibble")?;
        let tibble = match tibble_robj.as_function() {
            Some(fun) => fun,
            None => {
                return Err(extendr_api::Error::ExpectedFunction(tibble_robj));
            }
        };

        let result = tibble.call(pairlist!(x = value.x, y = value.y,))?;

        Ok(result)
    }
}
