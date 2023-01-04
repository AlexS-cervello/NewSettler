use std::fs::OpenOptions;

use crate::entity::users;
use crate::migration::{DbErr, Migrator, MigratorTrait};
use directories::BaseDirs;
use sea_orm::{
    ColumnTrait, Database as SeaOrmDatabase, DatabaseConnection, EntityTrait,
    NotSet, QueryFilter, Set,
};
use teloxide::types::ChatId;

#[derive(Debug)]
pub enum Error {
    Database(DbErr),
    File(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Database(ref err) => {
                write!(f, "Database error: {}", err)
            }
            Self::File(ref err) => {
                write!(f, "File error: {}", err)
            }
        }
    }
}

impl From<DbErr> for Error {
    fn from(err: DbErr) -> Self {
        Self::Database(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::File(err)
    }
}

async fn get_db_pool() -> Result<DatabaseConnection, Error> {
    let base_dirs = BaseDirs::new();
    let db_name = "settler_db.sqlite";
    let db_path = std::env::var_os("SETTLER_DB")
        .map(Into::into)
        .unwrap_or_else(|| {
            if std::env::consts::OS != "android" {
                base_dirs
                    .map(|x| x.data_dir().join(db_name))
                    .unwrap_or_else(|| db_name.into())
            } else {
                db_name.into()
            }
        });
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&db_path)?;
    let db_str = format!("sqlite:{}", db_path.display());
    let pool = SeaOrmDatabase::connect(&db_str).await?;
    Ok(pool)
}

#[derive(Clone)]
pub struct Database {
    pool: DatabaseConnection,
}

impl Database {
    pub async fn new() -> Result<Self, Error> {
        get_db_pool().await.map(|pool| Self { pool })
    }
    pub async fn apply_migrations(&self) -> Result<(), Error> {
        Ok(Migrator::up(&self.pool, None).await?)
    }
    pub async fn insert_user(&self, chat_id: &ChatId) -> Result<(), Error> {
        let user = users::Entity::find()
            .filter(users::Column::ChatId.eq(chat_id.0))
            .one(&self.pool)
            .await?;
        if let None = user {
            let user = users::ActiveModel {
                id: NotSet,
                chat_id: Set(chat_id.0),
                is_active: Set(true),
            };
            users::Entity::insert(user).exec(&self.pool).await?;
        }
        Ok(())
    }
    pub async fn get_users_id(&self) -> Result<Vec<i64>, Error> {
        let users = users::Entity::find()
            .filter(users::Column::IsActive.eq(true))
            .all(&self.pool)
            .await?;
        let user_ids: Vec<i64> =
            users.into_iter().map(|user| user.chat_id).collect();
        Ok(user_ids)
    }
}
