use libsql::Connection;

#[derive(Default, Clone, Debug)]
pub struct Schema {
     pub field_name: String,
     pub field_type: String
}
