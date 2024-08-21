use bevy_reflect::Reflect;

#[derive(Default, Clone, Debug)]
pub struct Schema {
     pub field_name: String,
     pub field_type: String
}

unsafe impl Send for Schema { }

#[derive(Debug)]
pub struct GenericValue<'a> {
     pub field_name: String,
     pub field_value: &'a dyn Reflect,
     pub field_type: String
}

unsafe impl<'a> Send for GenericValue<'a> { }