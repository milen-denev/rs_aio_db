use std::sync::Arc;
use std::sync::RwLock;

use bevy_reflect::Struct;
use libsql::Connection;
use libsql::Builder;
use log::debug;
use log::info;

use crate::db::internal::helpers::get_system_char_delimiter;
use crate::db::internal::queries::alter_table_drop_column;
use crate::db::internal::queries::alter_table_new_column;

use super::aio_query::QueryBuilder;
use super::aio_query::QueryRowResult;
use super::aio_query::QueryRowsResult;
use super::internal::helpers::get_schema_from_generic;
use super::internal::queries::create_table;
use super::internal::queries::delete_value;
use super::internal::queries::get_current_db_schema;
use super::internal::queries::get_many_values;
use super::internal::queries::get_single_value;
use super::internal::queries::insert_value;
use super::internal::queries::update_value;
use super::models::Schema;

pub struct AioDatabase {
     name: String,
     conn: Arc<RwLock<Connection>>,
     schema: Vec<Schema>
}

impl AioDatabase {
     pub async fn create<'a, T:  Default + Struct + Clone>(location: String, name: String) -> AioDatabase {
          let db_location = format!("{}{}{}{}", location, get_system_char_delimiter(), name, ".db");
          let builder = Builder::new_local(db_location).build().await.unwrap();
          let connection = builder.connect().unwrap();
          
          let generic_schema = get_schema_from_generic::<T>();
          let current_schema_option = get_current_db_schema(&name, &connection).await;

          if let Some(current_schema) = current_schema_option {
               debug!("Current Db schema: {:?}", current_schema);

               for current in current_schema.iter() {
                    if !generic_schema.iter().any(|x| x.field_name == current.field_name) {
                         info!("Dropping column: {}", current.field_name.as_str());
                         alter_table_drop_column(&name, current.field_name.as_str(), &connection).await;
                         continue;
                    }
               }

               for generic_field in generic_schema.iter() {
                    if !current_schema.iter().any(|x| x.field_name == generic_field.field_name) {
                         info!("Adding column: {} as {}", generic_field.field_name.as_str(), generic_field.field_type.as_str());
                         alter_table_new_column(&name, generic_field, &connection).await;
                         continue;
                    }
               }
          }
          else {
               debug!("Creating table {} with schema: {:?}", name, generic_schema);
               create_table(&generic_schema, &name, &connection).await;
          }

          AioDatabase {
               name: name,
               conn: Arc::new(RwLock::new(connection)),
               schema: generic_schema
          }
     }

     pub async fn create_in_memory<'a, T:  Default + Struct + Clone>(name: String) -> AioDatabase {
          let builder = Builder::new_local(":memory:").build().await.unwrap();
          let connection = builder.connect().unwrap();
          
          let generic_schema = get_schema_from_generic::<T>();
          let current_schema_option = get_current_db_schema(&name, &connection).await;

          if let Some(current_schema) = current_schema_option {
               debug!("Current Db schema: {:?}", current_schema);

               for current in current_schema.iter() {
                    if !generic_schema.iter().any(|x| x.field_name == current.field_name) {
                         info!("Dropping column: {}", current.field_name.as_str());
                         alter_table_drop_column(&name, current.field_name.as_str(), &connection).await;
                         continue;
                    }
               }

               for generic_field in generic_schema.iter() {
                    if !current_schema.iter().any(|x| x.field_name == generic_field.field_name) {
                         info!("Adding column: {} as {}", generic_field.field_name.as_str(), generic_field.field_type.as_str());
                         alter_table_new_column(&name, generic_field, &connection).await;
                         continue;
                    }
               }
          }
          else {
               debug!("Creating table {} with schema: {:?}", name, generic_schema);
               create_table(&generic_schema, &name, &connection).await;
          }

          AioDatabase {
               name: name,
               conn: Arc::new(RwLock::new(connection)),
               schema: generic_schema
          }
     }

     pub fn get_name(&self) -> &str {
          return self.name.as_str();
     }

     pub fn get_schema(&self) -> &Vec<Schema> {
          return &self.schema;
     }

     pub async fn insert_value<'a, T:  Default + Struct + Clone>(&self, value: T) {
          let r_connection = self.conn.clone();
          let conn = r_connection.read().unwrap();
          insert_value::<T>(&value, self.get_name(), conn).await;
     }

     pub async fn get_single_value<'a, T: Default + Struct + Clone>(&self, query_string: String) -> Option<T> {
          let r_connection = self.conn.clone();
          let conn = r_connection.read().unwrap();

          if let Some(mut query_result) = QueryRowResult::<T>::new(query_string, &conn).await {
               get_single_value::<T>(&mut query_result);
               return query_result.value;
          }
          else {
               return None;
          }
     }

     pub async fn get_many_values<'a, T: Default + Struct + Clone>(&self, query_string: String) -> Option<Vec<T>> {
          let r_connection = self.conn.clone();
          let conn = r_connection.read().unwrap();

          if let Some(mut query_result) = QueryRowsResult::<T>::new_many(query_string, &conn).await {
               get_many_values::<T>(&mut query_result).await;
               return query_result.value;
          }
          else {
               return None;
          }
     }

     pub fn query(&self) -> QueryBuilder {
          return QueryBuilder {
               table_name: self.get_name().to_string(),
               query_options: Vec::default(),
               db: &self
          }
     }

     pub async fn update_value<'a, T: Default + Struct + Clone>(&self, value: T, where_query: String) -> u64 {
          let r_connection = self.conn.clone();
          let conn = r_connection.read().unwrap();

          return update_value::<T>(&value, self.get_name(), &where_query, conn).await;
     }

     pub async fn delete_value<'a, T: Default + Struct + Clone>(&self, where_query: String) -> u64 {
          let r_connection = self.conn.clone();
          let conn = r_connection.read().unwrap();

          return delete_value::<T>(self.get_name(), &where_query, conn).await;
     }
}