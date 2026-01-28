use core::error;
use std::mem;

use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};

use crate::db::models::sessions::{self};

pub struct Db {
    database: DatabaseConnection,
}

impl Drop for Db {
    fn drop(&mut self) {
        let db = mem::take(&mut self.database);
        tokio::spawn(async move {
            let _ = db.close().await;
        });
    }
}

impl Db {
    pub async fn new(connection: String) -> Result<Self, Box<dyn error::Error>> {
        let db = sea_orm::Database::connect(connection).await.map_err(|e| {
            panic!("Failed to connect to database: {}", e);
        })?;

        match db.ping().await {
            Ok(_) => println!("[db] database reachable"),
            Err(e) => {
                if let Err(close_err) = db.close().await {
                    eprintln!("[db] failed to close database: {}", close_err);
                }
                return Err(format!("[db] could not reach database: {}", e).into());
            }
        }

        Ok(Db { database: db })
    }

    pub async fn insert(&self, model: sessions::ActiveModel) -> Result<sessions::Model, DbErr> {
        model.insert(&self.database).await
    }
}
