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

use ::angle::Rad;

use std::convert::From;

/*===============================================================================================*/
/*------DEG STRUCT-------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Stores a value in Degrees.
#[cfg_attr (feature = "serde_serialize", derive (Deserialize, Serialize))]
#[derive (Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Deg<T> where
    T: Copy + Float + NumCast {

    // Public
    /// The value of the degree.
    pub value: T,
}

/*===============================================================================================*/
/*------CONSTRUCTORS-----------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Deg<T> where
    T: Copy + Float + NumCast {

    /// Returns a new `Deg` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::angle::Deg;
    /// let deg = Deg::<f32>::new (45.0);
    /// ```
    pub fn new<C> (value: C) -> Deg<T> where
        C: Num + NumCast {

        Deg {value: T::from (value).unwrap ()}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> From<&'a Rad<T>> for Deg<T> where
    T: Copy + Float + NumCast {

    fn from (rad: &Rad<T>) -> Deg<T> {
        Deg::new (rad.value * T::from (57.295779).unwrap ())
    }
}
