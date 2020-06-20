//! # Parse iso8583 messages.

use super::data;

#[allow(dead_code)]
const BIT_OFFSET_1: u8 = 1 << 3;
#[allow(dead_code)]
const BIT_OFFSET_2: u8 = 1 << 2;
#[allow(dead_code)]
const BIT_OFFSET_3: u8 = 1 << 1;
#[allow(dead_code)]
const BIT_OFFSET_4: u8 = 1 << 0;

#[allow(dead_code)]
fn nibble_to_number(c: u8) -> Result<u8, &'static str> {
    match c as char {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        'a' | 'A' => Ok(10),
        'b' | 'B' => Ok(11),
        'c' | 'C' => Ok(12),
        'd' | 'D' => Ok(13),
        'e' | 'E' => Ok(14),
        'f' | 'F' => Ok(15),
        _ => Err("Invalid hex character"),
    }
}

/// i8583::Unpacker type and associated functions for creating iso8583 messages.
/// 
///
/// # Examples
///
/// ```
/// use i8583::Unpacker;
///
/// let response = "080022380000008000009A0000070814102414102414102407082020XXXX";
/// let mut iso = Unpacker::new(response.as_bytes());
/// let fields = iso.unpack(&i8583::nibss::SPEC)
///                 .unwrap_or_else(|err_msg| panic!("{}", err_msg));
/// ```
pub struct Unpacker<'a> {
    message: &'a [u8],
    data_elements: [Option<&'a [u8]>; 129],
}

impl<'a> Unpacker<'a> {
    /// Create a new unpacker for the given iso message.
    /// The lifetime of the unpacker will be tied to that of the given message
    pub fn new(msg: &'a [u8]) -> Unpacker {
        Unpacker {
            message: msg,
            data_elements: [None; 129],
        }
    }

    /// Unpack initialized message in [new](#method.new) into array of slices using provided spec.
    pub fn unpack(&mut self, spec: &[Option<data::Spec>; 129]) -> Result<&[Option<&'a [u8]>], String> {
        let mti_len = 4;
        let mut bitmap_capacity: [u8; 32] = [0; 32];

        let (bitmap, bmp_slice_len, no_of_elements) = self.hex_bitmap(&mut bitmap_capacity)?;
        let data = &self.message[mti_len + bmp_slice_len..];

        self.data_elements[0] = Some(&self.message[..mti_len]); //  Set MTI
        self.data_elements[1] = Some(&self.message[mti_len..(mti_len + bmp_slice_len)]); //  Set BITMAP

        let mut i = 1;
        let mut str_index: usize = 0;
        for val in bitmap.iter() {
            for bit in [BIT_OFFSET_1, BIT_OFFSET_2, BIT_OFFSET_3, BIT_OFFSET_4].iter() {
                if val & bit != 0 {
                    if let Some(field) = &spec[i] {
                        match field.data_size {
                            data::Size::FIXED(n) => {
                                str_index =
                                    fixed_len(&mut self.data_elements, data, i, str_index, n)?
                            }
                            data::Size::LL(max) => {
                                str_index =
                                    var_len(&mut self.data_elements, data, max, i, str_index, 2)?
                            }
                            data::Size::LLL(max) => {
                                str_index =
                                    var_len(&mut self.data_elements, data, max, i, str_index, 3)?
                            }
                            data::Size::LLLL(max) => {
                                str_index =
                                    var_len(&mut self.data_elements, data, max, i, str_index, 4)?
                            }
                            data::Size::BMP => {}
                        }
                    }
                }
                i = i + 1;
            }
        }

        Ok(&self.data_elements[..no_of_elements])
    }

    fn hex_bitmap(&self, bitmap: &'a mut [u8; 32]) -> Result<(&'a [u8], usize, usize), String> {
        let mti_len = 4;
        let msg_len = self.message.len();
        let (primary_bmp_size, sec_bmp_size) = (16, 32);

        if msg_len < (mti_len + primary_bmp_size) {
            return Err(format!("Invalid iso message with length: {}", msg_len));
        }
        // The above condition is a guard to prevent unwrap from panicking below
        let sec_bitmap_present = hex_secondary_bitmap(self.message[mti_len])?;

        if sec_bitmap_present && msg_len < (mti_len + sec_bmp_size) {
            return Err(format!(
                "Iso len({}) is less than expected for message with secondary bitmap",
                msg_len
            ));
        }

        let bitmap_hex = if sec_bitmap_present {
            &self.message[mti_len..(mti_len + sec_bmp_size)]
        } else {
            &self.message[mti_len..(mti_len + primary_bmp_size)]
        };

        for (i, val) in bitmap_hex.iter().enumerate() {
            bitmap[i] = nibble_to_number(*val)?;
        }

        let bitmap_and_elements = if sec_bitmap_present {
            (&bitmap[..], sec_bmp_size, 129)
        } else {
            (&bitmap[..primary_bmp_size], primary_bmp_size, 65)
        };

        Ok(bitmap_and_elements)
    }
}

fn hex_secondary_bitmap(bitmap_index_zero: u8) -> Result<bool, &'static str> {
    let val = nibble_to_number(bitmap_index_zero)?;
    Ok((val & BIT_OFFSET_1) != 0)
}

fn fixed_len<'a>(
    data_elements: &mut [Option<&'a [u8]>; 129],
    data: &'a [u8],
    i: usize,
    str_index: usize,
    n: u16,
) -> Result<usize, String> {
    let n = n as usize;
    if str_index + n > data.len() {
        return Err(format!(
            "Field {} with fixed length {} exceeds message bounds",
            i, n
        ));
    }

    data_elements[i] = Some(&data[str_index..str_index + n]);
    Ok(str_index + n)
}

fn var_len<'a>(
    data_elements: &mut [Option<&'a [u8]>; 129],
    data: &'a [u8],
    max: u16,
    i: usize,
    str_index: usize,
    n: usize,
) -> Result<usize, String> {
    let max = max as usize;
    let mut str_index = str_index;

    let len: usize = unsafe {
        // Contrary to the above, this block is memory safe.
        // The safety check is only being deferred to the parse method
        match std::str::from_utf8_unchecked(&data[str_index..str_index + n]).parse() {
            Ok(len) => len,
            Err(msg) => return Err(format!("{} : Could not convert field({}) len", msg, i)),
        }
    };

    if len > max {
        return Err(format!(
            "Field {} length {} exceeds specified maximum of {}",
            i, len, max
        ));
    }
    str_index += n;
    if str_index + len > data.len() {
        return Err(format!(
            "Field {} with fixed length {} exceeds message bounds",
            i, len
        ));
    }

    data_elements[i] = Some(&data[str_index..str_index + len]);
    Ok(str_index + len)
}
