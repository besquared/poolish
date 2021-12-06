use anyhow::Result;

use memmap2::{
  MmapMut,
  MmapOptions
};

const SIZE_CLASSES: [usize; 20] = [
  2^13, 2^14, 2^15, 2^16,  //   8k,  16k,  32k,  64k
  2^17, 2^18, 2^19, 2^20,  // 128k, 256k, 512k,   1m
  2^21, 2^22, 2^23, 2^24,  //   2m,   4m,   8m,  16m
  2^25, 2^26, 2^27, 2^28,  //  32m,  64m, 128m, 256m
  2^29, 2^30, 2^31, 2^32   // 512m,   1g,    2g,  4g
];

pub struct BufferManager {
  frames: [MmapMut; 20]
}

impl BufferManager {
  pub fn try_new() -> Result<Self> {
    let frames: [MmapMut; 20] = [
      MmapMut::map_anon(2^13)?,
      MmapMut::map_anon(2^14)?,
      MmapMut::map_anon(2^15)?,
      MmapMut::map_anon(2^16)?,
      MmapMut::map_anon(2^17)?,
      MmapMut::map_anon(2^18)?,
      MmapMut::map_anon(2^19)?,
      MmapMut::map_anon(2^20)?,
      MmapMut::map_anon(2^21)?,
      MmapMut::map_anon(2^22)?,
      MmapMut::map_anon(2^23)?,
      MmapMut::map_anon(2^24)?,
      MmapMut::map_anon(2^25)?,
      MmapMut::map_anon(2^26)?,
      MmapMut::map_anon(2^27)?,
      MmapMut::map_anon(2^28)?,
      MmapMut::map_anon(2^29)?,
      MmapMut::map_anon(2^30)?,
      MmapMut::map_anon(2^31)?,
      MmapMut::map_anon(2^32)?
    ];

    Ok(Self { frames })
  }
}