## System Design

### Model

The model of our system is a shared forest of B+Trees

### Clustering

(1) Distributed shared-disk architecture
(2) Membership is managed through the raft consensus protocol
(3) The number of cluster members and workers is fixed at cluster initialization

### Physical Data Model

(1) Each table in the system is stored in a B+Tree
(2) Each member of the cluster has the same shared view of the tree
(3) Tuples are stored in a DSM layout directly into the leaves of the tree

### Logical Data Model

(1) We use an extended relational algebra model
(2) The logical objects are tables, attributes, and operators
(3) We adopt the three-state logic system of the relational algebra
(4) Tuples are logically partitioned by a user provided unique unsigned integer key

### Storage

(1) Pages are variable length
(2) Local pages are persisted in local page storage as needed
(3) Shared pages are persisted in shared page storage when committed

### Availability

(1) A cluster will tolerate (N-1) / 2 member failures

### Transitions

(1) Logical partitions are assigned to workers using a consistent hashing policy
(2) When a member joins the cluster it becomes responsible for a subset of partitions
(3) When a member leaves the cluster all uncommitted transactions are forcefully aborted
(4) When a member leaves the cluster its partitions are re-assigned to the remaining workers

### Atomicity

(1) Atomicity is provided for by the consistency mechanisms

### Consistency

(1) We use multi-version concurrency control
(1) Pages are versioned using copy-on-write (CoW)
(2) All past page versions are maintained until they become unused
(3) All writes are performed with an optimistic "write then validate" policy
(4) Consensus is reached by using raft to distribute a transaction event log (BEGIN, VALIDATE, COMMIT)
(5) Write-write conflicts are avoided using a "first writer wins" policy at (page, offset) granularity
(6) To improve CoW for hot write paths the garbage collector may split record pages into multiple smaller pages
(7) To improve locality for hot read paths the garbage collector may merge multiple record pages into a single larger page

### Isolation

(1) The system will offer snapshot isolation using the versioned pages in the trees

### Durability

(1) Before any commit succeeds all modified pages will be flushed to shared page storage

### Recovery

(1) If we do end up needing recovery we will be using ARIES
(2) We will probably end up with a local write-ahead logs on each worker for rollback

### Execution

(1) The execution strategy is vectorized, push-based, and partially compiled
(2) Physical operators are executed using cooperative concurrent dataflow model (Timely)
(3) Logical operator pipelines are "fused" into physical operators which are compiled "just-in-time"
(4) Operators will have a modular design so that we can match their implementation with their environments

### Data Type System

(1) Each attribute in the system has a static type
(2) Every operator in the system has statically typed inputs and outputs

### Programming Model

(1) The logical unit of execution is a transaction
(2) The physical unit of execution is the statement
(3) Each statement is constructed using low-level operators
(4) Using these low-level operators higher level APIs such as SQL can be constructed
