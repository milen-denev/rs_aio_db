# Aio Database Release Notes

### v5.8
- Added support for Vec<u8> type (BLOB type). This allows to save any data in the database, even files

### v5.7
- Internal: Increased reliability then creating a database table
- Added **create_remote** for testing purposes
- Internal: Database schema is from now and on boxed

### v5.6
- Hotfix: Expanded the same fix from v.5.5
- Preparations for release of v6.0

### v5.5
- Hotfix: If a query returns 1 result / row panics the application 

### v5.4
- Internal: Added connection pooling which greatly increases the concurrency possibilities, performance and reliability
- Internal: Changed journal_mode to WAL
- Internal: Added query that changes the default settings of the sqlite database

### v5.3
- **insert_value** is now accepting references
- Add **Contains**, **StartsWith** and **EndsWith** Operators
- Internal: Added test to guarantee that all apis work before release

### v5.2 
- Fix Documentation
- Fix Some Examples showing the same code for in-memory and local db
- Added **partial_update** for updating single field / column of a row

### v5.1
- Fix Re-Exports

### v5.0
- Alpha Release