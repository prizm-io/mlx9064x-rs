// SPDX-License-Identifier: Apache-2.0
//! MLX90642-specific EEPROM handling built on the MLX90641 pipeline.
use embedded_hal::blocking::i2c;

use crate::common::{CalibrationData, FromI2C};
use crate::error::{Error, LibraryError};
use crate::register::{AccessPattern, Resolution, Subpage};

use super::Mlx90642;

/// MLX90642-specific calibration wrapper.
///
/// The MLX90642 shares the same EEPROM layout as the MLX90641. Reuse the well-tested
/// `Mlx90641Calibration` implementation internally while exposing a dedicated type whose
/// [`CalibrationData::Camera`] maps to [`Mlx90642`].
#[derive(Clone, Debug, PartialEq)]
pub struct Mlx90642Calibration(crate::mlx90641::Mlx90641Calibration);

impl Mlx90642Calibration {
    /// Parse calibration values from raw EEPROM contents.
    pub fn from_data(data: &[u8]) -> Result<Self, LibraryError> {
        crate::mlx90641::Mlx90641Calibration::from_data(data).map(Self)
    }
}

impl<I2C> FromI2C<I2C> for Mlx90642Calibration
where
    I2C: i2c::WriteRead + i2c::Write,
{
    type Error = Error<I2C>;

    type Ok = Self;

    fn from_i2c(bus: &mut I2C, i2c_address: u8) -> Result<Self::Ok, Self::Error> {
        crate::mlx90641::Mlx90641Calibration::from_i2c(bus, i2c_address).map(Self)
    }
}

impl<'a> CalibrationData<'a> for Mlx90642Calibration {
    type Camera = Mlx90642;

    fn k_v_dd(&self) -> i16 {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::k_v_dd(&self.0)
    }

    fn v_dd_25(&self) -> i16 {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::v_dd_25(&self.0)
    }

    fn resolution(&self) -> Resolution {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::resolution(&self.0)
    }

    fn v_dd_0(&self) -> f32 {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::v_dd_0(&self.0)
    }

    fn k_v_ptat(&self) -> f32 {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::k_v_ptat(&self.0)
    }

    fn k_t_ptat(&self) -> f32 {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::k_t_ptat(&self.0)
    }

    fn v_ptat_25(&self) -> f32 {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::v_ptat_25(&self.0)
    }

    fn alpha_ptat(&self) -> f32 {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::alpha_ptat(&self.0)
    }

    fn gain(&self) -> f32 {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::gain(&self.0)
    }

    fn k_s_ta(&self) -> f32 {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::k_s_ta(&self.0)
    }

    fn corner_temperatures(&self) -> &[i16] {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::corner_temperatures(&self.0)
    }

    fn k_s_to(&self) -> &[f32] {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::k_s_to(&self.0)
    }

    fn alpha_correction(&self) -> &[f32] {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::alpha_correction(&self.0)
    }

    fn emissivity(&self) -> Option<f32> {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::emissivity(&self.0)
    }

    type OffsetReferenceIterator =
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::OffsetReferenceIterator;

    fn offset_reference_pixels(&'a self, subpage: Subpage) -> Self::OffsetReferenceIterator {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::offset_reference_pixels(
            &self.0, subpage,
        )
    }

    fn offset_reference_cp(&self, subpage: Subpage) -> i16 {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::offset_reference_cp(
            &self.0, subpage,
        )
    }

    type AlphaIterator =
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::AlphaIterator;

    fn alpha_pixels(&'a self, subpage: Subpage) -> Self::AlphaIterator {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::alpha_pixels(
            &self.0, subpage,
        )
    }

    fn alpha_cp(&self, subpage: Subpage) -> f32 {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::alpha_cp(&self.0, subpage)
    }

    type KvIterator = <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::KvIterator;

    fn k_v_pixels(&'a self, subpage: Subpage) -> Self::KvIterator {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::k_v_pixels(&self.0, subpage)
    }

    fn k_v_cp(&self, subpage: Subpage) -> f32 {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::k_v_cp(&self.0, subpage)
    }

    type KtaIterator = <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::KtaIterator;

    fn k_ta_pixels(&'a self, subpage: Subpage) -> Self::KtaIterator {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::k_ta_pixels(&self.0, subpage)
    }

    fn k_ta_cp(&self, subpage: Subpage) -> f32 {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::k_ta_cp(&self.0, subpage)
    }

    fn temperature_gradient_coefficient(&self) -> Option<f32> {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::temperature_gradient_coefficient(
            &self.0,
        )
    }

    type AccessPatternCompensation =
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::AccessPatternCompensation;

    fn access_pattern_compensation_pixels(
        &'a self,
        access_pattern: AccessPattern,
    ) -> Self::AccessPatternCompensation {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::access_pattern_compensation_pixels(
            &self.0,
            access_pattern,
        )
    }

    fn access_pattern_compensation_cp(
        &self,
        subpage: Subpage,
        access_pattern: AccessPattern,
    ) -> Option<f32> {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::access_pattern_compensation_cp(
            &self.0,
            subpage,
            access_pattern,
        )
    }

    type FailedPixels = <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::FailedPixels;

    fn failed_pixels(&'a self) -> Self::FailedPixels {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::failed_pixels(&self.0)
    }

    type OutlierPixels =
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::OutlierPixels;

    fn outlier_pixels(&'a self) -> Self::OutlierPixels {
        <crate::mlx90641::Mlx90641Calibration as CalibrationData<'a>>::outlier_pixels(&self.0)
    }
}
