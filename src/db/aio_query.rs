use std::sync::{Arc, RwLock};

use bevy_reflect::Struct;
use libsql::{Connection, Row, Rows};

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
     Le(String)
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

     /// Updates **all values** that matches the query filter with values of the struct of type **T**. Returns the number of rows affected.
     pub async fn update_value<'a, T: Default + Struct + Clone>(self, value: T)  -> u64 {
          let db = self.db;
          let where_query = generate_where_query::<T>(&self);
          return db.update_value::<T>(value, where_query).await;
     }

     /// Updates specific field / column that matches the query filter. Returns the number of rows affected.
     pub async fn partial_update<'a, T: Default + Struct + Clone>(self, field_name: String, field_value: String)  -> u64 {
          let db = self.db;
          let where_query = generate_where_query::<T>(&self);
          return db.partial_update::<T>(field_name, field_value, where_query).await;
     }

     /// Deletes **all values** hat match the query filter. Returns the number of rows affected.
     pub async fn delete_value<'a, T: Default + Struct + Clone>(self) -> u64 {
          let db = self.db;
          let where_query = generate_where_query::<T>(&self);
          return db.delete_value::<T>(where_query).await;
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

#[derive(Clone)]
pub(crate) struct QueryRowResult<T> {
     pub value: Option<T>,
     pub row: Arc<Row>
}

impl<T> QueryRowResult<T> {
     pub(crate) async fn new(
          query: String, 
          connection: &Connection) -> Option<QueryRowResult<T>> { 
          let row_result = connection
               .query(&query, ())
               .await
               .unwrap()
               .next()
               .await;

          if let Ok(row_option) = row_result {
               if let Some(row) = row_option {
                    return Some(QueryRowResult::<T> {
                         value: None,
                         row: Arc::new(row)
                    });
               }
               else {
                    return None;
               }
          }
          else {
               return None;
          }
     }
}

#[derive(Clone)]
pub(crate) struct QueryRowsResult<T> {
     pub value: Option<Vec<T>>,
     pub rows: Arc<RwLock<Rows>>
}

impl<T> QueryRowsResult<T> {
     pub(crate) async fn new_many(
          query: String, 
          connection: &Connection) -> Option<QueryRowsResult<T>> { 
          let rows = connection
               .query(&query, ())
               .await
               .unwrap();
          
          if rows.column_count() > 0 {
               return Some(QueryRowsResult::<T> {
                    value: None,
                    rows: Arc::new(RwLock::new(rows))
               });
          }
          else {
               return None
          }
     }
}