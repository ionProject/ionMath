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

use ::vector::{Vec2, Vec4, VecTrait, VecTraitF};
use ::util;

use std::ops::{Add,   AddAssign,
               Sub,   SubAssign,
               Mul,   MulAssign,
               Div,   DivAssign,
               Index, IndexMut};
use std::convert::From;

/*===============================================================================================*/
/*------VEC3 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The generic Vec3 struct.
///
/// It is used for 3D transformations and graphics.
/// It can accept any number as a value.
#[cfg_attr (feature = "serde_serialize", derive (Deserialize, Serialize))]
#[derive (Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3<V> where
    V: Copy + Num + NumCast {

    // Public
    /// The vector x-coordinate.
    pub x: V,
    /// The vector y-coordinate.
    pub y: V,
    /// The vector z-coordinate.
    pub z: V,
}

// Predefined Vec3 types
/// `Vec3<f32>`
pub type Vec3f = Vec3<f32>;
/// `Vec3<i32>`
pub type Vec3i = Vec3<i32>;
/// `Vec3<u32>`
pub type Vec3u = Vec3<u32>;

/*===============================================================================================*/
/*------VEC3 TRAIT IMPLEMENTATIONS---------------------------------------------------------------*/
/*===============================================================================================*/

impl<V, U> From<U> for Vec3<V> where
    V: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: U) -> Vec3<V> {

        Vec3::new (V::from (value).unwrap (),
                   V::from (value).unwrap (),
                   V::from (value).unwrap ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V, U> From<&'a Vec2<U>> for Vec3<V> where
    V: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: &Vec2<U>) -> Vec3<V> {

        Vec3::new (V::from (value.x).unwrap (),
                   V::from (value.y).unwrap (),
                   V::zero ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V, U> From<&'a Vec3<U>> for Vec3<V> where
    V: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: &Vec3<U>) -> Vec3<V> {

        Vec3::new (V::from (value.x).unwrap (),
                   V::from (value.y).unwrap (),
                   V::from (value.z).unwrap ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V, U> From<&'a Vec4<U>> for Vec3<V> where
    V: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: &Vec4<U>) -> Vec3<V> {

        Vec3::new (V::from (value.x).unwrap (),
                   V::from (value.y).unwrap (),
                   V::from (value.z).unwrap ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> VecTrait for Vec3<V> where
    V: Copy + Default + Num + NumCast + PartialOrd {

    type ValType = V;


    fn clamp (&self, min: &Self, max: &Self) -> Self {

        Vec3::new (util::clamp (self.x, min.x, max.x),
                   util::clamp (self.y, min.y, max.y),
                   util::clamp (self.z, min.z, max.z))
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the dot product of two vectors.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec3, VecTrait};
    /// let vec01 = Vec3::<f32>::new (1.0, 3.0, 0.0);
    /// let vec02 = Vec3::<f32>::new (4.0, 9.0, 0.0);
    ///
    /// let dot_product = vec01.dot (&vec02);
    /// ```
    fn dot (&self, rhs: &Vec3<V>) -> V {

        (self.x * rhs.x) +
        (self.y * rhs.y) +
        (self.z * rhs.z)
    }

/*-----------------------------------------------------------------------------------------------*/

    fn lerp (start: &Self, end: &Self, percentage: f32) -> Self {

        Vec3::new (util::lerp (start.x, end.x, percentage),
                   util::lerp (start.y, end.y, percentage),
                   util::lerp (start.z, end.z, percentage))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn min (lhs: &Self, rhs: &Self) -> Self {

        Vec3::new (util::min (lhs.x, rhs.x),
                   util::min (lhs.y, rhs.y),
                   util::min (lhs.z, rhs.z))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn max (lhs: &Self, rhs: &Self) -> Self {

        Vec3::new (util::max (lhs.x, rhs.x),
                   util::max (lhs.y, rhs.y),
                   util::max (lhs.z, rhs.z))
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> VecTraitF for Vec3<V> where
    V: Default + Float {

    type ValTypeF = V;

    /// Returns the distance between two vectors.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec3, VecTraitF};
    /// let vec01 = Vec3::<f32>::new (1.0, 3.0, 0.0);
    /// let vec02 = Vec3::<f32>::new (4.0, 9.0, 0.0);
    ///
    /// let distance = vec01.distance (&vec02);
    /// ```
    fn distance<'a> (&'a self, rhs: &'a Vec3<V>) -> V {
        (self - rhs).length ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the length of a vector.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec3, VecTraitF};
    /// let vec = Vec3::<f32>::new (1.0, 3.0, 0.0);
    /// let vec_length = vec.length ();
    /// ```
    fn length (&self) -> V {

        (self.x * self.x +
         self.y * self.y +
         self.z * self.z).sqrt ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a normalized vector.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec3, VecTraitF};
    /// let vec = Vec3::<f32>::new (3.0, 9.0, 0.0);
    /// let vec_normalized = vec.normalize ();
    /// ```
    fn normalize (&self) -> Vec3<V> {

        let length = self.length ();

        if length != V::zero () {

            return Vec3::new (self.x / length,
                              self.y / length,
                              self.z / length);
        }

        Vec3::default ()
    }
}

/*===============================================================================================*/
/*------VEC3 OPERATORS---------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<V> Add for Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn add (self, rhs: Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x + rhs.x,
                   self.y + rhs.y,
                   self.z + rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Add<&'a Vec3<V>> for Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn add (self, rhs: &Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x + rhs.x,
                   self.y + rhs.y,
                   self.z + rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Add<Vec3<V>> for &'a Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn add (self, rhs: Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x + rhs.x,
                   self.y + rhs.y,
                   self.z + rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Add for &'a Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn add (self, rhs: &Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x + rhs.x,
                   self.y + rhs.y,
                   self.z + rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Add<V> for Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn add (self, rhs: V) -> Vec3<V> {

        Vec3::new (self.x + rhs,
                   self.y + rhs,
                   self.z + rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Add<V> for &'a Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn add (self, rhs: V) -> Vec3<V> {

        Vec3::new (self.x + rhs,
                   self.y + rhs,
                   self.z + rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> AddAssign for Vec3<V> where
    V: Copy + Num + NumCast {

    fn add_assign (&mut self, rhs: Vec3<V>) {

        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> AddAssign<V> for Vec3<V> where
    V: Copy + Num + NumCast {

    fn add_assign (&mut self, rhs: V) {

        self.x = self.x + rhs;
        self.y = self.y + rhs;
        self.z = self.z + rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Sub for Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn sub (self, rhs: Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x - rhs.x,
                   self.y - rhs.y,
                   self.z - rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Sub<&'a Vec3<V>> for Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn sub (self, rhs: &Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x - rhs.x,
                   self.y - rhs.y,
                   self.z - rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Sub<Vec3<V>> for &'a Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn sub (self, rhs: Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x - rhs.x,
                   self.y - rhs.y,
                   self.z - rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Sub for &'a Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn sub (self, rhs: &Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x - rhs.x,
                   self.y - rhs.y,
                   self.z - rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Sub<V> for Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn sub (self, rhs: V) -> Vec3<V> {

        Vec3::new (self.x - rhs,
                   self.y - rhs,
                   self.z - rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Sub<V> for &'a Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn sub (self, rhs: V) -> Vec3<V> {

        Vec3::new (self.x - rhs,
                   self.y - rhs,
                   self.z - rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> SubAssign for Vec3<V> where
    V: Copy + Num + NumCast {

    fn sub_assign (&mut self, rhs: Vec3<V>) {

        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> SubAssign<V> for Vec3<V> where
    V: Copy + Num + NumCast {

    fn sub_assign (&mut self, rhs: V) {

        self.x = self.x - rhs;
        self.y = self.y - rhs;
        self.z = self.z - rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Mul for Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn mul (self, rhs: Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x * rhs.x,
                   self.y * rhs.y,
                   self.z * rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Mul<&'a Vec3<V>> for Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn mul (self, rhs: &Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x * rhs.x,
                   self.y * rhs.y,
                   self.z * rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Mul<Vec3<V>> for &'a Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn mul (self, rhs: Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x * rhs.x,
                   self.y * rhs.y,
                   self.z * rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Mul for &'a Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn mul (self, rhs: &Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x * rhs.x,
                   self.y * rhs.y,
                   self.z * rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Mul<V> for Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn mul (self, rhs: V) -> Vec3<V> {

        Vec3::new (self.x * rhs,
                   self.y * rhs,
                   self.z * rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Mul<V> for &'a Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn mul (self, rhs: V) -> Vec3<V> {

        Vec3::new (self.x * rhs,
                   self.y * rhs,
                   self.z * rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> MulAssign for Vec3<V> where
    V: Copy + Num + NumCast {

    fn mul_assign (&mut self, rhs: Vec3<V>) {

        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> MulAssign<V> for Vec3<V> where
    V: Copy + Num + NumCast {

    fn mul_assign (&mut self, rhs: V) {

        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Div for Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn div (self, rhs: Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x / rhs.x,
                   self.y / rhs.y,
                   self.z / rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Div<&'a Vec3<V>> for Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn div (self, rhs: &Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x / rhs.x,
                   self.y / rhs.y,
                   self.z / rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Div<Vec3<V>> for &'a Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn div (self, rhs: Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x / rhs.x,
                   self.y / rhs.y,
                   self.z / rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Div for &'a Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn div (self, rhs: &Vec3<V>) -> Vec3<V> {

        Vec3::new (self.x / rhs.x,
                   self.y / rhs.y,
                   self.z / rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Div<V> for Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn div (self, rhs: V) -> Vec3<V> {

        Vec3::new (self.x / rhs,
                   self.y / rhs,
                   self.z / rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, V> Div<V> for &'a Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = Vec3<V>;

    fn div (self, rhs: V) -> Vec3<V> {

        Vec3::new (self.x / rhs,
                   self.y / rhs,
                   self.z / rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> DivAssign for Vec3<V> where
    V: Copy + Num + NumCast {

    fn div_assign (&mut self, rhs: Vec3<V>) {

        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> DivAssign<V> for Vec3<V> where
    V: Copy + Num + NumCast {

    fn div_assign (&mut self, rhs: V) {

        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Index<u8> for Vec3<V> where
    V: Copy + Num + NumCast {

    type Output = V;

    fn index (&self, index: u8) -> &V {

        match index {

            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable! ("Index out of range for Vec3")
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> IndexMut<u8> for Vec3<V> where
    V: Copy + Num + NumCast {

    fn index_mut (&mut self, index: u8) -> &mut V {

        match index {

            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unreachable! ("Index out of range for Vec3")
        }
    }
}

/*===============================================================================================*/
/*------VEC3 PUBLIC METHODS----------------------------------------------------------------------*/
/*===============================================================================================*/

impl<V> Vec3<V> where
    V: Copy + Num + NumCast {

    /// Returns the cross product of two vectors.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec01 = Vec3::<f32>::new (1, 3, 6);
    /// let vec02 = Vec3::<f32>::new (4, 9, 2);
    ///
    /// let cross_product = vec01.cross (&vec02);
    /// ```
    pub fn cross (&self, rhs: &Vec3<V>) -> Vec3<V> {

        Vec3::new ((self.y * rhs.z) - (self.z * rhs.y),
                   (self.z * rhs.x) - (self.x * rhs.z),
                   (self.x * rhs.y) - (self.y * rhs.x))
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a new `Vec3<V>` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec = Vec3::<f32>::new (3, 7, 10);
    /// ```
    pub fn new<C> (x: C, y: C, z: C) -> Vec3<V> where
        C: Num + NumCast {

        Vec3 {x: V::from (x).unwrap (),
              y: V::from (y).unwrap (),
              z: V::from (z).unwrap ()}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec3<V>` with a value of (0, 1, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec = Vec3::<f32>::up ();
    /// ```
    pub fn up () -> Vec3<V> {

        Vec3::new (V::zero (),
                   V::one  (),
                   V::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec3<V>` with a value of (0, -1, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec = Vec3::<f32>::down ();
    /// ```
    pub fn down () -> Vec3<V> {

        Vec3::new (V::zero (),
                   V::from (-1).unwrap (),
                   V::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec3<V>` with a value of (1, 0, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec = Vec3::<f32>::right ();
    /// ```
    pub fn right () -> Vec3<V> {

        Vec3::new (V::one  (),
                   V::zero (),
                   V::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec3<V>` with a value of (-1, 0, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec = Vec3::<f32>::left ();
    /// ```
    pub fn left () -> Vec3<V> {

        Vec3::new (V::from (-1).unwrap (),
                   V::zero (),
                   V::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec3<V>` with a value of (0, 0, 1)
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec = Vec3::<f32>::forward ();
    /// ```
    pub fn forward () -> Vec3<V> {

        Vec3::new (V::zero (),
                   V::zero (),
                   V::one  ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec3<V>` with a value of (0, 0, -1)
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec = Vec3::<f32>::back ();
    /// ```
    pub fn back () -> Vec3<V> {

        Vec3::new (V::zero (),
                   V::zero (),
                   V::from (-1).unwrap ())
    }
}
