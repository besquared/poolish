## Overview

Vex provides ad-hoc on-line analytical processing for interactive multi-tenant applications

ad-hoc - The processing to be done is mostly or completely unknown ahead of time
on-line - The end-user or application remains connected to the system while execution occurs
analytical - Each processing request usually involves accessing most of or even the entire database
processing - Processing involves transforming data using filters, aggregation, joins, and other operations
interactive - The end-user or application expects that their requests will complete in a matter of seconds
multi-tenant - The database contains data from multiple end-users and all requests share the same system resources
applications - End-user, backend, operations, and internally facing analytics applications

## What *is* vex?

Vex is a distributed, shared-disk, "larger than memory" database.

(1) distributed - vex runs on a cluster of nodes connected to one another on a network
(2) shared-disk - Every node in a cluster has access to a shared storage pool that it can use to persist data
(3) "larger than memory" - vex is designed to operate as an in-memory database but also uses persistent storage to
     allow for databases that don't fit entirely into memory
(4) database - vex looks like a typical database and has features including a relational model, transactions, and
    support for standard SQL

## When should vex be used?

(1) When you're building an analytics application that should respond to requests in less than 10 seconds
(2) If your application needs to perform analytical processing on databases that range from kilobytes to terabytes

## What should vex not be used for?

(1) Vex is intended to be a secondary store that operates alongside, rather than replaces, your primary cloud database