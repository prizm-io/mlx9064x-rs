// SPDX-License-Identifier: Apache-2.0
//! MLX90642-specific EEPROM handling built on the MLX90641 pipeline.
use embedded_hal::blocking::i2c;

use crate::common::{Address, CalibrationData, FromI2C};
use crate::error::{Error, LibraryError};
use crate::register::{AccessPattern, Resolution, Subpage};

use super::Mlx90642;
use crate::mlx90641::hamming::add_checksum;

/// Length of the MLX90642 EEPROM region in bytes.
///
/// The MLX90642 uses the same address window (0x2400-0x273F) as the MLX90641, but the contents do
/// not include the MLX90641-specific Hamming checksum bits. Each address holds a 16-bit word, so the
/// usable length is the number of addresses multiplied by two.
const EEPROM_LENGTH: usize = (0x2740 - 0x2400) * 2;

/// Base address of the EEPROM window.
const EEPROM_BASE: Address = Address::new(0x2400);

/// Number of data bits present in the MLX90641 Hamming codewords.
const HAMMING_DATA_MASK: u16 = 0x07FF;

/// MLX90642-specific calibration wrapper.
///
/// The MLX90642 shares the same EEPROM layout as the MLX90641. Reuse the well-tested
/// `Mlx90641Calibration` implementation internally while exposing a dedicated type whose
/// [`CalibrationData::Camera`] maps to [`Mlx90642`].
#[derive(Clone, Debug, PartialEq)]
pub struct Mlx90642Calibration(crate::mlx90641::Mlx90641Calibration);

impl Mlx90642Calibration {
    /// Parse calibration values from raw EEPROM contents.
    ///
    /// The MLX90642 EEPROM omits the MLX90641's per-word Hamming checksum, so try the native parser
    /// first and then synthesize the missing checksum bits if parsing fails with
    /// [`LibraryError::Checksum`].
    pub fn from_data(data: &[u8]) -> Result<Self, LibraryError> {
        if data.len() != EEPROM_LENGTH {
            return Err(LibraryError::InvalidData(
                "MLX90642 EEPROM dump has an unexpected length",
            ));
        }

        Self::parse_mlx90641_calibration(data).map(Self)
    }

    fn parse_mlx90641_calibration(
        data: &[u8],
    ) -> Result<crate::mlx90641::Mlx90641Calibration, LibraryError> {
        match crate::mlx90641::Mlx90641Calibration::from_data(data) {
            Ok(calibration) => Ok(calibration),
            Err(LibraryError::Checksum(_)) => {
                let corrected = Self::synthesize_checksums(data)?;
                crate::mlx90641::Mlx90641Calibration::from_data(&corrected)
            }
            Err(err) => Err(err),
        }
    }

    fn synthesize_checksums(data: &[u8]) -> Result<[u8; EEPROM_LENGTH], LibraryError> {
        if data.len() != EEPROM_LENGTH {
            return Err(LibraryError::InvalidData(
                "MLX90642 EEPROM dump has an unexpected length",
            ));
        }

        let mut corrected = [0u8; EEPROM_LENGTH];
        for (dest, src) in corrected
            .chunks_exact_mut(2)
            .zip(data.chunks_exact(2))
        {
            let raw_word = u16::from_be_bytes([src[0], src[1]]);
            let data_bits = raw_word & HAMMING_DATA_MASK;
            let with_checksum = add_checksum(data_bits)?;
            dest.copy_from_slice(&with_checksum.to_be_bytes());
        }
        Ok(corrected)
    }
}

impl<I2C> FromI2C<I2C> for Mlx90642Calibration
where
    I2C: i2c::WriteRead + i2c::Write,
{
    type Error = Error<I2C>;

    type Ok = Self;

    fn from_i2c(bus: &mut I2C, i2c_address: u8) -> Result<Self::Ok, Self::Error> {
        let mut eeprom_buf = [0u8; EEPROM_LENGTH];
        bus.write_read(i2c_address, &EEPROM_BASE.as_bytes(), &mut eeprom_buf)
            .map_err(Error::I2cWriteReadError)?;
        Self::from_data(&eeprom_buf).map_err(Error::from)
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

#[cfg(test)]
mod test {
    use super::Mlx90642Calibration;
    use crate::error::LibraryError;
    use mlx9064x_test_data::mlx90641_datasheet_eeprom;

    #[test]
    fn parses_eeprom_with_checksums() {
        let data = mlx90641_datasheet_eeprom();
        Mlx90642Calibration::from_data(&data).expect("checksum-bearing data should parse");
    }

    #[test]
    fn synthesizes_missing_checksums() {
        let mut data = mlx90641_datasheet_eeprom();
        for chunk in data.chunks_exact_mut(2) {
            let word = u16::from_be_bytes([chunk[0], chunk[1]]);
            let without_checksum = (word & super::HAMMING_DATA_MASK).to_be_bytes();
            chunk.copy_from_slice(&without_checksum);
        }

        Mlx90642Calibration::from_data(&data).expect("missing checksums should be synthesized");
    }

    #[test]
    fn rejects_invalid_lengths() {
        let data = [0u8; 4];
        let err = Mlx90642Calibration::from_data(&data).unwrap_err();
        assert!(matches!(err, LibraryError::InvalidData(_)));
    }
}
