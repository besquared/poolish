## Concurrency in a Shared-Disk Distributed System

```
+----------------------+ +----------------------+
| Machine 1            | | Machine 2            |
|                      | |                      |
| CPU 0 |  CPU 1 | ... | | CPU 0 |  CPU 1 | ... |
| DDR 0 |  DDR 1 | ... | | DDR 0 |  DDR 1 | ... |
| PMEM / NVMe (LSP)    | | PMEM / NVMe (LSP)    |
|----------------------+ +----------------------|
| NIC / INFB           | | NIC / INFB           |
+----------------------+-+----------------------+
|                    Network                    |
+-----------------------------------------------+
|            Shared Storage Pool (SSP)          |
+-----------------------------------------------+
```

Each machine is called a node
A collection of nodes is called a pod
A database is stored as a directory on the SSP 
A database can only be accessed by one pod at a time
Each table in the database is represented as a B+Tree
The B+Tree structure is an overlay onto a virtual memory system

Any time a transaction inserts, removes, or modified pages:

1. Those pages must be flushed to the shared storage pool
2. A log of those changes must be sent to every other node in the pod
3. When the transaction attempts to COMMIT each node in the pod must COMMIT

We can keep this relatively orderly by only allowing the leader to coordinate transactions that modify the database.

### Issues

We're going to have to reconcile pages to make this work. Pages may not be consistent across nodes otherwise.

This could happen because each node in a pod may update the SAME page within the SAME transaction. The only way
to solve this is for the data to be partitioned perfectly or to merge conflicting pages upon commit. If we have to
merge conflicting pages upon commit then we'll have to shuffle those pages around to do so.

How exactly do we partition up the tree between the workers?
Can we do a consistent hashing of some sort?
At the beginning of the transaction we can hash the leaf nodes into a ring containing the workers.

When an insert query in the transaction happens and new pages are inserted then those insertions will be shared to all of
the members of the pod who from that point forward will 

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

### Partitioned Queries

Here's another problem, in a shared disk system, how do we make sure that each node.
There are a certain number of leaf pages in the system when the transaction begins, we partition those.
Any pages that are written during the transaction are considered local to the current node until it commits.

Even read-only transactions require a BEGIN consensus in the log which ensures that we cannot accept
any transactions unless we are able to talk to a leader.
