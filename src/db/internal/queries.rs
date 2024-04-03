use std::sync::Arc;

use bevy_reflect::Struct;
use libsql::Connection;
use log::debug;
use r2d2::PooledConnection;
use crate::db::{aio_database::AioDatabaseConnection, aio_query::{QueryBuilder, QueryRowResult, QueryRowsResult}, internal::helpers::{get_values_from_generic, query_match_operators}, models::Schema};
use super::{helpers::set_values_from_row_result, schema_gen::{generate_db_schema_query, get_current_schema, get_sql_type}};

pub(crate) async fn create_table(schema_vec: &Vec<Schema>, name: &str, connection: &Connection) {
     let create_table_script = generate_db_schema_query(schema_vec, name);
     
     connection.execute(&create_table_script, ())
          .await
          .unwrap();
}

pub(crate) async fn change_db_settings(connection: &Connection) {
     _ = connection.execute("PRAGMA journal_mode=WAL;", ())
          .await;

     _ = connection.execute("PRAGMA journal_size_limit=-1;", ())
          .await;

     _ = connection.execute("PRAGMA auto_vacuum=2;", ())
          .await;
}

pub(crate) async fn get_current_db_schema(name: &str, connection: &Connection) -> Option<Vec<Schema>> {   
     let stmt_res = connection
          .prepare("SELECT sql FROM sqlite_schema WHERE name = ?1")
          .await;

     if let Ok(mut stmt) = stmt_res {
          let result_row = stmt
               .query([name])
               .await
               .unwrap()
               .next()
               .await
               .unwrap();

          if let Some(row) = result_row {
               let rows = row.get::<String>(0).unwrap();
               let current_schema = get_current_schema(rows);
               return Some(current_schema);
          }
          else {
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

     connection.execute(&query, ())
          .await
          .unwrap();
}

pub(crate) async fn alter_table_drop_column(name: &str, column_name: &str, connection: &Connection) {
     let query = format!("ALTER TABLE {name} DROP COLUMN {column_name}");

     connection.execute(&query, ())
          .await
          .unwrap();
}

pub(crate) async fn insert_value<T:  Default + Struct + Clone>(value: &T, table_name: &str, connection: PooledConnection<AioDatabaseConnection>) {
     let generic_values = get_values_from_generic::<T>(value);
     let mut query = format!("INSERT INTO {} (", table_name);

     for generic_value in generic_values.iter().take(generic_values.len() - 1) {
          query.push_str(generic_value.field_name.as_str());
          query.push(',');
     }

     query.push_str(generic_values.iter().last().unwrap().field_name.as_str());
     query.push(')');


     query.push_str(" VALUES (");

     for generic_value in generic_values.iter().take(generic_values.len() - 1) {
          query.push_str(format!("{:?}", generic_value.field_value).as_str());
          query.push(',');
     }

     query.push_str(format!("{:?}", generic_values.last().unwrap().field_value).as_str());
     query.push(')');

     debug!("Executing insert query: {}", query);

     connection.execute(&query, ())
          .await
          .unwrap();

     drop(connection);
}

pub(crate) fn generate_get_query<'a, T:  Default + Struct + Clone>(query_builder: &'a QueryBuilder<'_>) -> String {    
     let options = &query_builder.query_options;
     let table_name = &query_builder.table_name;
     let mut query = format!("SELECT * FROM {table_name} WHERE ");

     let schema = query_builder.db.get_schema();

     let len = options.len();

     if len > 1 {
          for option in options.iter().take(options.iter().len() - 1) {
               let current = schema.iter().find(|x| x.field_name == option.field_name).unwrap();
               let next = option.next.as_ref().unwrap();
               let operator = option.operator.as_ref().unwrap();
               query_match_operators(operator,  &mut query, &option.field_name, &current.field_type, false, Some(next));
          }
     }

     if len > 0 {
          let option = options.iter().last().unwrap();

          let current = schema.iter().find(|x| x.field_name == option.field_name).unwrap();
          let next = option.next.as_ref().unwrap();
          let operator = option.operator.as_ref().unwrap();
          query_match_operators(operator,  &mut query, &option.field_name, &current.field_type, true, Some(next));     
     }

     debug!("Executing select query: {}", query);

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

     debug!("Executing select query: {}", query);

     return query;
}

pub(crate) async fn update_value<T:  Default + Struct + Clone>(
     value: &T, 
     table_name: &str, 
     where_query: &str, 
     connection: PooledConnection<AioDatabaseConnection>) -> u64 {
     let generic_values = get_values_from_generic::<T>(value);
     let mut query = format!("UPDATE {} SET ", table_name);

     for generic_value in generic_values.iter().take(generic_values.len() - 1) {
          let name = generic_value.field_name.as_str();
          let value = generic_value.field_value;
          let set_query = format!("{} = {:?}", name, value).replace("\"", "'");
          query.push_str(set_query.as_str());
          query.push_str(", ");
     }

     let generic_value = generic_values.iter().last().unwrap();

     let name = generic_value.field_name.as_str();
     let value = generic_value.field_value;
     let set_query = format!("{} = {:?}", name, value).replace("\"", "'");
     query.push_str(set_query.as_str());
     query.push(' ');

     query.push_str(where_query);

     debug!("Executing insert query: {}", query);

     let rows_affected = connection.execute(&query, ())
          .await
          .unwrap();

     drop(connection);

     return rows_affected;
}

pub(crate) async fn partial_update<T:  Default + Struct + Clone>(
     field_name: String,
     field_value: String,
     table_name: &str, 
     where_query: &str, 
     connection: PooledConnection<AioDatabaseConnection>) -> u64 {
     
     let mut query = format!("UPDATE {} SET ", table_name);

     let name = field_name;
     let value = field_value;
     let set_query = format!("{} = {}", name, value).replace("\"", "'");
     query.push_str(set_query.as_str());
     query.push(' ');

     query.push_str(where_query);

     debug!("Executing insert query: {}", query);

     let rows_affected = connection.execute(&query, ())
          .await
          .unwrap();

     drop(connection);

     return rows_affected;
}

pub(crate) async fn delete_value<T:  Default + Struct + Clone>(
     table_name: &str, 
     where_query: &str, 
     connection: PooledConnection<AioDatabaseConnection>) -> u64 {
     let mut query = format!("DELETE FROM {} ", table_name);
     query.push_str(where_query);

     debug!("Executing insert query: {}", query);

     let rows_affected = connection.execute(&query, ())
          .await
          .unwrap();

     drop(connection);

     return rows_affected;
}

pub(crate) fn get_single_value<'a, T:  Default + Struct + Clone>(query_result: &mut QueryRowResult<T>) {    
     let result = set_values_from_row_result::<T>(query_result);
     query_result.value = Some(result);
}

pub(crate) async fn get_many_values<'a, T:  Default + Struct + Clone>(query_result: &mut QueryRowsResult<T>) { 
     let arc_rows = query_result.rows.clone();
     let mut rows = arc_rows.write().unwrap();

     let mut vec: Vec<T> = Vec::new();

     while let Ok(row) = rows.next().await {
          if let Some(content) = row {
               let query_row_result:  QueryRowResult<T> = QueryRowResult {
                    row: Arc::new(content),
                    value: None
               };
               let result = set_values_from_row_result::<T>(&query_row_result);
               vec.push(result);
          }
          else {
               break;
          }
     }

     query_result.value = Some(vec);
}