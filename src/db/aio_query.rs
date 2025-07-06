use core::str;

use bevy_reflect::{Reflect, ReflectMut, Struct};
use tokio_rusqlite::{Connection, Row};

use super::{aio_database::AioDatabase, internal::queries::{generate_get_query, generate_where_query}};

/// Used for building a SQL query through a simple Rust API for querying AioDatabase.
/// ### Example
/// ```rust
/// let query  = QueryBuilder::new(&file_db)
///     .field("name")
///     .where_is(Operator::Eq("Mylo".into()), None);
/// ```
pub struct QueryBuilder<'a> {
     pub table_name: String,
     pub query_options: Vec<QueryOption<'a>>,
     pub db: &'a AioDatabase
}

unsafe impl<'a> Send for QueryBuilder<'a> { }

/// Part of QueryBuilder's API for generating query.
/// ### Example
/// ```rust
/// let query  = QueryBuilder::new(&file_db)
///     .field("name")
///     .where_is(Operator::Eq("Mylo".into()), None);
/// ```
pub struct QueryOption<'a> {
     pub field_name: String,
     pub operator: Option<Operator>,
     pub next: Option<Next>,
     query_builder: Option<&'a QueryBuilder<'a>>
}

unsafe impl<'a> Send for QueryOption<'a> { }

/// # Inspired by OData filter queries.
/// - **Eq** = Equal
/// - **Ne** = Not equal
/// - **Gt** = Greater Than
/// - **Lt** = Less Than
/// - **Ge** = Greater or Equal
/// - **Le** = Less or Equal
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
     Eq(String),
     Ne(String),
     Gt(String),
     Lt(String),
     Ge(String),
     Le(String),
     Contains(String),
     StartsWith(String),
     EndsWith(String)
}

/// Use this for declaring what the next query filter will be if any (**AND** or **OR**).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Next {
     And,
     Or
}

impl QueryBuilder<'_> {
     /// Create a new instance of a QueryBuilder, used for querying 
     pub fn new<'a>(db: &'a AioDatabase) -> QueryBuilder<'a> {
          return QueryBuilder {
               table_name: db.get_name().to_string(),
               query_options: Vec::default(),
               db: db
          }
     }

     /// Declare which **field (column)** you want to query.
     /// ```rust
     /// let query_options  = QueryBuilder::new(&file_db)
     ///     .field("name");
     /// ```
     pub fn field<'a>(&'a self, name: &str) -> QueryOption<'a> {
          QueryOption {
               field_name: name.into(), 
               operator: None,
               query_builder: Some(self),
               next: Some(Next::And)
          }
     }

     /// Clears out all query options
     pub fn clear(&mut self) {
          self.query_options.clear();
     }

     /// Return the first **value (row)** that matched or **None** if there are not query matches. 
     pub async fn get_single_value<'a, T: Default + Struct + Clone>(self) -> Option<T> {
          let db = self.db;
          let query = generate_get_query::<T>(&self);
          return db.get_single_value::<T>(query).await;
     }

     /// Return the all **values (rows)** that matched or **None** if there are not query matches. 
     pub async fn get_many_values<'a, T: Default + Struct + Clone>(self) -> Option<Vec<T>> {
          let db = self.db;
          let query = generate_get_query::<T>(&self);
          return db.get_many_values::<T>(query).await;
     }

     /// Updates **all values** that matches the query filter with values of the struct of type **T**. Returns a Result of the number of rows affected or error if update was unsuccessful.
     pub async fn update_value<'a, T: Default + Struct + Clone>(self, value: T)  -> Result<usize, String> {
          let db = self.db;
          let where_query = generate_where_query::<T>(&self);
          return db.update_value::<T>(value, where_query).await;
     }

     /// Updates concurrently **all values** that matches the query filter with values of the struct of type **T**. Returns a Result of the number of rows affected or error if update was unsuccessful.
     pub async fn update_value_concurrent<'a, T: Default + Struct + Clone>(self, value: T)  -> Result<usize, String> {
          let db = self.db;
          let where_query = generate_where_query::<T>(&self);
          return db.update_value_concurrent::<T>(value, where_query).await;
     }

     /// Updates specific field / column that matches the query filter. Returns a Result of the number of rows affected or error if update was unsuccessful.
     pub async fn partial_update<'a, T: Default + Struct + Clone>(self, field_name: String, field_value: String)  -> Result<usize, String> {
          let db = self.db;
          let where_query = generate_where_query::<T>(&self);
          return db.partial_update::<T>(field_name, field_value, where_query).await;
     }

     /// Updates concurrently specific field / column that matches the query filter. Returns a Result of the number of rows affected or error if update was unsuccessful.
     pub async fn partial_update_concurrent<'a, T: Default + Struct + Clone>(self, field_name: String, field_value: String)  -> Result<usize, String> {
          let db = self.db;
          let where_query = generate_where_query::<T>(&self);
          return db.partial_update_concurrent::<T>(field_name, field_value, where_query).await;
     }

     /// Deletes **all values** that match the query filter. Returns a Result of the number of rows affected or error if update was unsuccessful.
     pub async fn delete_value<'a, T: Default + Struct + Clone>(self) -> Result<usize, String> {
          let db = self.db;
          let where_query = generate_where_query::<T>(&self);
          return db.delete_value::<T>(where_query).await;
     }

     /// Returns if any value / row matches the the query filter.
     pub async fn any<'a, T: Default + Struct + Clone>(self) -> bool {
          let db = self.db;
          let where_query = generate_where_query::<T>(&self);
          return db.any::<T>(where_query).await;
     }

     /// Returns the count of values / rows that match the the query filter.
     pub async fn count<'a, T: Default + Struct + Clone>(self) -> u64 {
          let db = self.db;
          let where_query = generate_where_query::<T>(&self);
          return db.count::<T>(where_query).await;
     }

     /// Returns if all rows / records match the the query filter.
     pub async fn all<'a, T: Default + Struct + Clone>(self) -> bool {
          let db = self.db;
          let where_query = generate_where_query::<T>(&self);
          return db.all::<T>(where_query).await;
     }
}

impl QueryOption<'_> {

     /// Define the Operator and it's value which will be used for the **WHERE** clause, and the Next which will be used for chaining with the next clause. by default None is equal to **Next::And** if there are more than 1 clauses.
     pub fn where_is<'a>(&'a self, operator: Operator, next: Option<Next>) -> QueryBuilder<'a> {
          let mut query = QueryBuilder {
               table_name: self.query_builder.unwrap().table_name.clone(),
               query_options: self.query_builder.unwrap().query_options.iter().map(|x| QueryOption {
                    field_name: x.field_name.clone(),
                    operator: x.operator.clone(),
                    query_builder: x.query_builder,
                    next: x.next.clone()
               }).collect(),
               db: self.query_builder.unwrap().db
          };

          if let Some(next) = next  {
               query.query_options.push(QueryOption {
                    field_name: self.field_name.clone(),
                    operator: Some(operator),
                    query_builder: None,
                    next: Some(next)
               });
          }
          else {
               query.query_options.push(QueryOption {
                    field_name: self.field_name.clone(),
                    operator: Some(operator),
                    query_builder: None,
                    next: Some(Next::And)
               });
          }
          
          return query;
     }
}

pub(crate) struct QueryRowResult<T> {
     pub value: Option<T>,
}

impl<T: Default + Struct + Clone> QueryRowResult<T> {
     pub(crate) async fn new(
          query: String, 
          connection: &Connection) -> Option<QueryRowResult<T>> { 
          
          let result = connection.call(move |conn| {
               let mut stmt = conn.prepare(&query)?;
               let mut rows = stmt.query([])?;
               
               if let Some(row) = rows.next()? {
                    let mapped_value = map_row_to_struct::<T>(row)?;
                    Ok(Some(mapped_value))
               } else {
                    Ok(None)
               }
          }).await;
          
          match result {
               Ok(value) => Some(QueryRowResult { value }),
               Err(_) => None
          }
     }
}

pub(crate) struct QueryRowsResult<T> {
     pub value: Option<Vec<Result<T, Error>>>,
}

impl<T: Default + Struct + Clone> QueryRowsResult<T> {
     pub(crate) async fn new_many(
          query: String, 
          connection: &Connection) -> Option<QueryRowsResult<T>> {
          
          let result = connection.call(move |conn| {
               let mut stmt = conn.prepare(&query)?;
               let rows = stmt.query_map([], |row| {
                    Ok(map_row_to_struct::<T>(row))
               })?;
               
               let mut results = Vec::new();
               
               for row_result in rows {
                    match row_result {
                         Ok(mapped_value) => results.push(mapped_value),
                         Err(e) => return Err(e.into())
                    }
               }
               
               if results.is_empty() {
                    Ok(None)
               } else {
                    Ok(Some(results))
               }
          }).await;
          
          match result {
               Ok(value) => Some(QueryRowsResult { value }),
               Err(_) => None
          }
     }
}

#[derive(Default, Reflect, Clone)]
pub(crate) struct AnyCountResult {
     pub count_total: u64
}

use bevy_reflect::GetField;
use tokio_rusqlite::Error;

// Helper function to map rusqlite::Row to Bevy Struct using reflection
fn map_row_to_struct<T: Default + Struct + Clone>(row: &Row) -> Result<T, Error> {
     let mut instance = T::default();

     let mut struct_mut2: Box<dyn Struct> = Box::new(T::default());
     let ReflectMut::Struct(reflected2) = struct_mut2.reflect_mut() else { unreachable!() };
     let struct_immutable: Box<dyn Struct> = Box::new(T::default());

     for (index, field) in struct_immutable.iter_fields().enumerate() {
          let field_type = field.reflect_type_ident().unwrap();
          let field_name = reflected2.name_at(index).clone().unwrap();

          // Try to get the value from the row by field name first, then by index
          match field_type {
               "bool" => {
                    let value: Result<i32, _> = row.get(field_name).or_else(|_| row.get(index));
                    if let Ok(sql_value) = value {
                         let bool_value = sql_value != 0;
                         *instance.get_field_mut::<bool>(field_name).unwrap() = bool_value;
                    }
               },
               "u8" => {
                    let value: Result<i64, _> = row.get(field_name).or_else(|_| row.get(index));
                    if let Ok(sql_value) = value {
                         *instance.get_field_mut::<u8>(field_name).unwrap() = sql_value as u8;
                    }
               },
               "u16" => {
                    let value: Result<i64, _> = row.get(field_name).or_else(|_| row.get(index));
                    if let Ok(sql_value) = value {
                         *instance.get_field_mut::<u16>(field_name).unwrap() = sql_value as u16;
                    }
               },
               "u32" => {
                    let value: Result<i64, _> = row.get(field_name).or_else(|_| row.get(index));
                    if let Ok(sql_value) = value {
                         *instance.get_field_mut::<u32>(field_name).unwrap() = sql_value as u32;
                    }
               },
               "u64" => {
                    let value: Result<i64, _> = row.get(field_name).or_else(|_| row.get(index));
                    if let Ok(sql_value) = value {
                         *instance.get_field_mut::<u64>(field_name).unwrap() = sql_value as u64;
                    }
               },
               "i8" => {
                    let value: Result<i64, _> = row.get(field_name).or_else(|_| row.get(index));
                    if let Ok(sql_value) = value {
                         *instance.get_field_mut::<i8>(field_name).unwrap() = sql_value as i8;
                    }
               },
               "i16" => {
                    let value: Result<i64, _> = row.get(field_name).or_else(|_| row.get(index));
                    if let Ok(sql_value) = value {
                         *instance.get_field_mut::<i16>(field_name).unwrap() = sql_value as i16;
                    }
               },
               "i32" => {
                    let value: Result<i32, _> = row.get(field_name).or_else(|_| row.get(index));
                    if let Ok(sql_value) = value {
                         *instance.get_field_mut::<i32>(field_name).unwrap() = sql_value;
                    }
               },
               "i64" => {
                    let value: Result<i64, _> = row.get(field_name).or_else(|_| row.get(index));
                    if let Ok(sql_value) = value {
                         *instance.get_field_mut::<i64>(field_name).unwrap() = sql_value;
                    }
               },
               "f32" => {
                    let value: Result<f64, _> = row.get(field_name).or_else(|_| row.get(index));
                    if let Ok(sql_value) = value {
                         *instance.get_field_mut::<f32>(field_name).unwrap() = sql_value as f32;
                    }
               },
               "f64" => {
                    let value: Result<f64, _> = row.get(field_name).or_else(|_| row.get(index));
                    if let Ok(sql_value) = value {
                         *instance.get_field_mut::<f64>(field_name).unwrap() = sql_value;
                    }
               },
               "char" => {
                    let value: Result<String, _> = row.get(field_name).or_else(|_| row.get(index));
                    if let Ok(sql_value) = value {
                         let char_value = sql_value.chars().next().unwrap_or(' ');
                         *instance.get_field_mut::<char>(field_name).unwrap() = char_value;
                    }
               },
               "String" => {
                    let value: Result<String, _> = row.get(field_name).or_else(|_| row.get(index));
                    if let Ok(sql_value) = value {
                         // Create buffer similar to your existing code
                         let mut buffer: Vec<u8> = Vec::new();
                         buffer.extend_from_slice(sql_value.as_bytes());
                         let string_value = String::from_utf8_lossy(&buffer).into_owned();
                         *instance.get_field_mut::<String>(field_name).unwrap() = string_value;
                    }
               },
               "Vec" => {
                    // Handle Vec<u8> stored as BLOB or hex string
                    let blob_result: Result<Vec<u8>, _> = row.get(field_name).or_else(|_| row.get(index));
                    if let Ok(sql_value) = blob_result {
                         let mut buffer: Vec<u8> = Vec::new();
                         buffer.extend_from_slice(&sql_value);
                         *instance.get_field_mut::<Vec<u8>>(field_name).unwrap() = buffer;
                    } else {
                         // Try as hex string if BLOB retrieval fails
                         let string_result: Result<String, _> = row.get(field_name).or_else(|_| row.get(index));
                         if let Ok(hex_string) = string_result {
                              if let Ok(decoded) = hex::decode(&hex_string) {
                                   let mut buffer: Vec<u8> = Vec::new();
                                   buffer.extend_from_slice(&decoded);
                                   *instance.get_field_mut::<Vec<u8>>(field_name).unwrap() = buffer;
                              }
                         }
                    }
               },
               _ => {
                    panic!("{} type not supported.", field_type);
               }
          }
     }

    Ok(instance)
}