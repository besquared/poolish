## Concurrency

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
