//! # Build iso8583 messages.

use super::data;
use std::borrow::Cow;

/// i8583::Packer type and associated functions for creating iso8583 messages.
///
/// # Examples
///
/// ```
/// use i8583::Packer;
///
/// let mut iso_packer = Packer::new(&i8583::nibss::SPEC);
///
/// let pan = String::from("5432101234567898");
///
/// if let Err(msg) = iso_packer.set_field(0, "0200".as_bytes().into()) {
///     eprintln!("{}", msg);
///     return;
/// }
///
/// if let Err(msg) = iso_packer.set_field(2, pan.into_bytes().into()) {
///     eprintln!("{}", msg);
///     return;
/// }
///
/// let iso = match iso_packer.pack(Vec::with_capacity(1024)) {
///     Ok(iso) => iso,
///     Err(msg) => {
///         eprintln!("{}", msg);
///         return;
///     }
/// };
/// let iso = String::from_utf8(iso).unwrap();
/// println!("{}", iso);
/// ```
pub struct Packer<'a> {
    spec: &'a [Option<data::Spec>; 129],
    bitmap: u128,
    data_elements: Vec<(Option<String>, Option<Cow<'a, [u8]>>)>,
}

impl<'a> Packer<'a> {
    /// Create a new packer with the given iso specification.
    /// The provided spec will be used to validate [set_field](#method.set_field) inputs
    pub fn new(spec: &'a [Option<data::Spec>; 129]) -> Packer {
        Packer {
            spec: spec,
            bitmap: 0,
            data_elements: vec![(None, None); 129],
        }
    }

    /// Manually set bit 64 to on or off.
    /// # Rationale
    /// Message authentication codes (MAC) can only be generated after
    /// obtaining a full iso 8583 message, this function provides a
    /// way to indicate the presence of a MAC in the bitmap while
    /// defering the computation and addition of the actual
    /// message authentication code.
    /// # Warning
    /// Do not set both [toggle_bit_64](#method.toggle_bit_64) and
    /// [toggle_bit_128](#method.toggle_bit_128) to on.
    ///
    /// Use of this function to set bit 64 to on will render [pack](#method.pack)
    /// output bitmap incorrect, a MAC must be appended to re-establish correctness.
    pub fn toggle_bit_64(&mut self, val: bool) {
        if val {
            self.bitmap = self.bitmap | (1 << 64);
        } else {
            self.bitmap = self.bitmap & (!1 << 64);   
        }
    }

    /// Manually set bit 128 to on or off.
    /// # Rationale
    /// Message authentication codes (MAC) can only be generated after
    /// obtaining a full iso 8583 message, this function provides a
    /// way to indicate the presence of a MAC in the bitmap while
    /// defering the computation and addition of the actual
    /// message authentication code.
    /// # Warning
    /// Do not set both [toggle_bit_64](#method.toggle_bit_64) and
    /// [toggle_bit_128](#method.toggle_bit_128) to on.
    ///
    /// Use of this function to set bit 128 to on will render [pack](#method.pack)
    /// output bitmap incorrect, a MAC must be appended to re-establish correctness.
    pub fn toggle_bit_128(&mut self, val: bool) {
        if val {
            self.bitmap = self.bitmap | 1;
        } else {
            self.bitmap = self.bitmap & !1;   
        }
    }

    /// Set val for field in iso message. This will replace existing value if previously set
    /// # Errors
    /// Will fail if specification is not found for field or val size is invalid
    pub fn set_field(&mut self, field: usize, val: Cow<'a, [u8]>) -> Result<(), String> {
        let element = match &self.spec[field] {
            Some(element) => element,
            None => return Err(format!("No specification found for field {}", field)),
        };

        let len = match Packer::validate_input_size(&val, &element.data_size) {
            Ok(len) => len,
            Err(msg) => return Err(format!("Field [{}]: {}", field, msg)),
        };

        if field > 0 {
            self.bitmap = self.bitmap | 1 << (128 - field);
        }

        self.data_elements[field] = (Some(len), Some(val));

        Ok(())
    }

    /// Unset val for field in iso message.
    pub fn unset_field(&mut self, field: usize) {
        if field > 0 {
            self.bitmap = self.bitmap & (!1 << (128 - field));
        }
        self.data_elements[field] = (None, None);
    }

    /// Generate iso message. Call this function after setting all fields
    /// # Errors
    /// Will fail if MTI(field 0) is not set
    pub fn pack(&self, buffer: Vec<u8>) -> Result<Vec<u8>, &'static str> {
        let mut buffer = buffer;
        let mut iso = match &self.data_elements[0] {
            (_, Some(mti)) => {
                buffer.extend_from_slice(mti);
                buffer
            }
            _ => return Err("MTI not set"),
        };

        let bitmap_len;

        if self.bitmap as u64 > 0 {
            bitmap_len = 128;
            iso.extend_from_slice(format!("{:032X}", self.bitmap | 1 << (127)).as_bytes()); // Toggle field 1 for sec bitmap
        } else {
            bitmap_len = 64;
            iso.extend_from_slice(format!("{:016X}", (self.bitmap >> 64) as u64).as_bytes());
        };

        let iso = self.data_elements[0..=bitmap_len]
            .iter()
            .skip(2) // Skip mti and bitmap
            .fold(iso, |mut iso, data_element| match data_element {
                (Some(len), Some(data)) => {
                    iso.extend_from_slice(len.as_bytes());
                    iso.extend_from_slice(data);
                    iso
                }
                _ => iso,
            });

        Ok(iso)
    }

    fn validate_input_size(data: &[u8], data_size: &data::Size) -> Result<String, String> {
        match data_size {
            data::Size::FIXED(len) => {
                if data.len() != *len as usize {
                    return Err(format!(
                        "Expected fixed length of {}, Found -> {}",
                        len,
                        data.len()
                    ));
                }
                Ok(String::new())
            }
            data::Size::LL(max) => {
                let len = data.len();
                if len > *max as usize {
                    return Err(format!("Expected maximum length of {}, Found {}", max, len));
                }
                Ok(format!("{:02}", len))
            }
            data::Size::LLL(max) => {
                let len = data.len();
                if len > *max as usize {
                    return Err(format!("Expected maximum length of {}, Found {}", max, len));
                }
                Ok(format!("{:03}", len))
            }
            data::Size::LLLL(max) => {
                let len = data.len();
                if len > *max as usize {
                    return Err(format!("Expected maximum length of {}, Found {}", max, len));
                }
                Ok(format!("{:04}", len))
            }
            data::Size::BMP => Err("BITMAP is autogenerated and should not be set by crate user".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_bit_64_test() -> Result<(), String> {
        let spec = &crate::v1987::SPEC;

        let mut iso_packer = Packer::new(spec);

        iso_packer.set_field(0, "0800".as_bytes().into())?;
        iso_packer.toggle_bit_64(true);

        let iso = iso_packer.pack(Vec::with_capacity(1024))?;
        assert_eq!("08000000000000000001", std::str::from_utf8(&iso).unwrap());

        Ok(())
    }

    #[test]
    fn toggle_bit_128_test() -> Result<(), String> {
        let spec = &crate::v1987::SPEC;

        let mut iso_packer = Packer::new(spec);

        iso_packer.set_field(0, "0800".as_bytes().into())?;
        iso_packer.toggle_bit_128(true);

        let iso = iso_packer.pack(Vec::with_capacity(1024))?;

        assert_eq!("080080000000000000000000000000000001", std::str::from_utf8(&iso).unwrap());

        Ok(())
    }
}
