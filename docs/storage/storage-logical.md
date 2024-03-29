## General Thoughts

I want to think about a system where we keep all data values in the leaf nodes of the b+tree itself.
We could then implement an MVCC system where writes to a specific page cause a copy of that page instead.
These pages would be versioned and chained together N2O and the garbage collector would compact them at a transaction
level.

Pros:

1. Easier implementation of snapshot isolation w/o maintenance of version vectors
2. No version pointers to follow once the correct page is found which improves cache locality
3. In a shared-disk system distributing the system simplifies to distributing a log of tree changes
4. Our tuple data is organized in a block-based DSM format so writes to fewer attributes is more performant

Cons:

1. When a page needs to be split or merged a lot of data has to be copied
2. Big table merges may require very large portions of the table be copied
3. Random wide updates could be 100x worse w.r.t. memory copies in the worst case
4. Serialization validation could potentially become more expensive without "precision locking" 

How can we mitigate some of these cons?

1. We can have pages that are very small potentially even as small as 1024b for some data types

Could we change leaf page sizes on the fly? Could we find leaf nodes that are frequently updated
and split them into smaller pages so that updates would potentially only impact one page or the other?
It's more important really that updates are only happening to a small number of tuples in the page at once
and that splitting the page would result in a more advantageous read-pattern.

```
        R
      /   \
    I1     I2
    |      |  
    L1     L2_1_t3 -> L2_1_t2 -> L2_1_t1 // L2_1 Version Chain (N2O)
           |
           |                             // L2 Value Chain (Link 1)
           |
           L2_2_t3 -> L2_2_t3            // L2_2 Version Chain (N2O)
           |
           |                             // L2 Value Chain (Link 2)
           |
           L2_3_t3 -> L2_3_t1            // L2_3 Version Chain (N2O)
           ^ Trunk
```

In this setup leaf nodes can have pointers to two linked lists:

1. A pointer to the the chain of previous versions of ths page in N2O order
2. A pointer to the blocks value chain which, when taken together, make up all of the attribute values in this block

If we notice that a majority of the updates to a page come from a specific subset of the page then we can split the
leaf page into multiple smaller pages and add them into the leaf nodes value chain. Similarly if we notice that a
value page hasn't seen any updates in a while we can merge it with an adjacent value page. 

This can make operators a bit easier to implement in some ways since once we find a block of values that was committed
before we began then we can simply read the page as if it were.

### Tracking Changes

To implement this each page could keep an update synopsis. This could also help the garbage collector to merge the pages
into one another without copying s much memory. The idea of an update synopsis is that each version of each page can keep
a range that is the minimum and maximum offset of the values that were written in that page. This information can be used
by the garbage collector to merge the value chain and then used by it to split/merge the values chain. 

### Splitting Pages

When the garbage collector runs it also merges the synopses into the trunk as well. Once all of synopses in the trunk have
been merged we use them to determine whether and how the trunk needs to be split or merged. Our goal is to try and localize
updates into as few pages as possible while keeping pages as large as possible to allow for more efficient zero-copy scans.

The policy could be relatively simple:

If the merged synopses tell us that either the either half of a page is updated more frequently than the other then we 
should split the page in half, copying the first and second halves respectively, and insert both new nodes into the trunk,
preserving the order of the original attribute values.

### Merging Pages

When do we merge pages? There isn't an explicit action that might cause us to merge pages and they aren't doing that much
harm to the system generally so it might be ok to leave them. The garbage collector could come back through to "hot" access
paths (maintained by the buffer manager as an atomic counter on the page). If a page has a high read to write ratio then it's
probably a candidate for merging. Since the trunk pages also have these counters the GC can use that information to make
decisions as well. When the GC merges trunk pages it should reset the read and write counters back to zero in order to "clear
the page and allow new statistics to begin to collect.

We know the r/w access ratio of the root, the intermediate nodes, and all of the leaf nodes and their value chains.

In order to know what a good R/W ratio target would be we need to understand how much worse it is for the operators to
use block sizes less than the optimal size. By localizing writes s.t. pages hold significantly less than the amount of
data that fits into L1 cache for an operation we're leaving some performance on the table.
