use bevy_reflect::{ReflectRef, Struct};
use log::{debug, info};

use crate::db::models::{GenericValue, Schema};

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