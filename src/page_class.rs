use anyhow::{
  anyhow, Result
};

use crate::HEADER_LEN;

#[derive(Clone, Debug)]
pub struct PageClass(u8);


//
// const SIZE_CLASSES: [usize; 20] = [
//   2^12, 2^13, 2^14, 2^15, //   4k,   8k,  16k,  32k
//   2^16, 2^17, 2^18, 2^19, //  64k, 128k, 256k, 512k
//   2^20, 2^21, 2^22, 2^23, //   1m,   2m,   4m,   8m
//   2^24, 2^25, 2^26, 2^27, //  16m,  32m,  64m, 128m
//   2^28, 2^29, 2^30, 2^31  // 256m, 512m,   1g,   2g
// ];
//

pub const MIN_CLASS: u8 = 12u8;
pub const MAX_CLASS: u8 = 31u8;

impl PageClass {
  pub fn id(&self) -> u8 {
    self.0
  }

  pub fn size(&self) -> u32 {
    2u32.pow(self.id() as u32)
  }

  pub fn index(&self) -> usize {
    (self.id() - MIN_CLASS) as usize
  }

  pub fn try_new(class: u8) -> Result<Self> {
    if class <= MIN_CLASS {
      Ok(Self(MIN_CLASS)) // Clamp to min value
    } else if class <= MAX_CLASS {
      Ok(Self(class))
    } else {
      Err(anyhow!("Page size class must be less than {}", MAX_CLASS))
    }
  }

  pub fn try_new_to_fit(data_len_in_bytes: u32) -> Result<Self> {
    let data_len = f64::from(data_len_in_bytes);
    let total_len = data_len + f64::from(HEADER_LEN);

    if total_len <= f64::from(u32::MAX) {
      let log_b = 10f64;
      let log_len = total_len.log(log_b);
      Self::try_new((log_len / 2f64.log(log_b)).ceil() as u8)
    } else {
      Err(anyhow!("Page class not found to fit {} header + data bytes", total_len.round()))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_try_new_to_fit() -> Result<()> {
    // Min class up to 4096b - 18b
    assert_eq!(12, PageClass::try_new_to_fit(0)?.id());
    assert_eq!(12, PageClass::try_new_to_fit(2u32.pow(1) + 1)?.id());
    assert_eq!(12, PageClass::try_new_to_fit(2u32.pow(2) + 1)?.id());
    assert_eq!(12, PageClass::try_new_to_fit(2u32.pow(3) + 1)?.id());
    assert_eq!(12, PageClass::try_new_to_fit(2u32.pow(4) + 1)?.id());
    assert_eq!(12, PageClass::try_new_to_fit(2u32.pow(5) + 1)?.id());
    assert_eq!(12, PageClass::try_new_to_fit(2u32.pow(6) + 1)?.id());
    assert_eq!(12, PageClass::try_new_to_fit(2u32.pow(7) + 1)?.id());
    assert_eq!(12, PageClass::try_new_to_fit(2u32.pow(8) + 1)?.id());
    assert_eq!(12, PageClass::try_new_to_fit(2u32.pow(9) + 1)?.id());
    assert_eq!(12, PageClass::try_new_to_fit(2u32.pow(10) + 1)?.id());
    assert_eq!(12, PageClass::try_new_to_fit(2u32.pow(11) + 1)?.id());

    assert_eq!(13, PageClass::try_new_to_fit(2u32.pow(12) + 1)?.id());
    assert_eq!(14, PageClass::try_new_to_fit(2u32.pow(13) + 1)?.id());
    assert_eq!(15, PageClass::try_new_to_fit(2u32.pow(14) + 1)?.id());
    assert_eq!(16, PageClass::try_new_to_fit(2u32.pow(15) + 1)?.id());

    assert_eq!(17, PageClass::try_new_to_fit(2u32.pow(16) + 1)?.id());
    assert_eq!(18, PageClass::try_new_to_fit(2u32.pow(17) + 1)?.id());
    assert_eq!(19, PageClass::try_new_to_fit(2u32.pow(18) + 1)?.id());
    assert_eq!(20, PageClass::try_new_to_fit(2u32.pow(19) + 1)?.id());

    assert_eq!(21, PageClass::try_new_to_fit(2u32.pow(20) + 1)?.id());
    assert_eq!(22, PageClass::try_new_to_fit(2u32.pow(21) + 1)?.id());
    assert_eq!(23, PageClass::try_new_to_fit(2u32.pow(22) + 1)?.id());
    assert_eq!(24, PageClass::try_new_to_fit(2u32.pow(23) + 1)?.id());

    assert_eq!(25, PageClass::try_new_to_fit(2u32.pow(24) + 1)?.id());
    assert_eq!(26, PageClass::try_new_to_fit(2u32.pow(25) + 1)?.id());
    assert_eq!(27, PageClass::try_new_to_fit(2u32.pow(26) + 1)?.id());
    assert_eq!(28, PageClass::try_new_to_fit(2u32.pow(27) + 1)?.id());

    assert_eq!(29, PageClass::try_new_to_fit(2u32.pow(28) + 1)?.id());
    assert_eq!(30, PageClass::try_new_to_fit(2u32.pow(29) + 1)?.id());
    assert_eq!(31, PageClass::try_new_to_fit(2u32.pow(30) + 1)?.id());

    if let Ok(class) = PageClass::try_new_to_fit(2u32.pow(31)) {
      assert!(false, "page class {} unexpectedly found for bytes {}", class.id(), 2u32.pow(31))
    }

    if let Ok(class) = PageClass::try_new_to_fit(u32::MAX) {
      assert!(false, "page class {} unexpectedly found for bytes {}", class.id(), u32::MAX)
    }

    Ok(())
  }
}