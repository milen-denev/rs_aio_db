use crate::db::models::Schema;

pub fn get_sql_type(rust_type: &str) -> Option<String> {
     match rust_type {
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

pub fn get_current_schema(query_result: String) -> Vec<Schema> {
     let content = extract_parentheses_contents(&query_result).unwrap();
     let list = split_by_comma(&content);
     let schema = split_by_empty_space(list);
     return schema;
}

pub fn extract_parentheses_contents(input: &str) -> Option<String> {
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

pub fn split_by_comma(input: &str) -> Vec<&str> {
     return input.split(',').map(|s| s.trim()).collect();
}

pub fn split_by_empty_space(input: Vec<&str>) -> Vec<Schema> {
     let mut list: Vec<Schema> = Vec::new();

     for field in input {
          let splitted: Vec<&str> = field.split(' ').map(|f| f.trim()).collect();
          list.push(Schema {
               field_name: splitted.first().unwrap().to_string(),
               field_type: splitted.last().unwrap().to_string()
          });
     }

     return list;
}

pub fn generate_db_schema_query(schema_vec: &Vec<Schema>, name: &str) -> String {
     let mut create_table = format!("CREATE TABLE IF NOT EXISTS {} (", name);
               
     for new_field in schema_vec.iter().take(schema_vec.len() - 1) {
          let sql_type = get_sql_type(new_field.field_type.as_str()).unwrap();
          create_table = format!("{} {},", create_table, format!("{} {}", new_field.field_name, sql_type));
     }

     let last_field_type = schema_vec.iter().last().unwrap();
     let sql_type = get_sql_type(last_field_type.field_type.as_str()).unwrap();
     create_table = format!("{} {})", create_table, format!("{} {}", last_field_type.field_name, sql_type));

     return create_table;
}