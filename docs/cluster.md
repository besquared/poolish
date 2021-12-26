## Cluster Architecture

The shared disk shared-disk distributed database architecture

```
+----------------------+ +----------------------+
| Machine 1            | | Machine 2            |
|                      | |                      |
| CPU 0 |  CPU 1 | ... | | CPU 0 |  CPU 1 | ... |
| DDR 0 |  DDR 1 | ... | | DDR 0 |  DDR 1 | ... |
| PMEM / NVMe (LSP)    | | PMEM / NVMe (LSP)    |
|----------------------+ +----------------------|
|  ETH / INF Adapter   | |  ETH / INF Adapter   |
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
The B+Tree structure is an overlay onto a shared virtual memory system