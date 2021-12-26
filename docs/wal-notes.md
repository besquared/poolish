### The Write-Ahead Log

From "A Survey of B-Tree Logging and Recovery"

> A recovery log contains at least three kinds of information or types of log records:
>
> "redo" information needed to provide durability,
> "undo" information in case a transaction cannot complete,
> commit records to indicate that a transaction indeed completed.
>
> There might be additional types of log records, for example, information logged  when a new transaction starts or when
> a transaction switches from forward processing to roll back and compensation of its earlier actions. Some additional
> types of log records are discussed shortly in the context of specific techniques and optimizations.

So each of the members of our pod will be responsible for some subset of leaf pages. The state we share could be a big
logical page table for the database. But that doesn't really matter that much. Also each table is its own B+tree so how
does that work.

What does our WAL look like?

```
Change attribute value at page (2, 0) from 5 -> 10 (txn_id = 1)

redo: [0, 1, CPage, 2, 4] <-- Copy-on-write page that holds data
undo: [1, 1, DPage, 4]
redo: [4, 1, CVLnk, 2, 4] <- Create link in the version chain from 2 -> 4
undo: [4, 1, CVLnk, 2, 3] <- Set the link in the version chain back to 2 -> 1
```

How do we deal with a batch update request that does an insert from a select?
In that scenario it might be that every single node will need to add one or more pages to the tree.
If this does happen then we will potentially need to merge the two pages in order to reconcile the changes.

Let's say we have each leaf node responsible for 4 rows. And we have 4 leaf nodes, each with 4 values. If we have a 4
node cluster and they all receive an `INSERT AS ...` then all four will want to create new pages to hold the values that
they all include.

I guess the natural thing to do when we need to merge the pages is to simply stitch them together into a value chain for
that node. In that scenario each inner node of the tree may hold a variable number of values. This would sort of violate
one of our rules about how many things we put into each block. Our blocks would have variable size at that point. We could
always rely on the garbage collector to stitch them together at that point?

This would work only in one case, which is that somehow we could ensure that tuples got stitched together in the correct
order across all of the attributes. Since we are using a DSM storage model if we stitch the value chain of each attribute
together in the wrong order then our tree would be invalid. How should we stitch the value chain together in such a way
that it is always valid.
