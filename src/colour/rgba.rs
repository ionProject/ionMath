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
use ::util::{Clamp, Lerp};
use ::vector::{Vec2, Vec3, Vec4};

use std::convert::From;

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
/*------RGBA TRAIT IMPLEMENTATIONS---------------------------------------------------------------*/
/*===============================================================================================*/

impl Clamp for RGBA {

    fn clamp (self, min: Self, max: Self) -> Self {

        debug_assert! (min.r < max.r, "Min cannot be greater than max.");
        debug_assert! (min.g < max.g, "Min cannot be greater than max.");
        debug_assert! (min.b < max.b, "Min cannot be greater than max.");
        debug_assert! (min.a < max.a, "Min cannot be greater than max.");

        let rval = if self.r < min.r {min.r} else if self.r > max.r {max.r} else {self.r};
        let gval = if self.g < min.g {min.g} else if self.g > max.g {max.g} else {self.g};
        let bval = if self.b < min.b {min.b} else if self.b > max.b {max.b} else {self.b};
        let aval = if self.a < min.a {min.a} else if self.a > max.a {max.a} else {self.a};

        RGBA::new (rval, gval, bval, aval)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for RGBA {

    fn default () -> Self {
        RGBA::from (1.0)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl From<f32> for RGBA {

    fn from (value: f32) -> Self {
        RGBA::new (value, value, value, value)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> From<&'a Vec2<T>> for RGBA where
    T: Copy + Num + NumCast {

    fn from (value: &Vec2<T>) -> Self {

        RGBA::new (value.x.to_f32 ().unwrap (),
                   value.y.to_f32 ().unwrap (),
                   0.0, 1.0)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> From<&'a Vec3<T>> for RGBA where
    T: Copy + Num + NumCast {

    fn from (value: &Vec3<T>) -> Self {

        RGBA::new (value.x.to_f32 ().unwrap (),
                   value.y.to_f32 ().unwrap (),
                   value.z.to_f32 ().unwrap (),
                   1.0)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> From<&'a Vec4<T>> for RGBA where
    T: Copy + Num + NumCast {


    fn from (value: &Vec4<T>) -> Self {

        RGBA::new (value.x.to_f32 ().unwrap (),
                   value.y.to_f32 ().unwrap (),
                   value.z.to_f32 ().unwrap (),
                   value.w.to_f32 ().unwrap ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Lerp for RGBA {

    fn lerp (start: &Self, end: &Self, percentage: f32) -> Self {

        RGBA::new (f32::lerp (&start.r, &end.r, percentage),
                   f32::lerp (&start.g, &end.g, percentage),
                   f32::lerp (&start.b, &end.b, percentage),
                   f32::lerp (&start.a, &end.a, percentage))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn lerp_unclamped (start: &Self, end: &Self, percentage: f32) -> Self {

        RGBA::new (f32::lerp_unclamped (&start.r, &end.r, percentage),
                   f32::lerp_unclamped (&start.g, &end.g, percentage),
                   f32::lerp_unclamped (&start.b, &end.b, percentage),
                   f32::lerp_unclamped (&start.a, &end.a, percentage))
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl ColourTrait for RGBA {

    /// Returns the colour black.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::black ();
    /// ```
    fn black () -> Self {
        RGBA::new (0.0, 0.0, 0.0, 1.0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour light grey.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::light_grey ();
    /// ```
    fn light_grey () -> Self {
        RGBA::new (0.75, 0.75, 0.75, 1.0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour grey.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::grey ();
    /// ```
    fn grey () -> Self {
        RGBA::new (0.5, 0.5, 0.5, 1.0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour dark grey.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::dark_grey ();
    /// ```
    fn dark_grey () -> Self {
        RGBA::new (0.25, 0.25, 0.25, 1.0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour white.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::white ();
    /// ```
    fn white () -> Self {
        RGBA::from (1.0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour red.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::red ();
    /// ```
    fn red () -> Self {
        RGBA::new (1.0, 0.0, 0.0, 1.0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour green.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::green ();
    /// ```
    fn green () -> Self {
        RGBA::new (0.0, 1.0, 0.0, 1.0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour blue.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::blue ();
    /// ```
    fn blue () -> Self {
        RGBA::new (0.0, 0.0, 1.0, 1.0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour yellow.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::yellow ();
    /// ```
    fn yellow () -> Self {
        RGBA::new (1.0, 1.0, 0.0, 1.0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour cyan.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::cyan ();
    /// ```
    fn cyan () -> Self {
        RGBA::new (0.0, 1.0, 1.0, 1.0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour magenta.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::magenta ();
    /// ```
    fn magenta () -> Self {
        RGBA::new (1.0, 0.0, 1.0, 1.0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a fully transparent colour.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::transparent ();
    /// ```
    fn transparent () -> Self {
        RGBA::from (0.0)
    }
}

/*===============================================================================================*/
/*------RGBA PUBLIC METHODS----------------------------------------------------------------------*/
/*===============================================================================================*/

impl RGBA {
    /// Returns a new `RGBA` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::RGBA;
    /// let colour = RGBA::new (0.0, 1.0, 0.5, 1.0);
    /// ```
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        RGBA { r: r, g: g, b: b, a: a }
    }
}
