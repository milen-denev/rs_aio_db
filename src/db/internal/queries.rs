use bevy_reflect::Struct;
use libsql::Connection;
use log::debug;
use crate::db::models::Schema;
use super::{helpers::get_values_from_generic, schema_gen::{generate_db_schema_query, get_current_schema, get_sql_type}};

pub async fn create_table(schema_vec: &Vec<Schema>, name: &str, connection: &Connection) {
     let create_table_script = generate_db_schema_query(schema_vec, name);
     
     connection.execute(&create_table_script, ())
          .await
          .unwrap();
}

pub async fn get_current_db_schema(name: &str, connection: &Connection) -> Option<Vec<Schema>> {   
     let mut stmt = connection
          .prepare("SELECT sql FROM sqlite_schema WHERE name = ?1")
          .await
          .unwrap();

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

pub async fn alter_table_new_column(name: &str, schema: &Schema, connection: &Connection) {
     let sql_type = get_sql_type(schema.field_type.as_str()).unwrap();
     let column_name = schema.field_name.as_str();

     let query = format!("ALTER TABLE {name} ADD COLUMN {column_name} {sql_type}");

     connection.execute(&query, ())
          .await
          .unwrap();
}

pub async fn alter_table_drop_column(name: &str, column_name: &str, connection: &Connection) {
     let query = format!("ALTER TABLE {name} DROP COLUMN {column_name}");

     connection.execute(&query, ())
          .await
          .unwrap();
}

pub async fn insert_value<T:  Default + Struct + Clone>(value: &T, table_name: &str, connection: &Connection) {
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
}