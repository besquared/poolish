### Thoughts 2021-12-22

Do we need to free pages at all? If our eviction system just randomly marks pages for eviction
as our program runs then I don't know that we need to do anything special in-particular. If those
pages get pulled back in that fine.

"When memory for a new page is needed, the least recently unswizzled page is used (after flushing it if it is dirty)."
- Leanstore

"Whenever a thread requests a new, empty page or swizzles a page, it will check if the percentage of cooling pages
is below a threshold and will unswizzle a page if necessary."
- Leanstore

"Umbra employs essentially the same replacement strategy as LeanStore, where we speculatively unpin pages but do
not immediately evict them from main memory. These cooling pages are then placed in a FIFO queue, and eventually
evicted if they reach the end of the queue."
- Umbra

This means there are basically two main rules:

(1) We must keep enough free space to allow at least one page of the largest size to be unswizzled (2gb)
(2) If we don't have enough space for the largest page (2gb) then randomly place pages into the fridge until we do
(3) If a new page is needed we first pull it from the fridge before we pull it from the free pool
