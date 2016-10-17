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

use ::vector::Vec3;

/*===============================================================================================*/
/*------mat3 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The generic `Mat3` struct.
///
/// It is used for manipulating objects in 3d space.
#[derive (Copy, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Mat3<T> where
    T: Copy + Num + NumCast {

    // Private
    array: [Vec3<T>; 3]
}

// Predefined Mat3 types
/// `Mat3<f32>`
pub type Mat3f = Mat3<f32>;
/// `Mat3<i32>`
pub type Mat3i = Mat3<i32>;
/// `Mat3<u32>`
pub type Mat3u = Mat3<u32>;

/*===============================================================================================*/
/*------MAT3 PUBLIC METHODS----------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Mat3<T> where
    T: Copy + Num + NumCast {

    /// Returns a new `Mat3` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::matrix::Mat3;
    /// let mat = Mat3::new (1, 2, 3,
    ///                      4, 5, 6,
    ///                      7, 8, 9);
    /// ```
    pub fn new (m11: T, m21: T, m31: T,
                m12: T, m22: T, m32: T,
                m13: T, m23: T, m33: T) -> Self {

        Mat3 {array: [Vec3::new (m11, m21, m31),
                      Vec3::new (m12, m22, m32),
                      Vec3::new (m13, m23, m33)]}
    }
}
