## Leaf Page Storage

```
struct Page {
  swip: usize,
  vlds: usize, // vers, latch, dirty
}

// 512 bits of state information
// 1 bit per value validity
// 1 bit per value nullity

struct DataPage<T> : Page {
  next: usize, // pointer to next values pages
  past: usize, // pointer to the newest past version

  // Used by the GC + Optimizer
  
  rcnt: usize, // recent read counter
  wcnt: usize, // recent write counter  
  lchg: usize, // lower offset of recently changed values
  uchg: usize, // upper offset of the recently changed values
  
  // The Data Itself
   
  valid: Vec<u8>, // bitmap of offsets that aren't deleted
  nulls: Vec<u8>, // bitmap of offsets that are null
  values: Vec<T>, // data values
}

Example u8 values in 4kb page

4096b page
 512b head
 384b valid
 384b nulls
2816b values (352 u8 values)

General for 8 bit values

v = 8    // bits per value
b = 2    // bits for bitmaps
p = 4096 // bits per page
h = 512  // bits per page head

bits_per_bitmap = (1 + (p / (v + b)) / 64) * 64      // relies on integer division truncation
number_of_values = (p - (bits_per_bitmap * 2)) / v   // relies on integer division truncation
```