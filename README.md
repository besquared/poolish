Pages is a virtual memory manager

It is based on:

* [LeanStore: In-Memory Data Management Beyond Main Memory](https://db.in.tum.de/~leis/papers/leanstore.pdf)
* [Umbra: A Disk-Based System with In-Memory Performance](http://cidrdb.org/cidr2020/papers/p29-neumann-cidr20.pdf)

Its goal is to allow a program to work with a virtual memory size that is much larger than the physical memory available
to the machine. Like all buffer managers it does this by tracking page allocations and moving data between the main memory
system (DRAM) and the secondary storage system (Disk/SSD/NVMe) as necessary.

The main goals are:

1. Variable length pages
2. High hit rate efficiency
3. Support for hundreds of concurrent threads

The system uses several strategies, described in detail in the papers above, and summarized as:

1. A fixed set of page size classes each of which is 2^class bytes
2. Using the OS anonymous mmap functionality to allocate virtual memory
3. Allocating one full-size virtual memory pool for every page size class
4. Accessing pages using "swizzled" pointers instead of a shared page map
5. A "cooling" FIFO queue that blends randomization and second chance policies

Pages specifically makes several design choices and imposes several constraints

1. Requires at least 32-bit data pointers
2. Fixed page classes 12 to 31 (4kb to 2gb)
3. Total memory pool must be at least 2gb
4. Total memory pool must be divisible by 2gb

### Pointer Tags

In a computer, memory addresses are integers that reference a location in the computer's main memory system. Even though 
the main memory system is made up of a large collection of individual bits, on most modern systems, valid memory addresses
have a minimum granularity of one byte (8 bits). That means that each valid memory address references the location of one
byte of the main memory.

For example let's take a computer system that has 16 bits (2 bytes) of total memory:

```
0000 0000 0000 0000
^         ^
0         8 <-- Addresses
```

Even though there are 16 total bits and 16 corresponding addresses, the computer system only recognizes every 8th address
as being valid. When a program requests to read the memory at address `8` from the main memory then it will read not just
the bit stored at location `8` but also the bits at positions 9-15. We say that the memory on these systems is `byte addressable`.

Now that we know how memory is addressed let's talk about the addresses themselves. On a 64-bit system each memory address
is represented as a 64-bit integer. If we exclude the address `0`, which is an unusable, reserved address, then the smallest
valid memory address would be `8` followed by `16`, `24`, etc. up to the largest valid address which is 2^64 - 8.

A 64-bit machine uses 64-bit memory addresses. These addresses are integers which we can see represented in binary:

```
 8 = 0000 0000 0000 1000
16 = 0000 0000 0001 0000
24 = 0000 0000 0001 1000
...                  ^^^ <- 3 Least Significant Bits
```

The important thing to note here is that all addresses must be a multiple of 8 since they address the location of a byte.
This means that a valid memory address is always an integer where the least significant bits are `000` on a 64-bit system,
`00` on a 32-bit systems, and `0` on a 16-bit system.

In rust these addresses are represented using `pointers`. A pointer is a usize integer that can be de-referenced using the
operator `*` to obtain the value stored at that corresponding memory address. When using a pointers it's important to never
de-reference one whose least significant bits aren't zero, as that is never a valid memory address. However, when the pointer
is at rest we can treat it as we might any other integer and use those bits to store additional type information, so long as
we make sure to set them to zero before we attempt to de-reference the pointer and read from the main memory. This technique
is called `tagging`.

For example let's assume we have a pointer to a 32-bit integer value and want to tag the least significant bit:

```
let value: u32 = 10u32;

// Bitcast reference (pointer) to an integer
let value_ptr: usize = &value as usize;

// Set the least significant bit of the pointer to 1
let tagged_ptr: usize = value_ptr + 1;

// False since the the least significant bit is set to 0
let is_value_ptr_tagged: bool = value_ptr & 1usize == 1usize;

// True since the least significant bit is set to 1
let is_tagged_ptr_tagged: bool = tagged_ptr & 1usize == 1usize;

// Read original value
let value = *((tagged_ptr - 1) as *const u32);
```