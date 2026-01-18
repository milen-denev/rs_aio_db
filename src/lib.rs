/// # All in one aka Aio Database
/// ### Locally preserved database example
/// ```rust
/// //This will create a Test.db file at G:\ location
/// let file_db = AioDatabase::create::<Person>("G:\\".into(), "Test".into()).await;
/// ```
/// ### In-memory database example
/// ```rust
/// let in_memory_db = AioDatabase::create_in_memory::<Person>("Test".into()).await;
/// ```
/// #### Create a model
/// ```rust
/// use rs_aio_db::Reflect;
/// 
/// #[derive(Default, Clone, Debug, Reflect)]
/// struct Person {
///     name: String,
///     age: i32,
///     height: i32,
///     married: bool,
/// }
/// ```
/// 
/// #### For Inserting values:
/// ```rust
/// file_db.insert_value(Person {
///    name: "Mylo".into(),
///    age: 0,
///    height: 0,
///    married: true
/// }).await;
/// ```
/// 
/// #### For getting existing values / records:
/// ```rust
/// let get_record = file_db
///    .query()
///    .field("age")
///    .where_is(Operator::Gt(5.to_string()), Some(Next::Or))
///    .field("name")
///    .where_is(Operator::Eq("Mylo".into()), None)
///    .get_many_values::<Person>().await;
/// ```
/// 
/// #### Update existing values / records:
/// ```rust
/// let update_rows = file_db
///    .query()
///    .field("age")
///    .where_is(Operator::Eq((0).to_string()), Some(Next::Or))
///    .update_value(Person {
///        name: "Mylo".into(),
///        age: 5,
///        height: 5,
///        married: false
///    }).await;
/// ```
/// 
/// #### Deleting existing values / records:
/// ```rust
/// let delete_rows = file_db
///    .query()
///    .field("name")
///    .where_is(Operator::Eq("Mylo".into()), None)
///    .delete_value::<Person>().await;
/// ```
pub mod db;
pub use bevy_reflect::Reflect;
pub use serde::{Serialize, Deserialize};