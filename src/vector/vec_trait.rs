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
use self::num_traits::{Float, Num};

/*===============================================================================================*/
/*------VECTOR TRAIT-----------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Implemented by all vector types.
pub trait VecTrait:
    Default {

    /// The vector component type.
    type ValType: Num;

    /// Lerps between two vectors.
    fn lerp (start: &Self, end: &Self, percentage: f32) -> Self;
    /// Returns the largest components of two vectors.
    fn max  (lhs: &Self, rhs: &Self) -> Self;
    /// Returns the smallest components of two vectors.
    fn min  (lhs: &Self, rhs: &Self) -> Self;

    /// Clamps a vector between two values.
    fn clamp (&self, min: &Self, max: &Self) -> Self;
    /// Returns the dot product of two vectors.
    fn dot (&self, rhs: &Self) -> Self::ValType;
}

/*-----------------------------------------------------------------------------------------------*/

/// Implemented by vectors using float values.
pub trait VecTraitF:
    VecTrait {

    /// The vector component type.
    type ValTypeF: Float;

    /// Returns the distance between two vectors.
    fn distance (&self, rhs: &Self) -> Self::ValTypeF;
    /// Returns the length of a vector.
    fn length (&self) -> Self::ValTypeF;
    /// Normalizes a vector.
    fn normalize (&self) -> Self;
}
