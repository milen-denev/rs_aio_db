use bevy_reflect::{Struct, ReflectRef};
use libsql::Connection;
use libsql::Builder;

pub struct AioDatabase {
     name: String,
     conn: Connection
}

impl AioDatabase {
     pub async fn create<T:  Default + Struct>(location: String, name: String) -> AioDatabase {
          let db_location = format!("{}{}{}", location, get_system_char_delimiter(), name);
          let builder = Builder::new_local(db_location).build().await.unwrap();
          let connection = builder.connect().unwrap();
          
          let default_t = T::default();
          let default_t2 = T::default();
          let my_struct: Box<dyn Struct> = Box::new(default_t);

          let ReflectRef::Struct(reflected) = default_t2.reflect_ref() else { unreachable!() };

          for (i, field) in my_struct.iter_fields().enumerate() {
               let name = reflected.name_at(i).unwrap();
               println!("{}", name);
               println!("{}", field.reflect_type_ident().unwrap());
          }

          AioDatabase {
               name: name,
               conn: connection
          }
     }

     pub fn get_name(&self) -> &str {
          return self.name.as_str();
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
