# Aio Database Release Notes

### v0.8.2
- Fix critical bug and add tests cover up
- Update dependencies

### v0.8.1
- Update dependencies
- Update to bevy 0.15
- Small bug fix in local_db creation

### v0.8.0
- Update dependencies
- *MAJOR CHANGE:*Change the sqlite provider from libsql to rusqlite
- Internal updates and refactoring
- Update to bevy 0.15
- Small bug fix in local_db creation

### v0.7.11
- Update dependencies
- Rollback multiple types in `INTEGER` type mapping

### v0.7.10
- Update dependencies

### v0.7.9
- Prepared WAL and WAL2 modes support through `set_wal_mode`
- Prepared DELETE mode support through `set_wal_mode_to_rollback` for backward compatibility and easy switch between WAL and WAL2 modes
- Prepared concurrent APIs for `insert_value_concurrent`, `update_value_concurrent`, `partial_update_concurrent`
- Update dependencies
- Added `create_index` for non-unique indexes API

### v0.7.8
- Drop completely `impl Send` for external and internal structs

### v0.7.8
- Drop completely `impl Send` for external and internal structs

### v0.7.7
- Drop `impl Send` for R2D2 connection builder

### v0.7.6
- Add `impl Send` for external and internal structs
- Update dependencies

### v0.7.5
- Hotfix for `create_unique_index`

### v0.7.4
- Update dependencies
- Add `create_unique_index` and `drop_index`

### v0.7.3
- Update dependencies

### v0.7.2
- Update dependencies

### v0.7.1
- Update dependencies

### v0.7.0
- Update dependencies

### v0.6.7
- Update dependencies
- Add API for changing the PRAGMA synchronous settings

### v0.6.6
- Reversed boolean changes, now it's save as 0 or 1 in the sqlite databases (in the form of NUMERIC)

### v0.6.5
- Additional fixes for sqlite and libsql mapping to rust types

### v0.6.4
- Extend fixes for sqlite and libsql mapping to rust types
- Dropped unsupported types (u128 and i128)

### v0.6.3
- Fixed Rust types mapping to Sqlite types

### v0.6.2
- Fixed an issue where Sqlite's NULL values causes panic because Default values are missing then doing mapping

### v0.6.1
- Fixed an issue with the query used for auto-migration

### v0.6.0
- Official Release
- Added retries in queries that might lock the database. **insert_value** **update_value** **partial_update**, **delete_value** now returns Result type. 
- Added **set_query_retries** to the AioDatabase struct which sets how many retries should be made.
- Improved reliability
- Fixed string escaping

### v0.5.10
- Improved reliability
- Update dependencies
- Initial beta release

### v0.5.9
- Added **any**, **count** and **all** queries
- Fix typos
- Update dependencies

### v0.5.8
- Added support for Vec<u8> type (BLOB type). This allows to save any data in the database, even files

### v0.5.7
- Internal: Increased reliability then creating a database table
- Added **create_remote** for testing purposes
- Internal: Database schema is from now and on boxed

### v0.5.6
- Hotfix: Expanded the same fix from v.5.5
- Preparations for release of v6.0

### v0.5.5
- Hotfix: If a query returns 1 result / row panics the application 

### v0.5.4
- Internal: Added connection pooling which greatly increases the concurrency possibilities, performance and reliability
- Internal: Changed journal_mode to WAL
- Internal: Added query that changes the default settings of the sqlite database

### v0.5.3
- **insert_value** is now accepting references
- Add **Contains**, **StartsWith** and **EndsWith** Operators
- Internal: Added test to guarantee that all apis work before release

### v0.5.2 
- Fix Documentation
- Fix Some Examples showing the same code for in-memory and local db
- Added **partial_update** for updating single field / column of a row

### v0.5.1
- Fix Re-Exports

### v0.5.0
- Alpha Release