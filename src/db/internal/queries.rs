use std::time::Duration;

use bevy_reflect::Struct;
use hex::encode;

use log::{error, trace};

use rusqlite::Connection;

use crate::db::{aio_query::{QueryBuilder, QueryRowResult, QueryRowsResult}, internal::helpers::{get_values_from_generic, query_match_operators}, models::Schema, WalMode};
use super::{helpers::{set_values_from_many_rows_result, set_values_from_row_result}, schema_gen::{generate_db_schema_query, get_current_schema, get_sql_type}};

static SLEEP_DURATION: Duration = Duration::from_millis(10); //Retry every 10ms

pub(crate) async fn create_table(schema_vec: &Vec<Schema>, name: &str, connection: &Connection) {
     let create_table_query = generate_db_schema_query(schema_vec, name);
     _ = connection.execute(&create_table_query, ());
}

pub(crate) async fn change_db_settings(connection: &Connection) {
     _ = connection.pragma_update(None, "journal_size_limit", "-1");

     _ = connection.pragma_update(None, "synchronous", "NORMAL");

     _ = connection.pragma_update(None, "auto_vacuum", "FULL");

     _ = connection.pragma_update(None, "temp_store", "MEMORY");

     _ = connection.pragma_update(None, "journal_mode", "WAL");
}

pub(crate) async fn set_wal_mode(connection: &Connection, wal_mode: WalMode) -> Result<(), String> {
     let query = format!("PRAGMA journal_mode={};", wal_mode.to_string());
     trace!("Executing PRAGMA query: {}", query);
     let result = connection.pragma_update(None, "journal_mode", "WAL");
     
     if let Ok(_) = result {
          Ok(())
     } else {
          Err(result.unwrap_err().to_string())
     }
}

pub(crate) async fn set_wal_mode_to_rollback(connection: &Connection) {
     _ = connection.pragma_update(None, "journal_mode", "DELETE");
}

pub(crate) async fn change_synchronous_settings(connection: &Connection, val: bool) {
     if !val {
          _ = connection.pragma_update(None, "synchronous", "OFF");
     }
     else {
          _ = connection.pragma_update(None, "synchronous", "NORMAL");
     }
}

pub(crate) async fn get_current_db_schema(name: &str, connection: &Connection) -> Option<Vec<Schema>> {  
     let query = format!("SELECT sql FROM sqlite_schema WHERE name = '{}'", name);
     trace!("Executing insert query: {}", query);
     
     let statement = connection.prepare(&query);

     if let Ok(mut result_row) = statement {
          let _ = result_row.execute(());

          let mut rows_result = result_row.raw_query();

          if let Ok(rows) = rows_result.next() {
               if let Some(rows) = rows {
                    let schema_row = rows.get::<_, String>(0).unwrap();
                    let current_schema = get_current_schema(schema_row);
                    return Some(current_schema);
               } else {
                    return None;
               }
          } else {
               return None;
          }
     }
     else {
          return None;
     }
}

pub(crate) async fn alter_table_new_column(name: &str, schema: &Schema, connection: &Connection) {
     let sql_type = get_sql_type(schema.field_type.as_str()).unwrap();
     let column_name = schema.field_name.as_str();

     let query = format!("ALTER TABLE {name} ADD COLUMN {column_name} {sql_type}");
     trace!("Executing insert query: {}", query);
     _ = connection.execute(&query, ());
}

pub(crate) async fn alter_table_drop_column(name: &str, column_name: &str, connection: &Connection) {
     let query = format!("ALTER TABLE {name} DROP COLUMN {column_name}");
     trace!("Executing insert query: {}", query);
     _ = connection.execute(&query, ());
}

pub(crate) async fn insert_value<T:  Default + Struct + Clone>(
     value: &T, 
     table_name: &str, 
     connection: &Connection,
     time_to_retry: u32, 
     concurrent: bool) -> 
     Result<(), ()>
{
     let generic_values = get_values_from_generic::<T>(value);
     let mut query = {
          if concurrent {
               format!("BEGIN CONCURRENT; INSERT INTO {} (", table_name)
          } else {
               format!("INSERT INTO {} (", table_name)
          }
     };

     for generic_value in generic_values.iter().take(generic_values.len() - 1) {
          query.push_str(generic_value.field_name.as_str());
          query.push(',');
     }

     query.push_str(generic_values.iter().last().unwrap().field_name.as_str());
     query.push(')');

     query.push_str(" VALUES (");

     for generic_value in generic_values.iter().take(generic_values.len() - 1) {
          if generic_value.field_type == "Vec" {
               let vec_u8 = generic_value.field_value.as_any().downcast_ref::<Vec<u8>>().unwrap();
               let hex = encode(vec_u8);
               query.push_str(format!("x'{}'", hex).as_str());
               query.push(',');
          }
          else {
               let mut string_value = format!("{:?}", generic_value.field_value);

               if generic_value.field_type == "String" {
                    string_value.pop();
                    string_value.remove(0);

                    if string_value.contains("'") {
                         string_value = string_value.replace("'", "''");
                    }

                    string_value = format!("'{}'", string_value);
               }

               if generic_value.field_type == "bool" {
                    if string_value.contains("false") {
                         _ = string_value = 0.to_string();
                    }
                    else {
                         _ = string_value = 1.to_string();
                    }  
               }

               query.push_str(string_value.as_str());
               query.push(',');
          }
     }

     let last = generic_values.last().unwrap();

     if last.field_type == "Vec" {
          let vec_u8 = last.field_value.as_any().downcast_ref::<Vec<u8>>().unwrap();
          let hex = encode(vec_u8);
          query.push_str(format!("x'{}'", hex).as_str());
          query.push(')');
     }
     else {
          let mut string_value = format!("{:?}", last.field_value);

          if last.field_type == "String" {
               string_value.pop();
               string_value.remove(0);

               if string_value.contains("'") {
                    string_value = string_value.replace("'", "''");
               }

               string_value = format!("'{}'", string_value);
          }

          if last.field_type == "bool" {
               if string_value.contains("false") {
                    _ = string_value = 0.to_string();
               }
               else {
                    _ = string_value = 1.to_string();
               }  
          }

          query.push_str(string_value.as_str());
          query.push(')');
     }

     if concurrent {
          query.push_str("; ");
          query.push_str("COMMIT;");
     }

     trace!("Executing insert query: {}", query);

     let mut retries = 0;

     while retries < time_to_retry {
         let function_result = connection.execute(&query, ());
 
         if function_result.is_ok() {
             return Ok(());
         }
         else {
             let error = function_result.unwrap_err();
             error!("Error occurred on {} retry. Message: {:?}", retries + 1, error);
             retries = retries + 1;
         }
         tokio::time::sleep(SLEEP_DURATION).await;
     }
 
     return Err(());
}

pub(crate) fn generate_get_query<'a, T:  Default + Struct + Clone>(query_builder: &'a QueryBuilder<'_>) -> String {    
     let options = &query_builder.query_options;
     let table_name = &query_builder.table_name;
     let mut query = format!("SELECT * FROM {table_name}");

     let schema = query_builder.db.get_schema();

     let len = options.len();

     if len > 1 {
          query.push_str(" WHERE ");
          for option in options.iter().take(options.iter().len() - 1) {
               let current = schema.iter().find(|x| x.field_name == option.field_name).unwrap();
               let next = option.next.as_ref().unwrap();
               let operator = option.operator.as_ref().unwrap();
               query_match_operators(operator,  &mut query, &option.field_name, &current.field_type, false, Some(next));
          }
     }

     if len > 0 {
          query.push_str(" WHERE ");
          let option = options.iter().last().unwrap();

          let current = schema.iter().find(|x| x.field_name == option.field_name).unwrap();
          let next = option.next.as_ref().unwrap();
          let operator = option.operator.as_ref().unwrap();
          query_match_operators(operator,  &mut query, &option.field_name, &current.field_type, true, Some(next));     
     }

     trace!("Executing get query: {}", query);

     return query;
}

pub(crate) fn generate_where_query<'a, T:  Default + Struct + Clone>(query_builder: &'a QueryBuilder<'_>) -> String {    
     let options = &query_builder.query_options;
     let mut query = format!("WHERE ");

     let schema = query_builder.db.get_schema();

     for option in options.iter().take(options.iter().len() - 1) {
          let current = schema.iter().find(|x| x.field_name == option.field_name).unwrap();
          let next = option.next.as_ref().unwrap();
          let operator = option.operator.as_ref().unwrap();
          query_match_operators(operator,  &mut query, &option.field_name, &current.field_type, false, Some(next));
     }

     let option = options.iter().last().unwrap();

     let current = schema.iter().find(|x| x.field_name == option.field_name).unwrap();
     let next = option.next.as_ref().unwrap();
     let operator = option.operator.as_ref().unwrap();
     query_match_operators(operator,  &mut query, &option.field_name, &current.field_type, true, Some(next));

     trace!("Executing where query: {}", query);

     return query;
}

pub(crate) async fn update_value<T:  Default + Struct + Clone> (
     value: &T, 
     table_name: &str, 
     where_query: &str, 
     connection: &Connection,
     time_to_retry: u32, 
     concurrent: bool) -> 
     Result<usize, ()> {
     let generic_values = get_values_from_generic::<T>(value);

     let mut query = {
          if concurrent {
               format!("BEGIN CONCURRENT; UPDATE {} SET ", table_name)
          } else {
               format!("UPDATE {} SET ", table_name)
          }
     };

     for generic_value in generic_values.iter().take(generic_values.len() - 1) {
          let name = generic_value.field_name.as_str();
          let value = generic_value.field_value;

          if generic_value.field_type == "Vec" {
               let vec_u8 = generic_value.field_value.as_any().downcast_ref::<Vec<u8>>().unwrap();
               let hex = encode(vec_u8);
               let set_query = format!("{} = x'{}'", name, hex);
               query.push_str(set_query.as_str());
               query.push_str(", ");
          }
          else {
               let mut string_value = format!("{:?}", value);

               if string_value.contains("'") {
                    string_value = string_value.replace("'", "''");
               }

               if generic_value.field_type == "bool" {
                    if string_value.contains("false") {
                         _ = string_value = 0.to_string();
                    }
                    else {
                         _ = string_value = 1.to_string();
                    }  
               }

               let set_query = format!("{} = {}", name, string_value).replace("\"", "'");

               query.push_str(set_query.as_str());
               query.push_str(", ");
          }
     }

     let generic_value = generic_values.iter().last().unwrap();

     let name = generic_value.field_name.as_str();
     let value = generic_value.field_value;

     if generic_value.field_type == "Vec" {
          let vec_u8 = generic_value.field_value.as_any().downcast_ref::<Vec<u8>>().unwrap();
          let hex = encode(vec_u8);
          let set_query = format!("{} = x'{}'", name, hex);
          query.push_str(set_query.as_str());
          query.push(' ');
     }
     else {
          let mut string_value = format!("{:?}", value);
               
          if string_value.contains("'") {
               string_value = string_value.replace("'", "''");
          }

          if generic_value.field_type == "bool" {
               if string_value.contains("false") {
                    _ = string_value = 0.to_string();
               }
               else {
                    _ = string_value = 1.to_string();
               }  
          }

          let set_query = format!("{} = {:?}", name, string_value).replace("\"", "'");
          query.push_str(set_query.as_str());
          query.push(' ');     
     }

     query.push_str(where_query);

     if concurrent {
          query.push_str("; COMMIT;");
     }

     trace!("Executing update query: {}", query);

     let mut retries = 0;

     while retries < time_to_retry {
         let function_result = connection.execute(&query, ());
 
         if function_result.is_ok() {
             return Ok(function_result.unwrap());
         }
         else {
             let error = function_result.unwrap_err();
             error!("Error occurred on {} retry. Message: {:?}", retries + 1, error);
             retries = retries + 1;
         }
         tokio::time::sleep(SLEEP_DURATION).await;
     }
 
     return Err(());
}

pub(crate) async fn partial_update<T:  Default + Struct + Clone> (
     field_name: String,
     field_value: String,
     table_name: &str, 
     where_query: &str, 
     connection: &Connection,
     time_to_retry: u32,
     concurrent: bool) -> 
     Result<usize, ()> {

     let mut query = {
          if concurrent {
               format!("BEGIN CONCURRENT; UPDATE {} SET ", table_name)
          } else {
               format!("UPDATE {} SET ", table_name)
          }
     };
     
     let name = field_name;
     let value = field_value;

     let mut string_value = format!("{:?}", value);
               
     if string_value.contains("'") {
          string_value = string_value.replace("'", "''");
     }
     
     let set_query = format!("{} = {}", name, string_value).replace("\"", "'");
     query.push_str(set_query.as_str());
     query.push(' ');

     query.push_str(where_query);

     if concurrent {
          query.push_str("; COMMIT;");
     }

     trace!("Executing partial update query: {}", query);
     
     let mut retries = 0;

     while retries < time_to_retry {
         let function_result = connection.execute(&query, ());
 
         if function_result.is_ok() {
             return Ok(function_result.unwrap());
         }
         else {
             let error = function_result.unwrap_err();
             error!("Error occurred on {} retry. Message: {:?}", retries + 1, error);
             retries = retries + 1;
         }
         tokio::time::sleep(SLEEP_DURATION).await;
     }
 
     return Err(());
}

pub(crate) async fn delete_value<T:  Default + Struct + Clone> (
     table_name: &str, 
     where_query: &str, 
     connection: &Connection,
     time_to_retry: u32) ->
     Result<usize, ()> {
     let mut query = format!("DELETE FROM {} ", table_name);
     query.push_str(where_query);

     trace!("Executing delete query: {}", query);

     let mut retries = 0;

     while retries < time_to_retry {
         let function_result = connection.execute(&query, ());
 
         if function_result.is_ok() {
             return Ok(function_result.unwrap());
         }
         else {
             let error = function_result.unwrap_err();
             error!("Error occurred on {} retry. Message: {:?}", retries + 1, error);
             retries = retries + 1;
         }
         tokio::time::sleep(SLEEP_DURATION).await;
     }
 
     return Err(());
}

pub(crate) async fn any_count_query<T:  Default + Struct + Clone> (
     table_name: &str, 
     where_query: &str) -> String {
     let mut query = format!("SELECT COUNT(*) AS count_total FROM {} ", table_name);
     query.push_str(where_query);

     trace!("Executing any / count query: {}", query);

     return query;
}

pub(crate) async fn all_query<T:  Default + Struct + Clone>(
     table_name: &str) -> String {
     let query = format!("SELECT COUNT(*) AS count_total FROM {} ", table_name);

     trace!("Executing all query: {}", query);

     return query;
}

pub(crate) fn get_single_value<'a, T:  Default + Struct + Clone>(query_result: &mut QueryRowResult<T>) {    
     if let Ok(result) = set_values_from_row_result::<T>(query_result)  {
          query_result.value = Some(result);
     } else {
          query_result.value = None;
     }
}

pub(crate) async fn get_many_values<'a, T:  Default + Struct + Clone>(mut query_result: QueryRowsResult<'a, T>) -> Option<Vec<T>>  { 
     if let Ok(result) = set_values_from_many_rows_result::<T>(&mut query_result)  {
          Some(result)
     } else {
          None
     }
}

pub(crate) fn create_unique_index<T:  Default + Struct + Clone> (
     index_name: &str,
     table_name: &str, 
     columns: Vec<String>) -> String {
     let phantom = T::default();
     let generic_values = get_values_from_generic::<T>(&phantom);
     
     let generic_values_str: Vec<String> = generic_values.iter().map(|x| { let raw = format!("{:?}", *&x.field_name);raw.replace("\"", "") }).collect();

     for column in columns.iter() {
          if !generic_values_str.contains(column) {
               panic!("One of the specified columns isn't field of the struct of type T provided.");
          }
          else {
               break;
          }
     }

     drop(generic_values_str);
     drop(generic_values);
     drop(phantom);

     let mut query = format!("CREATE UNIQUE INDEX IF NOT EXISTS {} ON {} (", index_name, table_name);

     for column in columns.iter().take(columns.iter().count() - 1) {
          let column_string = format!("{},", column);
          query.push_str(&column_string)
     }

     let last_column = columns.iter().last().unwrap();

     let column_string = format!("{});", last_column);
     query.push_str(&column_string);

     trace!("Executing create unique index query: {}", query);

     return query;
}

pub(crate) fn create_index<T:  Default + Struct + Clone> (
     index_name: &str,
     table_name: &str, 
     columns: Vec<String>) -> String {
     let phantom = T::default();
     let generic_values = get_values_from_generic::<T>(&phantom);
     
     let generic_values_str: Vec<String> = generic_values.iter().map(|x| { let raw = format!("{:?}", *&x.field_name);raw.replace("\"", "") }).collect();

     for column in columns.iter() {
          if !generic_values_str.contains(column) {
               panic!("One of the specified columns isn't field of the struct of type T provided.");
          }
          else {
               break;
          }
     }

     drop(generic_values_str);
     drop(generic_values);
     drop(phantom);

     let mut query = format!("CREATE INDEX IF NOT EXISTS {} ON {} (", index_name, table_name);

     for column in columns.iter().take(columns.iter().count() - 1) {
          let column_string = format!("{},", column);
          query.push_str(&column_string)
     }

     let last_column = columns.iter().last().unwrap();

     let column_string = format!("{});", last_column);
     query.push_str(&column_string);

     trace!("Executing create unique index query: {}", query);

     return query;
}

pub(crate) fn drop_index(
     index_name: &str) -> String {
     let query = format!("DROP INDEX IF EXISTS {}", index_name);

     trace!("Executing drop index query: {}", query);

     return query;
}