use bevy_reflect::{DynamicStruct, GetField, Reflect, ReflectMut, ReflectRef, Struct};
use log::{debug, info};

use crate::db::{aio_query::QueryRowResult, models::{GenericValue, Schema}};

pub fn get_system_char_delimiter() -> &'static str {
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

pub fn get_schema_from_generic<T:  Default + Struct>() -> Vec<Schema> {  
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
     
     return schema_vec;
}

pub fn get_values_from_generic<'a, T:  Default + Struct + Clone>(value: &'a T) -> Vec<GenericValue> {  
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

pub fn set_values_from_row_result<'a, T:  Default + Struct + Clone>(row_result: &QueryRowResult<T>) -> T {  
     let mut struct_mut2: Box<dyn Struct> = Box::new(T::default());
     let ReflectMut::Struct(reflected2) = struct_mut2.reflect_mut() else { unreachable!() };
     let struct_immutable: Box<dyn Struct> = Box::new(T::default());
     
     let mut test = T::default();

     for (i, field) in struct_immutable.iter_fields().enumerate() {
          let field_type = field.reflect_type_ident().unwrap();
          let field_name = reflected2.name_at(i).clone().unwrap();

          match field_type {
               "bool" => {
                    *test.get_field_mut::<bool>(field_name).unwrap() = row_result.row.get::<bool>(i as i32).unwrap();
               },
               "u8" => {
                    *test.get_field_mut::<u8>(field_name).unwrap() = row_result.row.get::<u32>(i as i32).unwrap() as u8;
               },
               "u16" => {
                    *test.get_field_mut::<u16>(field_name).unwrap() = row_result.row.get::<u32>(i as i32).unwrap() as u16;
               },
               "u32" => {
                    *test.get_field_mut::<u32>(field_name).unwrap() = row_result.row.get::<u32>(i as i32).unwrap() as u32;
               },
               "u64" => {
                    *test.get_field_mut::<u64>(field_name).unwrap() = row_result.row.get::<u32>(i as i32).unwrap() as u64;
               },
               "u128" => {
                    *test.get_field_mut::<u128>(field_name).unwrap() = row_result.row.get::<u32>(i as i32).unwrap() as u128;
               },
               "i8" => {
                    *test.get_field_mut::<i8>(field_name).unwrap() = row_result.row.get::<i32>(i as i32).unwrap() as i8;
               },
               "i16" => {
                    *test.get_field_mut::<i16>(field_name).unwrap() = row_result.row.get::<i32>(i as i32).unwrap() as i16;
               },
               "i32" => {
                    *test.get_field_mut::<i32>(field_name).unwrap() = row_result.row.get::<i32>(i as i32).unwrap() as i32;
               },
               "i64" => {
                    *test.get_field_mut::<i64>(field_name).unwrap() = row_result.row.get::<i32>(i as i32).unwrap() as i64;
               },
               "i128" => {
                    *test.get_field_mut::<i128>(field_name).unwrap() = row_result.row.get::<i32>(i as i32).unwrap() as i128;
               },
               "f32" => {
                    *test.get_field_mut::<f32>(field_name).unwrap() = row_result.row.get::<f64>(i as i32).unwrap() as f32;
               },
               "f64" => {
                    *test.get_field_mut::<f64>(field_name).unwrap() = row_result.row.get::<f64>(i as i32).unwrap() as f64;
               },
               "char" => {
                    *test.get_field_mut::<char>(field_name).unwrap() = row_result.row.get::<String>(i as i32).unwrap().pop().unwrap() as char;
               },
               "String" => {
                   
                    let value = row_result.row.get::<String>(i as i32).unwrap();
                    *test.get_field_mut::<String>(field_name).unwrap() = value;
               },
               _ => panic!("{} type not supported.", field_type)
          }
     }

     return test;
}