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
use self::num_traits::{Num, NumCast};

use ::colour::ColourTrait;
use ::util;
use ::vector::{Vec2, Vec3, Vec4};

use std::convert::From;
use std::ops::{Add,   AddAssign,
               Sub,   SubAssign,
               Mul,   MulAssign,
               Div,   DivAssign,
               Index, IndexMut};

/*===============================================================================================*/
/*------RGBA STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Stores a RGBA colour value.
#[cfg_attr (feature = "serde_serialize", derive (Deserialize, Serialize))]
#[derive (Copy, Clone, Debug, PartialEq)]
pub struct RGBA {

    // Public
    /// Red channel.
    pub r: f32,
    /// Green channel.
    pub g: f32,
    /// Blue channel.
    pub b: f32,
    /// Alpha channel.
    pub a: f32,
}

/*===============================================================================================*/
/*------CONSTRUCTORS-----------------------------------------------------------------------------*/
/*===============================================================================================*/

impl RGBA {

    /// Returns a new `RGBA` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::RGBA;
    /// let colour = RGBA::new (0.0, 1.0, 0.5, 1.0);
    /// ```
    pub fn new<T> (r: T, g: T, b: T, a: T) -> RGBA where
        T: Copy + Num + NumCast {

        RGBA {r: r.to_f32 ().unwrap (),
              g: g.to_f32 ().unwrap (),
              b: b.to_f32 ().unwrap (),
              a: a.to_f32 ().unwrap ()}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> From<T> for RGBA where
    T: Copy + Num + NumCast {

    fn from (value: T) -> RGBA {

        RGBA::new (value,
                   value,
                   value,
                   value)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> From<&'a Vec2<T>> for RGBA where
    T: Copy + Num + NumCast {

    fn from (value: &Vec2<T>) -> RGBA {

        RGBA::new (value.x,
                   value.y,
                   T::zero (),
                   T::one ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> From<&'a Vec3<T>> for RGBA where
    T: Copy + Num + NumCast {

    fn from (value: &Vec3<T>) -> RGBA {

        RGBA::new (value.x,
                   value.y,
                   value.z,
                   T::one ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> From<&'a Vec4<T>> for RGBA where
    T: Copy + Num + NumCast {

    fn from (value: &Vec4<T>) -> RGBA {

        RGBA::new (value.x,
                   value.y,
                   value.z,
                   value.w)
    }
}
