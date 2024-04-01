use std::sync::{Arc, RwLock};

use bevy_reflect::Struct;
use libsql::{Connection, Row, Rows};

use super::{aio_database::AioDatabase, internal::queries::{generate_get_query, generate_where_query}};

pub struct QueryBuilder<'a> {
     pub table_name: String,
     pub query_options: Vec<QueryOption<'a>>,
     pub db: &'a AioDatabase
}

pub struct QueryOption<'a> {
     pub field_name: String,
     pub operator: Option<Operator>,
     pub next: Option<Next>,
     query_builder: Option<&'a QueryBuilder<'a>>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
     ///equal
     Eq(String),
     ///Not equal
     Ne(String),
     ///Greater Than
     Gt(String),
     ///Less Than
     Lt(String),
     ///Greater or Equal
     Ge(String),
     ///Less or Equal
     Le(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Next {
     And,
     Or
}

impl QueryBuilder<'_> {
     pub fn new<'a>(db: &'a AioDatabase) -> QueryBuilder<'a> {
          return QueryBuilder {
               table_name: db.get_name().to_string(),
               query_options: Vec::default(),
               db: db
          }
     }

     pub fn field<'a>(&'a self, name: &str) -> QueryOption<'a> {
          QueryOption {
               field_name: name.into(), 
               operator: None,
               query_builder: Some(self),
               next: Some(Next::And)
          }
     }

     pub fn clear(&mut self) {
          self.query_options.clear();
     }

     pub async fn get_single_value<'a, T: Default + Struct + Clone>(self) -> Option<T> {
          let db = self.db;
          let query = generate_get_query::<T>(&self);
          return db.get_single_value::<T>(query).await;
     }

     pub async fn get_many_values<'a, T: Default + Struct + Clone>(self) -> Option<Vec<T>> {
          let db = self.db;
          let query = generate_get_query::<T>(&self);
          return db.get_many_values::<T>(query).await;
     }

     pub async fn update_value<'a, T: Default + Struct + Clone>(self, value: T) {
          let db = self.db;
          let where_query = generate_where_query::<T>(&self);
          db.update_value::<T>(value, where_query).await;
     }
}

impl QueryOption<'_> {
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
pub struct QueryRowResult<T> {
     pub value: Option<T>,
     pub row: Arc<Row>
}

impl<T> QueryRowResult<T> {
     pub async fn new(
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
pub struct QueryRowsResult<T> {
     pub value: Option<Vec<T>>,
     pub rows: Arc<RwLock<Rows>>
}

impl<T> QueryRowsResult<T> {
     pub async fn new_many(
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