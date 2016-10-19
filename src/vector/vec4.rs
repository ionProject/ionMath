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

use ::vector::{Vec2, Vec3};
use ::util::{Clamp, Lerp, MinMax};

use std::ops::{Add,   AddAssign,
    Sub,   SubAssign,
    Mul,   MulAssign,
    Div,   DivAssign,
    Index, IndexMut};
use std::convert::From;

/*===============================================================================================*/
/*------Vec4 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The generic Vec4 struct.
#[cfg_attr (feature = "serde_serialize", derive (Deserialize, Serialize))]
#[derive (Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec4<V> where
    V: Copy + Num + NumCast {

    // Public
    /// The vector x-coordinate.
    pub x: V,
    /// The vector y-coordinate.
    pub y: V,
    /// The vector z-coordinate.
    pub z: V,
    /// The vector w-coordinate
    pub w: V,
}

// Predefined Vec4 types
/// `Vec4<f32>`
pub type Vec4f = Vec4<f32>;
/// `Vec4<i32>`
pub type Vec4i = Vec4<i32>;
/// `Vec4<u32>`
pub type Vec4u = Vec4<u32>;

/*===============================================================================================*/
/*------Vec4 TRAIT IMPLEMENTATIONS---------------------------------------------------------------*/
/*===============================================================================================*/

impl<V> Clamp for Vec4<V> where
    V: Copy + Num + NumCast + PartialOrd {

    fn clamp (self, min: Self, max: Self) -> Self {

        debug_assert! (min.x < max.x, "Min cannot be greater than max.");
        debug_assert! (min.y < max.y, "Min cannot be greater than max.");
        debug_assert! (min.z < max.z, "Min cannot be greater than max.");
        debug_assert! (min.w < max.w, "Min cannot be greater than max.");

        let xval = if self.x < min.x {min.x} else if self.x > max.x {max.x} else {self.x};
        let yval = if self.y < min.y {min.y} else if self.y > max.y {max.y} else {self.y};
        let zval = if self.z < min.z {min.z} else if self.z > max.z {max.z} else {self.z};
        let wval = if self.w < min.w {min.w} else if self.w > max.w {max.w} else {self.w};

        Vec4::new (xval, yval, zval, wval)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V, U> From<Vec2<U>> for Vec4<V> where
    V: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: Vec2<U>) -> Vec4<V> {

        Vec4::new (V::from (value.x).unwrap (),
                   V::from (value.y).unwrap (),
                   V::zero (),
                   V::zero ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V, U> From<Vec3<U>> for Vec4<V> where
    V: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: Vec3<U>) -> Vec4<V> {

        Vec4::new (V::from (value.x).unwrap (),
                   V::from (value.y).unwrap (),
                   V::from (value.z).unwrap (),
                   V::zero ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Lerp for Vec4<V> where
    V: Copy + Num + NumCast {

    fn lerp (start: Self, end: Self, percentage: f32) -> Self {

        Vec4::new (V::lerp (start.x, end.x, percentage),
                   V::lerp (start.y, end.y, percentage),
                   V::lerp (start.z, end.z, percentage),
                   V::lerp (start.w, end.w, percentage))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn lerp_unclamped (start: Self, end: Self, percentage: f32) -> Self {

        Vec4::new (V::lerp_unclamped (start.x, end.x, percentage),
                   V::lerp_unclamped (start.y, end.y, percentage),
                   V::lerp_unclamped (start.z, end.z, percentage),
                   V::lerp_unclamped (start.w, end.w, percentage))
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> MinMax for Vec4<V> where
    V: Copy + Num + NumCast + PartialOrd {

    fn max (lhs: Self, rhs: Self) -> Self {

        Vec4::new (V::max (lhs.x, rhs.x),
                   V::max (lhs.y, rhs.y),
                   V::max (lhs.z, rhs.z),
                   V::max (lhs.w, rhs.w))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn min (lhs: Self, rhs: Self) -> Self {

        Vec4::new (V::min (lhs.x, rhs.x),
                   V::min (lhs.y, rhs.y),
                   V::min (lhs.z, rhs.z),
                   V::min (lhs.w, rhs.w))
    }
}

/*===============================================================================================*/
/*------Vec4 OPERATORS---------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<V> Add for Vec4<V> where
    V: Copy + Num + NumCast {

    type Output = Vec4<V>;

    fn add (self, rhs: Vec4<V>) -> Vec4<V> {

        Vec4::new (self.x + rhs.x,
                   self.y + rhs.y,
                   self.z + rhs.z,
                   self.w + rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Add<V> for Vec4<V> where
    V: Copy + Num + NumCast {

    type Output = Vec4<V>;

    fn add (self, rhs: V) -> Vec4<V> {

        Vec4::new (self.x + rhs,
                   self.y + rhs,
                   self.z + rhs,
                   self.w + rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> AddAssign for Vec4<V> where
    V: Copy + Num + NumCast {

    fn add_assign (&mut self, rhs: Vec4<V>) {

        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self.w = self.w + rhs.w;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> AddAssign<V> for Vec4<V> where
    V: Copy + Num + NumCast {

    fn add_assign (&mut self, rhs: V) {

        self.x = self.x + rhs;
        self.y = self.y + rhs;
        self.z = self.z + rhs;
        self.w = self.w + rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Sub for Vec4<V> where
    V: Copy + Num + NumCast {

    type Output = Vec4<V>;

    fn sub (self, rhs: Vec4<V>) -> Vec4<V> {

        Vec4::new (self.x - rhs.x,
                   self.y - rhs.y,
                   self.z - rhs.z,
                   self.w - rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Sub<V> for Vec4<V> where
    V: Copy + Num + NumCast {

    type Output = Vec4<V>;

    fn sub (self, rhs: V) -> Vec4<V> {

        Vec4::new (self.x - rhs,
                   self.y - rhs,
                   self.z - rhs,
                   self.w - rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> SubAssign for Vec4<V> where
    V: Copy + Num + NumCast {

    fn sub_assign (&mut self, rhs: Vec4<V>) {

        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
        self.w = self.w - rhs.w;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> SubAssign<V> for Vec4<V> where
    V: Copy + Num + NumCast {

    fn sub_assign (&mut self, rhs: V) {

        self.x = self.x - rhs;
        self.y = self.y - rhs;
        self.z = self.z - rhs;
        self.w = self.w - rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Mul for Vec4<V> where
    V: Copy + Num + NumCast {

    type Output = Vec4<V>;

    fn mul (self, rhs: Vec4<V>) -> Vec4<V> {

        Vec4::new (self.x * rhs.x,
                   self.y * rhs.y,
                   self.z * rhs.z,
                   self.w * rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Mul<V> for Vec4<V> where
    V: Copy + Num + NumCast {

    type Output = Vec4<V>;

    fn mul (self, rhs: V) -> Vec4<V> {

        Vec4::new (self.x * rhs,
                   self.y * rhs,
                   self.z * rhs,
                   self.w * rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> MulAssign for Vec4<V> where
    V: Copy + Num + NumCast {

    fn mul_assign (&mut self, rhs: Vec4<V>) {

        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
        self.w = self.w * rhs.w;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> MulAssign<V> for Vec4<V> where
    V: Copy + Num + NumCast {

    fn mul_assign (&mut self, rhs: V) {

        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
        self.w = self.w * rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Div for Vec4<V> where
    V: Copy + Num + NumCast {

    type Output = Vec4<V>;

    fn div (self, rhs: Vec4<V>) -> Vec4<V> {

        Vec4::new (self.x / rhs.x,
                   self.y / rhs.y,
                   self.z / rhs.z,
                   self.w / rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Div<V> for Vec4<V> where
    V: Copy + Num + NumCast {

    type Output = Vec4<V>;

    fn div (self, rhs: V) -> Vec4<V> {

        Vec4::new (self.x / rhs,
                   self.y / rhs,
                   self.z / rhs,
                   self.w / rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> DivAssign for Vec4<V> where
    V: Copy + Num + NumCast {

    fn div_assign (&mut self, rhs: Vec4<V>) {

        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
        self.w = self.w / rhs.w;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> DivAssign<V> for Vec4<V> where
    V: Copy + Num + NumCast {

    fn div_assign (&mut self, rhs: V) {

        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
        self.w = self.w / rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Index<u8> for Vec4<V> where
    V: Copy + Num + NumCast {

    type Output = V;

    fn index (&self, index: u8) -> &V {

        match index {

            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => unreachable! ("Index out of range for Vec4")
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> IndexMut<u8> for Vec4<V> where
    V: Copy + Num + NumCast {

    fn index_mut (&mut self, index: u8) -> &mut V {

        match index {

            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => unreachable! ("Index out of range for Vec4")
        }
    }
}

/*===============================================================================================*/
/*------Vec4 PUBLIC METHODS----------------------------------------------------------------------*/
/*===============================================================================================*/

impl<V> Vec4<V> where
    V: Copy + Num + NumCast {

    /// Converts self into an instance of `Vec4<C>`.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec01: Vec4<f32> = Vec4::new (4.3, 9.8, 7.0, 3.1);
    /// let vec02: Vec4<i32> = vec01.into ();
    /// ```
    pub fn into<C> (self) -> Vec4<C> where
        C: Copy + Num + NumCast {

        Vec4::new (C::from (self.x).unwrap (),
                   C::from (self.y).unwrap (),
                   C::from (self.z).unwrap (),
                   C::from (self.w).unwrap ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the dot product of two vectors.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec01 = Vec4::new (1, 3, 0, 5);
    /// let vec02 = Vec4::new (4, 9, 0, 4);
    ///
    /// let dot_product = vec01.dot (&vec02);
    /// ```
    pub fn dot (&self, rhs: &Vec4<V>) -> V {

        (self.x * rhs.x) +
        (self.y * rhs.y) +
        (self.z * rhs.z) +
        (self.w * rhs.w)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a new `Vec4<V>` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::new (3, 7, 10, 9);
    /// ```
    pub fn new (x: V, y: V, z: V, w: V) -> Vec4<V> {

        Vec4 {x: x,
              y: y,
              z: z,
              w: w}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a new instance of `Vec4<V>` from an instance of `Vec4<C>`.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec01: Vec4<f32> = Vec4::new  (3.2, 9.8, 7.0, 6.7);
    /// let vec02: Vec4<i32> = Vec4::from (vec01);
    /// ```
    pub fn from<C> (value: Vec4<C>) -> Vec4<V> where
        C: Copy + Num + NumCast {

        Vec4::new (V::from (value.x).unwrap (),
                   V::from (value.y).unwrap (),
                   V::from (value.z).unwrap (),
                   V::from (value.w).unwrap ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec4<V>` with a value of zero.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::<f32>::zero ();
    /// ```
    pub fn zero () -> Vec4<V> {

        Vec4::new (V::zero (),
                   V::zero (),
                   V::zero (),
                   V::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec4<V>` with a value of (0, 1, 0, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::<f32>::up ();
    /// ```
    pub fn up () -> Vec4<V> {

        Vec4::new (V::zero (),
                   V::one  (),
                   V::zero (),
                   V::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec4<V>` with a value of (0, -1, 0, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::<f32>::down ();
    /// ```
    pub fn down () -> Vec4<V> {

        Vec4::new (V::zero (),
                   V::from (-1).unwrap (),
                   V::zero (),
                   V::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec4<V>` with a value of (0, 1, 0, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::<f32>::right ();
    /// ```
    pub fn right () -> Vec4<V> {

        Vec4::new (V::one  (),
                   V::zero (),
                   V::zero (),
                   V::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec4<V>` with a value of (0, -1, 0, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::<f32>::left ();
    /// ```
    pub fn left () -> Vec4<V> {

        Vec4::new (V::from (-1).unwrap (),
                   V::zero (),
                   V::zero (),
                   V::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec4<V>` with a value of (0, 0, 1, 0)
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::<f32>::forward ();
    /// ```
    pub fn forward () -> Vec4<V> {

        Vec4::new (V::zero (),
                   V::zero (),
                   V::one  (),
                   V::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec4<V>` with a value of (0, 0, -1, 0)
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::<f32>::back ();
    /// ```
    pub fn back () -> Vec4<V> {

        Vec4::new (V::zero (),
                   V::zero (),
                   V::from (-1).unwrap (),
                   V::zero ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Vec4<V> where
    V: Float {

    /// Returns the distance between two vectors.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec01 = Vec4::new (1.0, 3.0, 0.0, 4.3);
    /// let vec02 = Vec4::new (4.0, 9.0, 0.0, 1.0);
    ///
    /// let distance = vec01.distance (&vec02);
    /// ```
    pub fn distance (&self, rhs: &Vec4<V>) -> V {

        (*self - *rhs).length ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the length of a vector.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::new (1.0, 3.0, 0.0, 6.0);
    /// let vec_length = vec.length ();
    /// ```
    pub fn length (&self) -> V {

        (self.x * self.x +
         self.y * self.y +
         self.z * self.z +
         self.w * self.w).sqrt ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a normalized vector.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::new (3.0, 9.0, 0.0, 4.0);
    /// let vec_normalized = vec.normalize ();
    /// ```
    pub fn normalize (&self) -> Vec4<V> {

        let length = self.length ();

        if length != V::zero () {

            return Vec4::new (self.x / length,
                              self.y / length,
                              self.z / length,
                              self.w / length);
        }

        Vec4::zero ()
    }
}
