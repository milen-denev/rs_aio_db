use bevy_reflect::Struct;
use libsql::ffi::Error;
use libsql::Connection;
use libsql::Builder;
use libsql::Database;
use log::debug;
use log::info;
use r2d2::ManageConnection;
use r2d2::Pool;

use crate::db::internal::helpers::get_system_char_delimiter;
use crate::db::internal::queries::alter_table_drop_column;
use crate::db::internal::queries::alter_table_new_column;

use super::aio_query::QueryBuilder;
use super::aio_query::QueryRowResult;
use super::aio_query::QueryRowsResult;
use super::internal::helpers::get_schema_from_generic;
use super::internal::queries::change_db_settings;
use super::internal::queries::create_table;
use super::internal::queries::delete_value;
use super::internal::queries::get_current_db_schema;
use super::internal::queries::get_many_values;
use super::internal::queries::get_single_value;
use super::internal::queries::insert_value;
use super::internal::queries::partial_update;
use super::internal::queries::update_value;
use super::models::Schema;

/// # All in one aka Aio Database
/// ### Locally preserved database example
/// ```rust
/// //This will create a Test.db file at G:\ location
/// let file_db = AioDatabase::create::<Person>("G:\\".into(), "Test".into()).await;
/// ```
/// ### In-memory database example
/// ```rust
/// let in_memory_db = AioDatabase::create_in_memory::<Person>("Test".into()).await;
/// ```
/// #### Create a model
/// ```rust
/// use rs_aio_db::Reflect;
/// 
/// #[derive(Default, Clone, Debug, Reflect)]
/// struct Person {
///     name: String,
///     age: i32,
///     height: i32,
///     married: bool,
/// }
/// ```
/// 
/// #### For Inserting values:
/// ```rust
/// file_db.insert_value(Person {
///    name: "Mylo".into(),
///    age: 0,
///    height: 0,
///    married: true
/// }).await;
/// ```
/// 
/// #### For getting existing values / records:
/// ```rust
/// let get_record = file_db
///    .query()
///    .field("age")
///    .where_is(Operator::Gt(5.to_string()), Some(Next::Or))
///    .field("name")
///    .where_is(Operator::Eq("Mylo".into()), None)
///    .get_many_values::<Person>().await;
/// ```
/// 
/// #### Update existing values / records:
/// ```rust
/// let update_rows = file_db
///    .query()
///    .field("age")
///    .where_is(Operator::Eq((0).to_string()), Some(Next::Or))
///    .update_value(Person {
///        name: "Mylo".into(),
///        age: 5,
///        height: 5,
///        married: false
///    }).await;
/// ```
/// 
/// #### Deleting existing values / records:
/// ```rust
/// let delete_rows = file_db
///    .query()
///    .field("name")
///    .where_is(Operator::Eq("Mylo".into()), None)
///    .delete_value::<Person>().await;
/// ```
pub struct AioDatabase {
     name: String,
     conn: Pool<AioDatabaseConnection>,
     schema: Vec<Schema>
}

pub(crate) struct AioDatabaseConnection {
     builder: Database
}

impl ManageConnection for AioDatabaseConnection {
    type Connection = Connection;
    type Error = Error;

     fn connect(&self) -> Result<Self::Connection, Self::Error> {
          let result = self.builder.connect();
          if let Ok(conn) = result {
               return Ok(conn);
          }
          else {
               return Err(Error::new(-1));
          }
     }

     fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
          return Ok(());
     }

     fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
          false
     }
}

impl AioDatabase {
     ///Create a locally persisted database.
     pub async fn create<'a, T>(location: String, name: String, max_pool_size: u32) -> AioDatabase  where T: Default + Struct + Clone  {       
          let db_location = format!("{}{}{}{}", location, get_system_char_delimiter(), name, ".db");
          let builder = Builder::new_local(db_location).build().await.unwrap();
          let aio_conn = AioDatabaseConnection {
               builder: builder
          };

          let conn_pool = r2d2::Builder::<AioDatabaseConnection>::new().max_size(max_pool_size).build(aio_conn).unwrap();
          let connection = conn_pool.clone().get().unwrap();

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
               change_db_settings(&connection).await;
          }

          let db = AioDatabase {
               name: name,
               conn: conn_pool,
               schema: generic_schema
          };

          return db;
     }

     ///Create an in-memory database.
     pub async fn create_in_memory<'a, T:  Default + Struct + Clone>(name: String, max_pool_size: u32) -> AioDatabase {
          let builder = Builder::new_local(":memory:").build().await.unwrap();
          let aio_conn = AioDatabaseConnection {
               builder: builder
          };

          let conn_pool = r2d2::Builder::<AioDatabaseConnection>::new().max_size(max_pool_size).build(aio_conn).unwrap();
          
          let connection = conn_pool.clone().get().unwrap();

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
               change_db_settings(&connection).await;
          }
          
          let db = AioDatabase {
               name: name,
               conn: conn_pool,
               schema: generic_schema
          };

          return db;
     }

     /// Get the name of the database and table's name as well.
     pub fn get_name(&self) -> &str {
          return self.name.as_str();
     }

     /// Get the schema of the struct / database.
     pub fn get_schema(&self) -> &Vec<Schema> {
          return &self.schema;
     }

     /// Inserts a **T** value in the database. 
     pub async fn insert_value<'a, T:  Default + Struct + Clone>(&self, value: &T) {
          let conn = self.conn.clone().get().unwrap();
          insert_value::<T>(&value, self.get_name(), conn).await;
     }

     /// Creates a QueryBuilder that allows to chain query filters for different field / columns.
     pub fn query(&self) -> QueryBuilder {
          return QueryBuilder {
               table_name: self.get_name().to_string(),
               query_options: Vec::default(),
               db: &self
          }
     }
     
     pub(crate) async fn get_single_value<'a, T: Default + Struct + Clone>(&self, query_string: String) -> Option<T> {
          let conn = self.conn.clone().get().unwrap();
          
          if let Some(mut query_result) = QueryRowResult::<T>::new(query_string, conn).await {
               get_single_value::<T>(&mut query_result);
               return query_result.value;
          }
          else {
               return None;
          }
     }

     pub(crate) async fn get_many_values<'a, T: Default + Struct + Clone>(&self, query_string: String) -> Option<Vec<T>> {
          let conn = self.conn.clone().get().unwrap();

          if let Some(mut query_result) = QueryRowsResult::<T>::new_many(query_string, conn).await {
               get_many_values::<T>(&mut query_result).await;
               return query_result.value;
          }
          else {
               return None;
          }
     }

     pub(crate) async fn update_value<'a, T: Default + Struct + Clone>(&self, value: T, where_query: String) -> u64 {
          let conn = self.conn.clone().get().unwrap();
          return update_value::<T>(&value, self.get_name(), &where_query, conn).await;
     }

     pub(crate) async fn partial_update<'a, T: Default + Struct + Clone>(&self, field_name: String, field_value: String, where_query: String) -> u64 {
          let conn = self.conn.clone().get().unwrap();
          return partial_update::<T>(field_name, field_value, self.get_name(), &where_query, conn).await;
     }

     pub(crate) async fn delete_value<'a, T: Default + Struct + Clone>(&self, where_query: String) -> u64 {
          let conn = self.conn.clone().get().unwrap();
          return delete_value::<T>(self.get_name(), &where_query, conn).await;
     }
}