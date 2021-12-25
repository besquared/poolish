### Notes 2021-12-24

We are going to try and do the MVCC strategy that HyPer uses.

This system has several features:

1. Lock-free reads and writes
2. Minimal data storage for undo buffer

The involves keeping an "undo buffer" for every
transaction that performs any writes. The undo buffer stores the values of each attribute that we change and
pointers to the previously valid value which may live either in the data itself or in another undo buffer.

Along with each block of data we store two additional pieces of information:

1. A vector of 64-bit integers, one per tuple, that store 0 if the value is current or a pointer to the latest version otherwise
2. A version synopsis, which stores the range of values contain at least one out of date value, meaning they have non-null version vectors

We need to maintain two system-wide counters to make this work:

1. A logical time counter which initializes at 0
2. A transaction id counter which initializes at 2^63

This is intended to make sure that transaction ids are always larger than any logical times

### Transactions with updates:

1. When the transaction enters the system it is assigned a transaction id and a started_at time
2. Since the transaction updates data, it is assigned a third committed_at timestamp upon commit

Before any attribute is updated its most recently updated value (greater than our started_at) is copied into the
transaction's undo buffer. This most recently updated value may be in the data itself or from a previous undo buffer.

While this transaction is running, we mark the newly created version with its transaction id which ensures that the
uncommitted version is only visible to the current transaction.

Upon commit, once the transaction is assigned a committed_at time, it marks its undo log's changes as being irrelevant
to transactions that begin from that point forward.

### Record Visibility

To access the visible version of a record attribute in a transaction we first read the in-place value of the attribute.

From there we follow undo buffer pointers and undo each of the value changes up until the version v s.t.:

v: the version in question
P: The predecessor version of v
TS: the timestamp associated with v

(1)           (2)           (3)
v.P = null || v.P.TS = T || v.P.TS < T.startTime

(1) There is no older version because it never existed or was (safely) garbage collected
(2) This version belongs to this transaction (a transaction can always read its own updates)
(3) All versions up to a transactions start time (We can always read up to our start time)

This is sufficient for serializable read-only transactions

### Serializability Validation

(1) We rely on optimistic reads in order to remain scalable and lock-free
(2) Therefore, at the end of every transaction we have to perform a validation step
(3) To avoid W-W conflicts we abort all transactions that try to update an uncommitted value
(5) This ensures that all version vectors point to a chain that leads to a value that is committed
(6) If a transaction updates a value multiple times we build a chain of those updates in its undo buffer

We have to ensure that all reads during our transaction could have been (logically) at the end of the transaction
without any observable change. Only updates that were committed during the transaction's lifetime, between it's
started_at and committed_at time are relevant for the validation step. Event that that modified, deleted, or created
values that intersect with the transactions read predicates will cause it to abort.

How it Works:

(1) Maintain a list of recently committed transactions with pointers to their undo buffers
(2) For each newly created version, check whether it satisfies any of the transaction's selection predicates
(3) For a deletion, we check whether or not the deleted object belonged to T ’s read set.
(4) For a modification (update) we have to check both the before image as well as the after image

If any of the above are true then we must abort the transaction

After successful validation the transaction is committed by first writing its commit into the redo log. After that, all
of the transaction's timestamps are changed to its newly assigned commit timestamp. 

### Examples and Questions

T1:

-- Given: attribute c has the value of c = 100 + b (not known ahead of time)
-- Updates all values of a where b > 100 (which also includes values c > 200 )

T2:

-- Reads all attributes of T for the records where c = 1024
-- The read is affected by T1 because it reads from the attribute a

-- Reads all values d of T for the records where c >= 200
-- This read is not affected by T1 because T1 only updated the attribute a

T1               | T2
-----------------+-------------------
                 | BEGIN
                 |
BEGIN            |
                 |
                 | UPDATE T
                 |    SET a = 1000
                 |  WHERE c = 1
                 |
                 | COMMIT
                 |
UPDATE T         |
   SET a = 1     |
 WHERE b >= 100  |
                 |
COMMIT???        |
-----------------+---------------------

If two transactions try and write to the same attribute we abort the second one. (First writer wins).
If a write transaction commits before another but after it started then we need to make sure that the second
transaction didn't update any tuples that the first read.

In the case of T1 above we read all tuples where b >= 100 due to the UPDATE predicate. We need t