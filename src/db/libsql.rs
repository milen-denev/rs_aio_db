use bevy_reflect::Struct;
use libsql::Connection;
use libsql::Builder;
use log::debug;
use log::info;

use crate::db::internal::helpers::get_system_char_delimiter;
use crate::db::internal::queries::alter_table_drop_column;
use crate::db::internal::queries::alter_table_new_column;

use super::internal::helpers::get_schema_from_generic;
use super::internal::queries::create_table;
use super::internal::queries::get_current_db_schema;
use super::models::Schema;

pub struct AioDatabase {
     name: String,
     conn: Connection,
     schema: Vec<Schema>
}

impl AioDatabase {
     pub async fn create<T:  Default + Struct>(location: String, name: String) -> AioDatabase {
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
               create_table(&generic_schema, &name, &connection).await;
          }

          AioDatabase {
               name: name,
               conn: connection,
               schema: generic_schema
          }
     }

     pub fn get_name(&self) -> &str {
          return self.name.as_str();
     }

     pub fn get_schema(&self) -> &Vec<Schema> {
          return &self.schema;
     }

     pub fn get_raw_connection(&self) -> &Connection {
          return &self.conn;
     }
}