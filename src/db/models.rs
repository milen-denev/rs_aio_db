use bevy_reflect::PartialReflect;

#[derive(Default, Clone, Debug)]
pub struct Schema {
     pub field_name: String,
     pub field_type: String
}

#[derive(Debug)]
pub struct GenericValue<'a> {
     pub field_name: String,
     pub field_value: &'a dyn PartialReflect,
     pub field_type: String
}