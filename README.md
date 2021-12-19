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