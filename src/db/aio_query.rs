use std::sync::Arc;

use libsql::{Connection, Row};

pub struct QueryBuilder {
     
}

pub struct Query {
     pub final_query_str: String
}

#[derive(Clone)]
pub struct QueryRowResult<T> {
     pub value: Option<T>,
     pub row: Arc<Row>
}

impl<T> QueryRowResult<T> {
     pub async fn new(
          query: String, 
          table_name: &str,
          connection: &Connection) -> QueryRowResult<T> {

          let query = format!("SELECT * FROM {table_name} WHERE test2 = 16");

          let row = connection
               .query(&query, ())
               .await
               .unwrap()
               .next()
               .await
               .unwrap()
               .unwrap();

          return QueryRowResult::<T> {
               value: None,
               row: Arc::new(row)
          }
     }
}