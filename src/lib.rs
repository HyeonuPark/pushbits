//! Push and pop the bit-packed fields.
//!
//! It's common for network protocols to have bit-packed fields
//! as a fixed width row in its header.
//! Each fields have their own bit width, and its bit offset is defined
//! as a sum of the widths of previous fields of the same row.
//!
//! Traditionaly to set/extract those field one should manage
//! both width and offset of the field.
//! But since all the fields of this row are handled,
//! the offset is a redaundant information we should not care ourselves.
//!
//! That's where the pushbits came from.
//! This crate provides fixed width bit container where you can
//! push and pop the bits as a integer using bitshift left operation.
//! If the widths are constant, the compiler optimize out all the overheads.

/// 32bits container where you can push and pop multiple bits as a integer.
///
/// See the crate level documentaion for more details.
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bits32 {
    bits: u32,
}

impl Bits32 {
    /// Bit width of this container.
    pub const BIT_WIDTH: u32 = 32;

    /// Create a new Bits32 container with the given bit pattern.
    #[inline]
    pub fn new(bits: u32) -> Self {
        Bits32 { bits }
    }

    /// Copy out the current bit pattern of this container.
    ///
    /// # Example
    ///
    /// ```
    /// # use pushbits::Bits32;
    /// let mut bits = Bits32::new(0);
    /// bits.push(5, 0b10110_u16);
    /// bits.push_bool(true);
    /// assert_eq!(0b10110_1, bits.get());
    /// ```
    #[inline]
    pub fn get(&self) -> u32 {
        self.bits
    }

    /// Push given number of bits into the LSB of this container
    /// using the bit shift left operation.
    ///
    /// # Panics
    ///
    /// It panics if the `num_bits` is greater than 31.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pushbits::Bits32;
    /// let mut bits = Bits32::new(0);
    /// bits.push(8, 0b11100110_u32);
    /// bits.push(5, 0b10001_u8);
    /// assert_eq!(0b11100110_10001, bits.get());
    /// ```
    #[inline]
    pub fn push<T: Into<u32>>(&mut self, num_bits: u32, value: T) {
        assert!(num_bits < Self::BIT_WIDTH);
        self.bits <<= num_bits;
        let mask = (1 << num_bits) - 1;
        let value = value.into() & mask;
        self.bits |= value;
    }

    /// Push a boolean as a single bit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pushbits::Bits32;
    /// let mut bits = Bits32::new(0);
    /// bits.push_bool(true);
    /// bits.push_bool(false);
    /// assert_eq!(0b10, bits.get());
    /// ```
    #[inline]
    pub fn push_bool(&mut self, value: bool) {
        self.push(1, value)
    }

    /// Pop given number of bits out from the MSB of this container
    /// using the bit shift left operation.
    ///
    /// # Panics
    ///
    /// It panics if the `num_bits` is greater than 31.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pushbits::Bits32;
    /// let mut bits = Bits32::new(0xDEADBEEF);
    /// assert_eq!(0xDEA, bits.pop(12));
    /// assert_eq!(0xDBE, bits.pop(12));
    /// assert_eq!(0xEF, bits.pop(8));
    /// ```
    #[inline]
    pub fn pop(&mut self, num_bits: u32) -> u32 {
        assert!(num_bits < Self::BIT_WIDTH);
        let res = self.bits >> (Self::BIT_WIDTH - num_bits);
        self.bits <<= num_bits;
        res
    }

    /// Pop a single bit out as a boolean.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pushbits::Bits32;
    /// let mut bits = Bits32::new(0b101);
    /// bits.push(Bits32::BIT_WIDTH - 3, 0_u32);
    /// assert_eq!(true, bits.pop_bool());
    /// assert_eq!(false, bits.pop_bool());
    /// assert_eq!(true, bits.pop_bool());
    /// ```
    #[inline]
    pub fn pop_bool(&mut self) -> bool {
        self.pop(1) != 0
    }
}
