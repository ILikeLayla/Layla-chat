use super::Table;
use sqlx::{MySql, Pool};

pub struct DataBase {
    name: String,
    tables: Vec<Table>,
    connection: Pool<MySql> 
}

// impl DataBase {
//     pub fn new() -> Result<Self, sqlx::Error> {

//     }
// }