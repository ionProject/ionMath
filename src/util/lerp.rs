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
/*------LERP TRAIT-------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The lerp trait.
pub trait Lerp {

    /// Linearly interpolates between two values.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::util::Lerp;
    /// println! ("{}", f32::lerp (&1.0, &7.5, 0.5));
    /// ```
    fn lerp (start: &Self, end: &Self, percentage: f32) -> Self;

    /// Linearly interpolates between two values without clamping.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::util::Lerp;
    /// println! ("{}", f32::lerp (&1.0, &7.5, 0.5));
    /// ```
    fn lerp_unclamped (start: &Self, end: &Self, percentage: f32) -> Self;
}
