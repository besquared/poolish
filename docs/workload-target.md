## Motivation

We want to design a system that allows the user to query tables that are 4 kilobytes up to about 1 terabyte.

### OLAP Query

This is the main purpose of the data system
We would like to process hundreds of these per second

```
SELECT a, b, c, SUM(d)
  FROM T
 WHERE a IN ('av1', 'av2', ...)
HAVING NOT ISNULL(SUM(d))
```

### Head Query

This is the secondary purpose of the data system
We would like to process hundreds of these per second as well

```
SELECT *
  FROM T
 WHERE LIMIT 100
```

### Pagination Query

This is the secondary purpose of the data system
We would like to process hundreds of these per second as well

```
SELECT *
  FROM T
 WHERE a BETWEEN 1000 AND 2000
```

### Bulk Merge Updates

This is the merge-based strategy for bulk updating from a primary source.
We expect that each table might be updated this way at a cadence from 1m - 5m.

For a fast moving table an update usually touches < 1% of the attribute values and adds << 1% of the tuple size.

```
BEGIN

CREATE TABLE T_TMP
    AS SELECT * FROM T2 WHERE TRUE = FALSE

INSERT INTO T_TMP
VALUES ((...), ...)

COMMIT

BEGIN

-- Update Tuples

UPDATE T2
   SET t2.a = t_tmp.a
       t2.b = t_tmp.b
       ...
  FROM T2 t2
  JOIN T_TMP t_tmp on t2.id = t_tmp.id
   AND t2.a <> t_tmp.a
   AND t2.b <> t_tmp.b
       ...

-- Add New Tuples

INSERT INTO T2 (a, b, c, ...)
SELECT a, b, c, ...
  FROM T_TMP
 WHERE T2.id NOT IN (SELECT DISTINCT id from T_TMP)

DROP TABLE T_TMP

COMMIT
```

### Bulk Replace Updates

This is the replace-based strategy for bulk updating from a primary source.
We expect that each table might be updated this way at a cadence from 1m - 5m.

```
BEGIN

CREATE TABLE T_TMP
    AS SELECT * FROM T2 WHERE TRUE = FALSE

INSERT INTO T_TMP
VALUES VALUES ((...), ...)

COMMIT

BEGIN

RENAME TABLE T2 T_TMP1;
RENAME TABLE T_TMP T2;

DROP TABLE T_TMP;
DROP TABLE T_TMP2;

COMMIT
```

### Direct Updates

This is the general strategy for making OLTP style updates.
There are multiple ways in which this may or may not be bad.

```
BEGIN

UPDATE T2
   SET a = v1,
       b = v2, 
       ...
 WHERE c IN (...)
   AND NOT a = v1
   AND NOT b = v2
       ...
 
COMMIT
```
