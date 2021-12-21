use anyhow::{
  anyhow, Result
};

use crate::HEADER_LEN;

//
// const SIZE_CLASSES: [usize; 20] = [
//   2^12, 2^13, 2^14, 2^15, //   4k,   8k,  16k,  32k
//   2^16, 2^17, 2^18, 2^19, //  64k, 128k, 256k, 512k
//   2^20, 2^21, 2^22, 2^23, //   1m,   2m,   4m,   8m
//   2^24, 2^25, 2^26, 2^27, //  16m,  32m,  64m, 128m
//   2^28, 2^29, 2^30, 2^31  // 256m, 512m,   1g,   2g
// ];
//

pub const MIN_CLASS_ID: usize = 12usize;
pub const MAX_CLASS_ID: usize = 31usize;

pub fn size_of(cid: usize) -> usize {
  2usize.pow(cid as u32)
}

pub fn index_of(cid: usize) -> usize {
  cid - MIN_CLASS_ID
}

pub fn to_fit(data_len_in_bytes: u32) -> Result<usize> {
  let raw_cid = exp_to_fit(data_len_in_bytes)?;

  if raw_cid <= MIN_CLASS_ID {
    Ok(MIN_CLASS_ID) // Clamp to min value
  } else if raw_cid <= MAX_CLASS_ID {
    Ok(raw_cid)
  } else {
    Err(anyhow!("Page class id must be less than {}", MAX_CLASS_ID))
  }
}

fn exp_to_fit(data_len_in_bytes: u32) -> Result<usize> {
  let data_len = f64::from(data_len_in_bytes);
  let total_len = data_len + f64::from(HEADER_LEN as u8);

  if total_len <= f64::from(u32::MAX) {
    let log_b = 10f64;
    let log_len = total_len.log(log_b);
    Ok((log_len / 2f64.log(log_b)).ceil() as usize)
  } else {
    Err(anyhow!("Page class not found to accommodate {} header + data bytes", total_len.round()))
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_try_new_to_fit() -> Result<()> {
    // Min class up to 4096b - 18b
    assert_eq!(12, to_fit(0)?);
    assert_eq!(12, to_fit(2u32.pow(1) + 1)?);
    assert_eq!(12, to_fit(2u32.pow(2) + 1)?);
    assert_eq!(12, to_fit(2u32.pow(3) + 1)?);
    assert_eq!(12, to_fit(2u32.pow(4) + 1)?);
    assert_eq!(12, to_fit(2u32.pow(5) + 1)?);
    assert_eq!(12, to_fit(2u32.pow(6) + 1)?);
    assert_eq!(12, to_fit(2u32.pow(7) + 1)?);
    assert_eq!(12, to_fit(2u32.pow(8) + 1)?);
    assert_eq!(12, to_fit(2u32.pow(9) + 1)?);
    assert_eq!(12, to_fit(2u32.pow(10) + 1)?);
    assert_eq!(12, to_fit(2u32.pow(11) + 1)?);

    assert_eq!(13, to_fit(2u32.pow(12) + 1)?);
    assert_eq!(14, to_fit(2u32.pow(13) + 1)?);
    assert_eq!(15, to_fit(2u32.pow(14) + 1)?);
    assert_eq!(16, to_fit(2u32.pow(15) + 1)?);

    assert_eq!(17, to_fit(2u32.pow(16) + 1)?);
    assert_eq!(18, to_fit(2u32.pow(17) + 1)?);
    assert_eq!(19, to_fit(2u32.pow(18) + 1)?);
    assert_eq!(20, to_fit(2u32.pow(19) + 1)?);

    assert_eq!(21, to_fit(2u32.pow(20) + 1)?);
    assert_eq!(22, to_fit(2u32.pow(21) + 1)?);
    assert_eq!(23, to_fit(2u32.pow(22) + 1)?);
    assert_eq!(24, to_fit(2u32.pow(23) + 1)?);

    assert_eq!(25, to_fit(2u32.pow(24) + 1)?);
    assert_eq!(26, to_fit(2u32.pow(25) + 1)?);
    assert_eq!(27, to_fit(2u32.pow(26) + 1)?);
    assert_eq!(28, to_fit(2u32.pow(27) + 1)?);

    assert_eq!(29, to_fit(2u32.pow(28) + 1)?);
    assert_eq!(30, to_fit(2u32.pow(29) + 1)?);
    assert_eq!(31, to_fit(2u32.pow(30) + 1)?);

    if let Ok(class) = to_fit(2u32.pow(31)) {
      assert!(false, "page class {} unexpectedly found for bytes {}", class, 2u32.pow(31))
    }

    if let Ok(class) = to_fit(u32::MAX) {
      assert!(false, "page class {} unexpectedly found for bytes {}", class, u32::MAX)
    }

    Ok(())
  }
}