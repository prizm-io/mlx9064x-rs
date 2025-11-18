// SPDX-License-Identifier: Apache-2.0
//! MLX90642 specific details.
mod eeprom;

use core::iter;

use crate::common::{Address, MelexisCamera};
use crate::register::{AccessPattern, Resolution, Subpage};
use crate::util::Sealed;

pub use crate::mlx90641::RamAddress;
pub use eeprom::Mlx90642Calibration;

/// MLX90642-specific constants and supporting functions.
///
/// The MLX90642 shares the same memory map and per-frame behaviour as the MLX90641. The
/// high-level driver still provides a dedicated type so applications can differentiate between the
/// two sensor families while re-using the same calibration pipeline internally.
#[derive(Clone, Debug, PartialEq)]
pub struct Mlx90642();

impl Sealed for Mlx90642 {}

impl MelexisCamera for Mlx90642 {
    type PixelRangeIterator = crate::mlx90641::SubpageInterleave;
    type PixelsInSubpageIterator = iter::Take<iter::Repeat<bool>>;

    fn pixel_ranges(subpage: Subpage, access_pattern: AccessPattern) -> Self::PixelRangeIterator {
        <crate::mlx90641::Mlx90641 as MelexisCamera>::pixel_ranges(subpage, access_pattern)
    }

    fn pixels_in_subpage(
        subpage: Subpage,
        access_pattern: AccessPattern,
    ) -> Self::PixelsInSubpageIterator {
        <crate::mlx90641::Mlx90641 as MelexisCamera>::pixels_in_subpage(subpage, access_pattern)
    }

    const T_A_V_BE: Address = <crate::mlx90641::Mlx90641 as MelexisCamera>::T_A_V_BE;

    const T_A_PTAT: Address = <crate::mlx90641::Mlx90641 as MelexisCamera>::T_A_PTAT;

    fn compensation_pixel(subpage: Subpage) -> Address {
        <crate::mlx90641::Mlx90641 as MelexisCamera>::compensation_pixel(subpage)
    }

    const GAIN: Address = <crate::mlx90641::Mlx90641 as MelexisCamera>::GAIN;

    const V_DD_PIXEL: Address = <crate::mlx90641::Mlx90641 as MelexisCamera>::V_DD_PIXEL;

    fn resolution_correction(
        calibrated_resolution: Resolution,
        current_resolution: Resolution,
    ) -> f32 {
        <crate::mlx90641::Mlx90641 as MelexisCamera>::resolution_correction(
            calibrated_resolution,
            current_resolution,
        )
    }

    const BASIC_TEMPERATURE_RANGE: usize =
        <crate::mlx90641::Mlx90641 as MelexisCamera>::BASIC_TEMPERATURE_RANGE;

    const SELF_HEATING: f32 = <crate::mlx90641::Mlx90641 as MelexisCamera>::SELF_HEATING;

    const HEIGHT: usize = <crate::mlx90641::Mlx90641 as MelexisCamera>::HEIGHT;

    const WIDTH: usize = <crate::mlx90641::Mlx90641 as MelexisCamera>::WIDTH;

    const NUM_PIXELS: usize = <crate::mlx90641::Mlx90641 as MelexisCamera>::NUM_PIXELS;
}
