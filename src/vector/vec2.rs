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

use ::vector::{Vec3, Vec4, VecTrait, VecTraitF};
use ::util::{Clamp, Lerp, MinMax};

use std::ops::{Add,   AddAssign,
               Sub,   SubAssign,
               Mul,   MulAssign,
               Div,   DivAssign,
               Index, IndexMut};
use std::convert::From;

/*===============================================================================================*/
/*------VEC2 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The generic Vec2 struct.
///
/// It is used mainly for 2D related mathematics such as texture coordinates,
/// UV coordinates, etc. <br>
/// It can accept any number as a value.
#[cfg_attr (feature = "serde_serialize", derive (Deserialize, Serialize))]
#[derive (Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2<V> where
    V: Copy + Num + NumCast {

    // Public
    /// The vector x-coordinate.
    pub x: V,
    /// The vector y-coordinate.
    pub y: V,
}

// Predefined Vec2 types
/// `Vec2<f32>`
pub type Vec2f = Vec2<f32>;
/// `Vec2<i32>`
pub type Vec2i = Vec2<i32>;
/// `Vec2<u32>`
pub type Vec2u = Vec2<u32>;

/*===============================================================================================*/
/*------VEC2 TRAIT IMPLEMENTATIONS---------------------------------------------------------------*/
/*===============================================================================================*/

impl<V> Clamp for Vec2<V> where
    V: Copy + Num + NumCast + PartialOrd {

    fn clamp (&self, min: &Vec2<V>, max: &Vec2<V>) -> Vec2<V> {

        debug_assert! (min.x < max.x, "Min cannot be greater than max.");
        debug_assert! (min.y < max.y, "Min cannot be greater than max.");

        Vec2::new (if self.x < min.x {min.x} else if self.x > max.x {max.x} else {self.x},
                   if self.y < min.y {min.y} else if self.y > max.y {max.y} else {self.y})
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V, U> From<U> for Vec2<V> where
    V: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: U) -> Vec2<V> {

        Vec2::new (V::from (value).unwrap (),
                   V::from (value).unwrap ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V, U> From<&'a Vec2<U>> for Vec2<V> where
    V: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: &Vec2<U>) -> Vec2<V> {

        Vec2::new (V::from (value.x).unwrap (),
                   V::from (value.y).unwrap ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V, U> From<&'a Vec3<U>> for Vec2<V> where
    V: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: &Vec3<U>) -> Vec2<V> {

        Vec2::new (V::from (value.x).unwrap (),
                   V::from (value.y).unwrap ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V, U> From<&'a Vec4<U>> for Vec2<V> where
    V: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: &Vec4<U>) -> Vec2<V> {

        Vec2::new (V::from (value.x).unwrap (),
                   V::from (value.y).unwrap ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Lerp for Vec2<V> where
    V: Copy + Num + NumCast {

    fn lerp<'a> (start: &'a Vec2<V>, end: &'a Vec2<V>, percentage: f32) -> Vec2<V> {
        (&(Vec2f::from (&(start + (end - start))) * percentage.clamp (&0.0, &1.0))).into ()
    }

/*-----------------------------------------------------------------------------------------------*/

    fn lerp_unclamped<'a> (start: &'a Vec2<V>, end: &'a Vec2<V>, percentage: f32) -> Vec2<V> {
        (&(Vec2f::from (&(start + (end - start))) * percentage)).into ()
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> MinMax for Vec2<V> where
    V: Copy + Num + NumCast + PartialOrd {

    fn max (lhs: &Vec2<V>, rhs: &Vec2<V>) -> Vec2<V> {

        Vec2::new (if lhs.x > rhs.x {lhs.x} else {rhs.x},
                   if lhs.y > rhs.y {lhs.y} else {rhs.y})
    }

/*-----------------------------------------------------------------------------------------------*/

    fn min (lhs: &Vec2<V>, rhs: &Vec2<V>) -> Vec2<V> {

        Vec2::new (if lhs.x < rhs.x {lhs.x} else {rhs.x},
                   if lhs.y < rhs.y {lhs.y} else {rhs.y})
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> VecTrait for Vec2<V> where
    V: Copy + Default + Num + NumCast + PartialOrd {

    type ValType = V;

    /// Returns a `Vec2<V>` with a value of zero.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec2, VecTrait};
    /// let vec = Vec2::<f32>::zero ();
    /// ```
    fn zero () -> Vec2<V> {

        Vec2::new (V::zero (),
                   V::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the dot product of two vectors.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec2, VecTrait};
    /// let vec01 = Vec2::<f32>::new (1.0, 3.0);
    /// let vec02 = Vec2::<f32>::new (4.0, 9.0);
    ///
    /// let dot_product = vec01.dot (&vec02);
    /// ```
    fn dot (&self, rhs: &Vec2<V>) -> V {

        (self.x * rhs.x) +
        (self.y * rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> VecTraitF for Vec2<V> where
    V: Default + Float {

    type ValTypeF = V;

    /// Returns the distance between two vectors.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec2, VecTraitF};
    /// let vec01 = Vec2::<f32>::new (1.0, 3.0);
    /// let vec02 = Vec2::<f32>::new (4.0, 9.0);
    ///
    /// let distance = vec01.distance (&vec02);
    /// ```
    fn distance (&self, rhs: &Vec2<V>) -> V {
        (*self - *rhs).length ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the length of a vector.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec2, VecTraitF};
    /// let vec = Vec2::<f32>::new (1.0, 3.0);
    /// let vec_length = vec.length ();
    /// ```
    fn length (&self) -> V {

        (self.x * self.x +
         self.y * self.y).sqrt ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a normalized vector.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec2, VecTraitF};
    /// let vec = Vec2::<f32>::new (3.0, 9.0);
    /// let vec_normalized = vec.normalize ();
    /// ```
    fn normalize (&self) -> Vec2<V> {

        let length = self.length ();

        if length != V::zero () {

            return Vec2::new (self.x / length,
                              self.y / length);
        }

        Vec2::zero ()
    }
}

/*===============================================================================================*/
/*------VEC2 OPERATORS---------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<V> Add for Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn add (self, rhs: Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x + rhs.x,
                   self.y + rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Add<&'a Vec2<V>> for Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn add (self, rhs: &Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x + rhs.x,
                   self.y + rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Add<Vec2<V>> for &'a Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn add (self, rhs: Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x + rhs.x,
                   self.y + rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Add for &'a Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn add (self, rhs: &Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x + rhs.x,
                   self.y + rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Add<V> for Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn add (self, rhs: V) -> Vec2<V> {

        Vec2::new (self.x + rhs,
                   self.y + rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Add<V> for &'a Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn add (self, rhs: V) -> Vec2<V> {

        Vec2::new (self.x + rhs,
                   self.y + rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> AddAssign for Vec2<V> where
    V: Copy + Num + NumCast {

    fn add_assign (&mut self, rhs: Vec2<V>) {

        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> AddAssign<V> for Vec2<V> where
    V: Copy + Num + NumCast {

    fn add_assign (&mut self, rhs: V) {

        self.x = self.x + rhs;
        self.y = self.y + rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Sub for Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn sub (self, rhs: Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x - rhs.x,
                   self.y - rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Sub<&'a Vec2<V>> for Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn sub (self, rhs: &Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x - rhs.x,
                   self.y - rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Sub<Vec2<V>> for &'a Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn sub (self, rhs: Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x - rhs.x,
                   self.y - rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Sub for &'a Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn sub (self, rhs: &Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x - rhs.x,
                   self.y - rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Sub<V> for Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn sub (self, rhs: V) -> Vec2<V> {

        Vec2::new (self.x - rhs,
                   self.y - rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Sub<V> for &'a Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn sub (self, rhs: V) -> Vec2<V> {

        Vec2::new (self.x - rhs,
                   self.y - rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> SubAssign for Vec2<V> where
    V: Copy + Num + NumCast {

    fn sub_assign (&mut self, rhs: Vec2<V>) {

        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> SubAssign<V> for Vec2<V> where
    V: Copy + Num + NumCast {

    fn sub_assign (&mut self, rhs: V) {

        self.x = self.x - rhs;
        self.y = self.y - rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Mul for Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn mul (self, rhs: Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x * rhs.x,
                   self.y * rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Mul<&'a Vec2<V>> for Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn mul (self, rhs: &Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x * rhs.x,
                   self.y * rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Mul<Vec2<V>> for &'a Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn mul (self, rhs: Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x * rhs.x,
                   self.y * rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Mul for &'a Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn mul (self, rhs: &Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x * rhs.x,
                   self.y * rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Mul<V> for Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn mul (self, rhs: V) -> Vec2<V> {

        Vec2::new (self.x * rhs,
                   self.y * rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Mul<V> for &'a Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn mul (self, rhs: V) -> Vec2<V> {

        Vec2::new (self.x * rhs,
                   self.y * rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> MulAssign for Vec2<V> where
    V: Copy + Num + NumCast {

    fn mul_assign (&mut self, rhs: Vec2<V>) {

        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> MulAssign<V> for Vec2<V> where
    V: Copy + Num + NumCast {

    fn mul_assign (&mut self, rhs: V) {

        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Div for Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn div (self, rhs: Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x / rhs.x,
                   self.y / rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Div<&'a Vec2<V>> for Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn div (self, rhs: &Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x / rhs.x,
                   self.y / rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Div<Vec2<V>> for &'a Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn div (self, rhs: Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x / rhs.x,
                   self.y / rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Div for &'a Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn div (self, rhs: &Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x / rhs.x,
                   self.y / rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Div<V> for Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn div (self, rhs: V) -> Vec2<V> {

        Vec2::new (self.x / rhs,
                   self.y / rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Div<V> for &'a Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn div (self, rhs: V) -> Vec2<V> {

        Vec2::new (self.x / rhs,
                   self.y / rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> DivAssign for Vec2<V> where
    V: Copy + Num + NumCast {

    fn div_assign (&mut self, rhs: Vec2<V>) {

        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> DivAssign<V> for Vec2<V> where
    V: Copy + Num + NumCast {

    fn div_assign (&mut self, rhs: V) {

        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Index<u8> for Vec2<V> where
    V: Copy + Num + NumCast {

    type Output = V;

    fn index (&self, index: u8) -> &V {

        match index {

            0 => &self.x,
            1 => &self.y,
            _ => unreachable! ("Index out of range for Vec2")
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> IndexMut<u8> for Vec2<V> where
    V: Copy + Num + NumCast {

    fn index_mut (&mut self, index: u8) -> &mut V {

        match index {

            0 => &mut self.x,
            1 => &mut self.y,
            _ => unreachable! ("Index out of range for Vec2")
        }
    }
}

/*===============================================================================================*/
/*------VEC2 PUBLIC METHODS----------------------------------------------------------------------*/
/*===============================================================================================*/

impl<V> Vec2<V> where
    V: Copy + Num + NumCast {

    /// Returns a new `Vec2<V>` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::new (3, 7);
    /// ```
    pub fn new<C> (x: C, y: C) -> Vec2<V> where
        C: Num + NumCast {

        Vec2 {x: V::from (x).unwrap (),
              y: V::from (y).unwrap ()}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec2<V>` with a value of (0, 1).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::up ();
    /// ```
    pub fn up () -> Vec2<V> {

        Vec2::new (V::zero (),
                   V::one  ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec2<V>` with a value of (0, -1).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::down ();
    /// ```
    pub fn down () -> Vec2<V> {

        Vec2::new (V::zero (),
                   V::from (-1).unwrap ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec2<V>` with a value of (0, 1).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::right ();
    /// ```
    pub fn right () -> Vec2<V> {

        Vec2::new (V::one  (),
                   V::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec2<V>` with a value of (0, -1).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::left ();
    /// ```
    pub fn left () -> Vec2<V> {

        Vec2::new (V::from (-1).unwrap (),
                   V::zero ())
    }
}
