use std::sync::Arc;

use libsql::{Connection, Row};

//let query = format!("SELECT * FROM {table_name} WHERE test2 = 16");

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
          connection: &Connection) -> Option<QueryRowResult<T>> { 
          let row_result = connection
               .query(&query, ())
               .await
               .unwrap()
               .next()
               .await;
               
          if let Ok(row_option) = row_result {
               if let Some(row) = row_option {
                    return Some(QueryRowResult::<T> {
                         value: None,
                         row: Arc::new(row)
                    });
               }
               else {
                    return None;
               }
          }
          else {
               return None;
          }
     }
}