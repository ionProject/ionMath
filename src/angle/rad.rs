/*===============================================================================================*/
// Copyright 2016 Kyle Finlay
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
/*===============================================================================================*/

// Crate imports
extern crate num_traits;

// Module imports
use self::num_traits::{Float, Num, NumCast};

use ::angle::Deg;

use std::convert::From;

/*===============================================================================================*/
/*------RAD STRUCT-------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Stores a value in Radians.
#[cfg_attr (feature = "serde_serialize", derive (Deserialize, Serialize))]
#[derive (Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Rad<T> where
    T: Copy + Float + NumCast {

    // Public
    /// The value of the radian.
    pub value: T,
}

/*===============================================================================================*/
/*------CONSTRUCTORS-----------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Rad<T> where
    T: Copy + Float + NumCast {

    /// Returns a new `Rad` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::angle::{Deg, Rad};
    /// let rad = Rad::<f32>::new (2.641);
    /// ```
    pub fn new<C> (value: C) -> Rad<T> where
        C: Num + NumCast {

        Rad {value: T::from (value).unwrap ()}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> From<&'a Deg<T>> for Rad<T> where
    T: Copy + Float + NumCast {

    fn from (deg: &Deg<T>) -> Rad<T> {
        Rad::new (deg.value * T::from (0.017453).unwrap ())
    }
}
