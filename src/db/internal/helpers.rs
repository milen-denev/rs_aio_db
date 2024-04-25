use bevy_reflect::{GetField, ReflectMut, ReflectRef, Struct};
use log::{debug, info};

use crate::db::{aio_query::{Next, Operator, QueryRowResult}, models::{GenericValue, Schema}};

pub(crate) fn get_system_char_delimiter() -> &'static str {
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

pub(crate) fn get_schema_from_generic<T:  Default + Struct>() -> Box<Vec<Schema>> {  
     let default_t = T::default();
     let default_t2 = T::default();
     let my_struct: Box<dyn Struct> = Box::new(default_t);

     let ReflectRef::Struct(reflected) = default_t2.reflect_ref() else { unreachable!() };

     let count = my_struct.iter_fields().count();
     let mut schema_vec: Box<Vec<Schema>> = Box::new(Vec::with_capacity(count));

     for (i, field) in my_struct.iter_fields().enumerate() {
          let field_name = reflected.name_at(i).unwrap();
          let field_type = field.reflect_type_ident().unwrap();
          info!("Found field named '{}' of type '{}'", field_name, field_type);

          schema_vec.push(Schema {
               field_name: field_name.into(),
               field_type: field_type.into()
          });
     }
     
     return schema_vec;
}

pub(crate) fn get_values_from_generic<'a, T:  Default + Struct + Clone>(value: &'a T) -> Vec<GenericValue> {  
     let copied_value = value.clone();
     let my_struct: Box<dyn Struct> = Box::new(copied_value);

     let ReflectRef::Struct(reflected) = value.reflect_ref() else { unreachable!() };

     let count = my_struct.iter_fields().count();
     let mut schema_vec: Vec<GenericValue> = Vec::with_capacity(count);

     for (i, field) in my_struct.iter_fields().enumerate() {
          let field_name = reflected.name_at(i).unwrap();
          let field_value = reflected.field_at(i).unwrap();
          let field_type = field.reflect_type_ident().unwrap();
          
          debug!("Found field named '{}' with value '{:?}'", field_name, field_value);
    
          schema_vec.push(GenericValue {
               field_name: field_name.into(),
               field_value: field_value,
               field_type: field_type.into()
          });
     }
     
     return schema_vec;
}

pub(crate) fn set_values_from_row_result<'a, T:  Default + Struct + Clone>(row_result: &QueryRowResult<T>) -> T {  
     let mut struct_mut2: Box<dyn Struct> = Box::new(T::default());
     let ReflectMut::Struct(reflected2) = struct_mut2.reflect_mut() else { unreachable!() };
     let struct_immutable: Box<dyn Struct> = Box::new(T::default());
     
     let mut t_struct = T::default();

     for (i, field) in struct_immutable.iter_fields().enumerate() {
          let field_type = field.reflect_type_ident().unwrap();
          let field_name = reflected2.name_at(i).clone().unwrap();

          match field_type {
               "bool" => {
                    *t_struct.get_field_mut::<bool>(field_name).unwrap() = match row_result.row.get::<i32>(i as i32).unwrap_or(0) {
                         0 => { false }
                         1 => { true },
                         _ => panic!("Invalid bool value")
                    };
               },
               "u8" => {
                    *t_struct.get_field_mut::<u8>(field_name).unwrap() = row_result.row.get::<u32>(i as i32).unwrap_or(0) as u8;
               },
               "u16" => {
                    *t_struct.get_field_mut::<u16>(field_name).unwrap() = row_result.row.get::<u32>(i as i32).unwrap_or(0) as u16;
               },
               "u32" => {
                    *t_struct.get_field_mut::<u32>(field_name).unwrap() = row_result.row.get::<u32>(i as i32).unwrap_or(0) as u32;
               },
               "u64" => {
                    *t_struct.get_field_mut::<u64>(field_name).unwrap() = row_result.row.get::<u32>(i as i32).unwrap_or(0) as u64;
               },
               "u128" => {
                    *t_struct.get_field_mut::<u128>(field_name).unwrap() = row_result.row.get::<u32>(i as i32).unwrap_or(0) as u128;
               },
               "i8" => {
                    *t_struct.get_field_mut::<i8>(field_name).unwrap() = row_result.row.get::<i32>(i as i32).unwrap_or(0) as i8;
               },
               "i16" => {
                    *t_struct.get_field_mut::<i16>(field_name).unwrap() = row_result.row.get::<i32>(i as i32).unwrap_or(0) as i16;
               },
               "i32" => {
                    *t_struct.get_field_mut::<i32>(field_name).unwrap() = row_result.row.get::<i32>(i as i32).unwrap_or(0) as i32;
               },
               "i64" => {
                    *t_struct.get_field_mut::<i64>(field_name).unwrap() = row_result.row.get::<i32>(i as i32).unwrap_or(0) as i64;
               },
               "i128" => {
                    *t_struct.get_field_mut::<i128>(field_name).unwrap() = row_result.row.get::<i32>(i as i32).unwrap_or(0) as i128;
               },
               "f32" => {
                    *t_struct.get_field_mut::<f32>(field_name).unwrap() = row_result.row.get::<f64>(i as i32).unwrap_or(0.00f64) as f32;
               },
               "f64" => {
                    *t_struct.get_field_mut::<f64>(field_name).unwrap() = row_result.row.get::<f64>(i as i32).unwrap_or(0.00f64) as f64;
               },
               "char" => {
                    *t_struct.get_field_mut::<char>(field_name).unwrap() = row_result.row.get::<String>(i as i32).unwrap_or(' '.into()).pop().unwrap() as char;
               },
               "String" => {
                    let value = row_result.row.get::<String>(i as i32).unwrap_or("".into());
                    *t_struct.get_field_mut::<String>(field_name).unwrap() = value;
               },
               "Vec" => {
                    let value = row_result.row.get::<Vec<u8>>(i as i32).unwrap();
                    //let vec_u8_data = hex::decode(value).unwrap();
                    *t_struct.get_field_mut::<Vec<u8>>(field_name).unwrap() = value;
               },
               _ => panic!("{} type not supported.", field_type)
          }
     }

     return t_struct;
}

pub(crate) fn get_next(next: &Next) -> String {
     if next == &Next::And {
          return "AND".into();
     }
     else {
          return "OR".into();
     }
}

pub(crate) fn get_end_value(value: &str, field_type: &str) -> String {
     let value = value.to_string();

     let end_value: String;

     if field_type == "String" {
          end_value = format!("'{}'", value);
     }
     else {
         end_value = value;
     }

     return end_value;
}

pub(crate) fn push_str_to_query_string(
     query_string: &mut String, 
     field_name: &str, 
     end_value: &str, 
     last_item: bool, 
     sql_operator: &str, 
     next: Option<&Next>) {
     if !last_item {
          let next = next.unwrap();
          let continuation = format!("{} {} {} {} ", field_name, sql_operator, end_value, get_next(next));
          query_string.push_str(continuation.as_str());
     } else {
          let continuation = format!("{} {} {}", field_name, sql_operator, end_value);
          query_string.push_str(continuation.as_str());
     }
}

pub(crate) fn push_contains_to_query_string(
     query_string: &mut String, 
     field_name: &str, 
     end_value: &str, 
     last_item: bool, 
     next: Option<&Next>) {
     let end_value = end_value.to_string();

     let like_end_value = format!("'%{}%'", end_value.replace("'", "''"));

     if !last_item {
          let next = next.unwrap();
          let continuation = format!("{} LIKE {} {} ", field_name, like_end_value, get_next(next));
          query_string.push_str(continuation.as_str());
     } else {
          let continuation = format!("{} LIKE {}", field_name, like_end_value);
          query_string.push_str(continuation.as_str());
     }
}

pub(crate) fn push_starts_with_to_query_string(
     query_string: &mut String, 
     field_name: &str, 
     end_value: &str, 
     last_item: bool, 
     next: Option<&Next>) {
     let end_value = end_value.to_string();

     let like_end_value = format!("'{}%'", end_value.replace("'", "''"));

     if !last_item {
          let next = next.unwrap();
          let continuation = format!("{} LIKE {} {} ", field_name, like_end_value, get_next(next));
          query_string.push_str(continuation.as_str());
     } else {
          let continuation = format!("{} LIKE {}", field_name, like_end_value);
          query_string.push_str(continuation.as_str());
     }
}

pub(crate) fn push_ends_with_to_query_string(
     query_string: &mut String, 
     field_name: &str, 
     end_value: &str, 
     last_item: bool, 
     next: Option<&Next>) {
     let end_value = end_value.to_string();

     let like_end_value = format!("'%{}'", end_value.replace("'", "''"));

     if !last_item {
          let next = next.unwrap();
          let continuation = format!("{} LIKE {} {} ", field_name, like_end_value, get_next(next));
          query_string.push_str(continuation.as_str());
     } else {
          let continuation = format!("{} LIKE {}", field_name, like_end_value);
          query_string.push_str(continuation.as_str());
     }
}

pub(crate) fn query_match_operators(
     operator: &Operator, 
     query_string: &mut String, 
     field_name: &str, 
     field_type: &str, 
     last_item: bool, 
     next: Option<&Next>) {
     match operator {
          Operator::Eq(value) => {
               let end_value = get_end_value(&value, field_type);
               push_str_to_query_string(
                    query_string, 
                    field_name,
                    &end_value,
                    last_item, 
                    "==",
                    next
               );
          },
          Operator::Ne(value) => {
               let end_value = get_end_value(&value, field_type);
               push_str_to_query_string(
                    query_string, 
                    field_name,
                    &end_value,
                    last_item, 
                    "<>",
                    next
               );
          },
          Operator::Gt(value) => {
               let end_value = get_end_value(&value, field_type);
               push_str_to_query_string(
                    query_string, 
                    field_name,
                    &end_value,
                    last_item, 
                    ">",
                    next
               );
          },
          Operator::Lt(value) => {
               let end_value = get_end_value(&value, field_type);
               push_str_to_query_string(
                    query_string, 
                    field_name,
                    &end_value,
                    last_item, 
                    "<",
                    next
               );
          },
          Operator::Ge(value) => {
               let end_value = get_end_value(&value, field_type);
               push_str_to_query_string(
                    query_string, 
                    field_name,
                    &end_value,
                    last_item, 
                    ">=",
                    next
               );
          },
          Operator::Le(value) => {
               let end_value = get_end_value(&value, field_type);
               push_str_to_query_string(
                    query_string, 
                    field_name,
                    &end_value,
                    last_item, 
                    "<=",
                    next
               );
          },
          Operator::Contains(value) => {
               push_contains_to_query_string(
                    query_string, 
                    field_name,
                    &value,
                    last_item,
                    next
               );
          },
          Operator::StartsWith(value) => {
               push_starts_with_to_query_string(
                    query_string, 
                    field_name,
                    &value,
                    last_item,
                    next
               );
          },
          Operator::EndsWith(value) => {
               push_ends_with_to_query_string(
                    query_string, 
                    field_name,
                    &value,
                    last_item,
                    next
               );
          }
     }
}