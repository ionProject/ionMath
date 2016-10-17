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

use ::vector::Vec4;

/*===============================================================================================*/
/*------MAT4 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The generic `Mat4` struct.
///
/// It is used for manipulating objects in 3d space.
#[derive (Copy, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Mat4<T> where
    T: Copy + Num + NumCast {

    // Private
    array: [Vec4<T>; 4]
}

// Predefined Mat4 types
/// `Mat4<f32>`
pub type Mat4f = Mat4<f32>;
/// `Mat4<i32>`
pub type Mat4i = Mat4<i32>;
/// `Mat4<u32>`
pub type Mat4u = Mat4<u32>;

/*===============================================================================================*/
/*------MAT4 PUBLIC METHODS----------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Mat4<T> where
    T: Copy + Num + NumCast {

    /// Returns a new `Mat4` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::matrix::Mat4;
    /// let mat = Mat4::new (1,  2,  3,  4,
    ///                      5,  6,  7,  8,
    ///                      9,  10, 11, 12,
    ///                      13, 14, 15, 16);
    /// ```
    pub fn new (m11: T, m21: T, m31: T, m41: T,
                m12: T, m22: T, m32: T, m42: T,
                m13: T, m23: T, m33: T, m43: T,
                m14: T, m24: T, m34: T, m44: T,) -> Self {

        Mat4 {array: [Vec4::new (m11, m21, m31, m41),
                      Vec4::new (m12, m22, m32, m42),
                      Vec4::new (m13, m23, m33, m43),
                      Vec4::new (m14, m24, m34, m44)]}
    }
}
