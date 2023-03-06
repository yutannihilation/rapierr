use extendr_api::prelude::*;

use std::convert::TryFrom;

/// An intermediate form to convert to tibble.
pub struct ResultTibble {
    // Frame (the simulation usually runs at 60FPS)
    pub frame: Integers,
    // Index of the object
    pub index: Integers,
    // Unscaled position of x.
    pub x: Doubles,
    // Unscaled position of y.
    pub y: Doubles,
    // Size of the object (radius of a ball, and width of the cuboid).
    pub size: Doubles,
    // Angle of the object
    pub angle: Doubles,
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

        let result = tibble.call(pairlist!(
            frame = value.frame,
            index = value.index,
            x = value.x,
            y = value.y,
            size = value.size,
            angle = value.angle,
        ))?;

        Ok(result)
    }
}
