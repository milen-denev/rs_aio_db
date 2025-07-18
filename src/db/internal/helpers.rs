use bevy_reflect::{GetField, ReflectMut, ReflectRef, Struct};
use log::debug;
use crate::db::{aio_query::{Next, Operator, QueryRowResult, QueryRowsResult}, models::{GenericValue, Schema}};

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
          debug!("Found field named '{}' of type '{}'", field_name, field_type);

          schema_vec.push(Schema {
               field_name: field_name.into(),
               field_type: field_type.into()
          });
     }
     
     return schema_vec;
}

pub(crate) fn get_values_from_generic<'a, T:  Default + Struct + Clone>(value: &'a T) -> Vec<GenericValue<'a>> {  
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

pub(crate) fn set_values_from_row_result<'a, T:  Default + Struct + Clone>(row_result: &mut QueryRowResult<T>) -> Result<T, ()> {  
     let mut struct_mut2: Box<dyn Struct> = Box::new(T::default());
     let ReflectMut::Struct(reflected2) = struct_mut2.reflect_mut() else { unreachable!() };
     let struct_immutable: Box<dyn Struct> = Box::new(T::default());
     
     let mut t_struct = T::default();

     if let Some(row) = row_result.value.as_ref() {
          for (i, field) in struct_immutable.iter_fields().enumerate() {
               let field_type = field.reflect_type_ident().unwrap();
               let field_name = reflected2.name_at(i).clone().unwrap();

               match field_type {
                    "bool" => {
                         *t_struct.get_field_mut::<bool>(field_name).unwrap() = match row.get_field::<bool>(field_name).unwrap_or(&false) {
                              false => false,
                              true => true
                         };
                    },
                    "u8" => {
                         *t_struct.get_field_mut::<u8>(field_name).unwrap() = *row.get_field::<u8>(field_name).unwrap_or(&0) as u8;
                    },
                    "u16" => {
                         *t_struct.get_field_mut::<u16>(field_name).unwrap() = *row.get_field::<u16>(field_name).unwrap_or(&0) as u16;
                    },
                    "u32" => {
                         *t_struct.get_field_mut::<u32>(field_name).unwrap() = *row.get_field::<u32>(field_name).unwrap_or(&0) as u32;
                    },
                    "u64" => {
                         *t_struct.get_field_mut::<u64>(field_name).unwrap() = *row.get_field::<u64>(field_name).unwrap_or(&0) as u64;
                    },
                    "i8" => {
                         *t_struct.get_field_mut::<i8>(field_name).unwrap() = *row.get_field::<i8>(field_name).unwrap_or(&0) as i8;
                    },
                    "i16" => {
                         *t_struct.get_field_mut::<i16>(field_name).unwrap() = *row.get_field::<i16>(field_name).unwrap_or(&0) as i16;
                    },
                    "i32" => {
                         *t_struct.get_field_mut::<i32>(field_name).unwrap() = *row.get_field::<i32>(field_name).unwrap_or(&0) as i32;
                    },
                    "i64" => {
                         *t_struct.get_field_mut::<i64>(field_name).unwrap() = *row.get_field::<i64>(field_name).unwrap_or(&0) as i64;
                    },
                    "f32" => {
                         *t_struct.get_field_mut::<f32>(field_name).unwrap() = *row.get_field::<f32>(field_name).unwrap_or(&0.00f32) as f32;
                    },
                    "f64" => {
                         *t_struct.get_field_mut::<f64>(field_name).unwrap() = *row.get_field::<f64>(field_name).unwrap_or(&0.00f64) as f64;
                    },
                    "char" => {
                         *t_struct.get_field_mut::<char>(field_name).unwrap() = *row.get_field::<char>(field_name).unwrap_or(&' ') as char;
                    },
                    "String" => {
                          let mut buffer: Vec<u8> = Vec::new();
                         buffer.extend_from_slice(row.get_field::<String>(field_name).unwrap_or(&"".into()).as_bytes());
                         *t_struct.get_field_mut::<String>(field_name).unwrap() = String::from_utf8_lossy(&buffer).into_owned();
                    },
                    "Vec" => {
                         let mut buffer: Vec<u8> = Vec::new();
                         buffer.extend_from_slice(row.get_field::<String>(field_name).unwrap_or(&"".into()).as_bytes());
                         //let vec_u8_data = hex::decode(value).unwrap();
                         *t_struct.get_field_mut::<Vec<u8>>(field_name).unwrap() = buffer;
                    },
                    _ => panic!("{} type not supported.", field_type)
               }
          }
     } else {
          return Err(());
     }

     return Ok(t_struct);
}

pub(crate) fn set_values_from_many_rows_result<'a, T:  Default + Struct + Clone>(row_result: &mut QueryRowsResult<T>) -> Result<Vec<T>, ()> {  
     let mut struct_mut2: Box<dyn Struct> = Box::new(T::default());
     let ReflectMut::Struct(reflected2) = struct_mut2.reflect_mut() else { unreachable!() };
     let struct_immutable: Box<dyn Struct> = Box::new(T::default());

     let mut vec_t: Vec<T> = Vec::default();

     if let Some(rows) = row_result.value.as_ref() {
          let count = rows.len();
          let mut i = 0;

          while i < count {
               if let Some(row) = rows.get(i) {
                    let row = row.as_ref().unwrap();

                    let mut t_struct = T::default();

                    for (i, field) in struct_immutable.iter_fields().enumerate() {
                         let field_type = field.reflect_type_ident().unwrap();
                         let field_name = reflected2.name_at(i).clone().unwrap();
          
                         match field_type {
                              "bool" => {
                                   *t_struct.get_field_mut::<bool>(field_name).unwrap() = match row.get_field::<bool>(field_name).unwrap_or(&false) {
                                        false => false,
                                        true => true
                                   };
                              },
                              "u8" => {
                                   *t_struct.get_field_mut::<u8>(field_name).unwrap() = *row.get_field::<u8>(field_name).unwrap_or(&0) as u8;
                              },
                              "u16" => {
                                   *t_struct.get_field_mut::<u16>(field_name).unwrap() = *row.get_field::<u16>(field_name).unwrap_or(&0) as u16;
                              },
                              "u32" => {
                                   *t_struct.get_field_mut::<u32>(field_name).unwrap() = *row.get_field::<u32>(field_name).unwrap_or(&0) as u32;
                              },
                              "u64" => {
                                   *t_struct.get_field_mut::<u64>(field_name).unwrap() = *row.get_field::<u64>(field_name).unwrap_or(&0) as u64;
                              },
                              "i8" => {
                                   *t_struct.get_field_mut::<i8>(field_name).unwrap() = *row.get_field::<i8>(field_name).unwrap_or(&0) as i8;
                              },
                              "i16" => {
                                   *t_struct.get_field_mut::<i16>(field_name).unwrap() = *row.get_field::<i16>(field_name).unwrap_or(&0) as i16;
                              },
                              "i32" => {
                                   *t_struct.get_field_mut::<i32>(field_name).unwrap() = *row.get_field::<i32>(field_name).unwrap_or(&0) as i32;
                              },
                              "i64" => {
                                   *t_struct.get_field_mut::<i64>(field_name).unwrap() = *row.get_field::<i64>(field_name).unwrap_or(&0) as i64;
                              },
                              "f32" => {
                                   *t_struct.get_field_mut::<f32>(field_name).unwrap() = *row.get_field::<f32>(field_name).unwrap_or(&0.00f32) as f32;
                              },
                              "f64" => {
                                   *t_struct.get_field_mut::<f64>(field_name).unwrap() = *row.get_field::<f64>(field_name).unwrap_or(&0.00f64) as f64;
                              },
                              "char" => {
                                   *t_struct.get_field_mut::<char>(field_name).unwrap() = *row.get_field::<char>(field_name).unwrap_or(&' ') as char;
                              },
                              "String" => {
                                   let mut buffer: Vec<u8> = Vec::new();
                                   buffer.extend_from_slice(row.get_field::<String>(field_name).unwrap_or(&"".into()).as_bytes());
                                   *t_struct.get_field_mut::<String>(field_name).unwrap() = String::from_utf8_lossy(&buffer).into_owned();
                              },
                              "Vec" => {
                                   let mut buffer: Vec<u8> = Vec::new();
                                   buffer.extend_from_slice(row.get_field::<String>(field_name).unwrap_or(&"".into()).as_bytes());
                                   //let vec_u8_data = hex::decode(value).unwrap();
                                   *t_struct.get_field_mut::<Vec<u8>>(field_name).unwrap() = buffer;
                              },
                              _ => panic!("{} type not supported.", field_type)
                         }
                    }
          
                    vec_t.push(t_struct);
               } else {
                    break;
               }

               i += 1;
          }

          return Ok(vec_t);
     }

     return Ok(vec![]); // Return an empty vector if no rows are found.
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