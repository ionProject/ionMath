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

/*===============================================================================================*/
//! Includes an array of small utility functions.
/*===============================================================================================*/

// Crate imports
extern crate num_traits;

// Module imports
use self::num_traits::{Num, NumCast};

/*===============================================================================================*/
/*------PUBLIC FUNCTIONS-------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Clamps a value between two numbers.
///
/// # Examples
/// ```
/// # use ion_math::util;
/// let health = 121;
/// let min_health = 0;
/// let max_health = 100;
///
/// let current_health = util::clamp (health, min_health, max_health);
/// ```
pub fn clamp<T> (value: T, min: T, max: T) -> T where
    T: Copy + Num + PartialOrd {

    debug_assert! (min < max, "Min cannot be greater than max.");
    if value < min {min} else if value > max {max} else {value}
}

/*-----------------------------------------------------------------------------------------------*/

/// Linearly interpolates between two values.
///
/// # Examples
/// ```
/// # use ion_math::util;
/// let v = util::lerp (1, 256, 0.5);
/// ```
pub fn lerp<T> (start: T, end: T, percentage: f32) -> T where
    T: Copy + Num + NumCast + PartialOrd {

    T::from ((start + (end - start)).to_f32 ().unwrap () * clamp (percentage, 0.0, 1.0)).unwrap ()
}

/*-----------------------------------------------------------------------------------------------*/

/// Returns the largest of two values.
///
/// # Examples
/// ```
/// # use ion_math::util;
/// let num = util::max (43, 7);
/// ```
pub fn max<T> (lhs: T, rhs: T) -> T where
    T: Copy + Num + PartialOrd {

    if lhs > rhs {lhs} else {rhs}
}

/*-----------------------------------------------------------------------------------------------*/

/// Returns the smallest of two values.
///
/// # Examples
/// ```
/// # use ion_math::util;
/// let num = util::min (43, 7);
/// ```
pub fn min<T> (lhs: T, rhs: T) -> T where
    T: Copy + Num + PartialOrd {

    if lhs < rhs {lhs} else {rhs}
}
