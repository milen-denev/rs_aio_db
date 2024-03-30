use bevy_reflect::{Struct, ReflectRef};
use libsql::Connection;
use libsql::Builder;
use log::info;

pub struct AioDatabase {
     name: String,
     conn: Connection,
     schema: Vec<Schema>
}

#[derive(Default, Clone, Debug)]
pub struct Schema {
     pub field_name: String,
     pub field_type: String
}

impl AioDatabase {
     pub async fn create<T:  Default + Struct>(location: String, name: String) -> AioDatabase {
          let db_location = format!("{}{}{}{}", location, get_system_char_delimiter(), name, ".db");
          let builder = Builder::new_local(db_location).build().await.unwrap();
          let connection = builder.connect().unwrap();
          
          let default_t = T::default();
          let default_t2 = T::default();
          let my_struct: Box<dyn Struct> = Box::new(default_t);

          let ReflectRef::Struct(reflected) = default_t2.reflect_ref() else { unreachable!() };

          let count = my_struct.iter_fields().count();
          let mut schema_vec: Vec<Schema> = Vec::with_capacity(count);

          for (i, field) in my_struct.iter_fields().enumerate() {
               let field_name = reflected.name_at(i).unwrap();
               let field_type = field.reflect_type_ident().unwrap();
               info!("Found field named '{}' of type '{}'", field_name, field_type);

               schema_vec.push(Schema {
                    field_name: field_name.into(),
                    field_type: field_type.into()
               });
          }

          let values = format!("{}", name);

          let mut stmt = connection
               .prepare("SELECT sql FROM sqlite_schema WHERE name = ?1")
               .await
               .unwrap();

          let result_row = stmt
               .query([values])
               .await
               .unwrap()
               .next()
               .await
               .unwrap();
          
          if let Some(row) = result_row {
               let rows = row.get::<String>(0).unwrap();
               let current_schema = get_current_schema(rows);

               println!("{:?}", current_schema);

               for current_field in current_schema {
                    //let requested_field = 
               }
          }
          else {
               let mut create_table = format!("CREATE TABLE IF NOT EXISTS {} (", name);
               
               for new_field in schema_vec.iter().take(schema_vec.len() - 1) {
                    let sql_type = get_sql_type(new_field.field_type.clone()).unwrap();
                    create_table = format!("{} {},", create_table, format!("{} {}", new_field.field_name, sql_type));
               }

               let last_field_type = schema_vec.iter().last().unwrap();
               let sql_type = get_sql_type(last_field_type.field_type.clone()).unwrap();
               create_table = format!("{} {})", create_table, format!("{} {}", last_field_type.field_name, sql_type));

               connection.execute(&create_table, ())
                    .await
                    .unwrap();
          }

          AioDatabase {
               name: name,
               conn: connection,
               schema: schema_vec
          }
     }

     pub fn get_name(&self) -> &str {
          return self.name.as_str();
     }

     pub fn get_schema(&self) -> &Vec<Schema> {
          return &self.schema;
     }

}

fn get_system_char_delimiter() -> &'static str {
     let os = std::env::consts::OS;
     if os == "windows" {
         "\\"
     }
     else if os == "linux" {
         "/"
     }
     else {
         panic!("OS not supported.");
     }
}

fn file_exists(full_file_path: &str) -> bool {
     let path = std::path::Path::new(full_file_path);
     path.exists() && path.is_file()
}

fn get_type<T>(_: &T) -> String {
     return format!("{}", std::any::type_name::<T>())
}

fn get_sql_type(rust_type: String) -> Option<String> {
     match rust_type.as_str() {
          "bool" => return Some("NUMERIC".into()),
          "u8" => return Some("INTEGER".into()),
          "u16" => return Some("INTEGER".into()),
          "u32" => return Some("INTEGER".into()),
          "u64" => return Some("INTEGER".into()),
          "u128" => return Some("INTEGER".into()),
          "i8" => return Some("INTEGER".into()),
          "i16" => return Some("INTEGER".into()),
          "i32" => return Some("INTEGER".into()),
          "i64" => return Some("INTEGER".into()),
          "i128" => return Some("INTEGER".into()),
          "f32" => return Some("REAL".into()),
          "f64" => return Some("REAL".into()),
          "char" => return Some("TEXT".into()),
          "String" => return Some("TEXT".into()),
          _ => return None
     }
}

fn get_rust_type(rust_type: String) -> Option<String> {
     match rust_type.as_str() {
          "NUMERIC" => return Some("bool".into()),
          "INTEGER" => return Some("i64".into()),
          "REAL" => return Some("f64".into()),
          "TEXT" => return Some("String".into()),
          _ => return None
     }
}

fn get_current_schema(query_result: String) -> Vec<Schema> {
     let content = extract_parentheses_contents(&query_result).unwrap();
     let list = split_by_comma(&content);
     let schema = split_by_empty_space(list);
     return schema;
}

fn extract_parentheses_contents(input: &str) -> Option<String> {
     let mut result = String::new();
     let mut in_parentheses = false;
 
     for c in input.chars() {
         match c {
             '(' => {
                 in_parentheses = true;
             }
             ')' => {
                 break; // Stop when encountering the closing parenthesis
             }
             _ => {
                 if in_parentheses {
                     result.push(c);
                 }
             }
         }
     }
 
     if !result.is_empty() && in_parentheses {
         Some(result)
     } else {
         None
     }
}

fn split_by_comma(input: &str) -> Vec<&str> {
     return input.split(',').map(|s| s.trim()).collect();
}

fn split_by_empty_space(input: Vec<&str>) -> Vec<Schema> {
     let mut list: Vec<Schema> = Vec::new();

     for field in input {
          let splitted: Vec<&str> = field.split(' ').map(|f| f.trim()).collect();
          list.push(Schema {
               field_name: splitted.first().unwrap().to_string(),
               field_type: get_rust_type(splitted.last().unwrap().to_string()).unwrap()
          });
     }

     return list;
}