### Notes 2021-12-24

(1) Dataflow language interface: Malloy, Sub-operators as first class citizens (paper), Modularis (ideas), etc.
(2) Is there a way that we could basically used a shared-disk system at the page level? Could the buffer manager be three-tier?
(3) If we want to do this and do MVCC then we need some way to coordinate about txids either centrally or distributed.
(4) First we need to decide how it is that we want to provide snapshot isolation, this depends on how we store data.
(5) The state we need to share isn't just the txnid but also the B+Tree pages and their updates. We could just distribute
the B+Tree index itself or at least deltas to the nodes. Basically we have a WAL but only for the B+Tree.

A distributed B+Tree storage engine using raft.

(1) A shared view of pod membership
(2) A distributed transaction counter
(3) A distributed persistent page id counter
(4) A distributed write-ahead log that versions pages
    * Pages are immutable, when a page is updated a new page is allocated and written

For optimization different page types may have different storage classes:

(1) Index Storage Path (ex: PMEM)
(2) Local Page Storage Path (ex: NVMe)
(3) Shared Page Storage Path (ex: EFS)

Page Types

The way that our buffer manager ensures that there is only one handle per page then  

### Notes 2021-12-23

Supporting Updates

(1) Need a b-tree system like a normal database for information_schema
(2) For performance reasons probably need to make locking more fair somehow
(3) Need to figure out how to implement multi-version concurrency control (locking)
(4) Making this efficient probably requires us to maintain some kind of tree-based structure
(6) This requires an atomic commit algorithm and maybe some other machinery
(7) We could also use a single-threaded partitioned strategy like h-store/voltdb.
    (7a) This isn't great since basically every read touches every partition
(8) HyPer only uses one writer thread so that there are no W-W conflicts which makes sense.

What's the point of updates if we don't persist things?

It's useful when we're using the system as a read-through cache where an outside system is maintaining persistent state
and we want to reflect that as incremental changes to our cache. This allows our caches to remain up to date and still be
very fast.

### Notes 2021-12-22

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
