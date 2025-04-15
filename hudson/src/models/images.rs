pub use super::_entities::images::{self, ActiveModel, Entity, Model};
use loco_rs::model::{ModelError, ModelResult};
use sea_orm::entity::prelude::*;
use serde::Serialize;
use validator::Validate;
pub type Images = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

#[derive(Debug, Validate, Serialize)]
pub struct Validator {
    #[validate(url)]
    pub url: String,
}

// implement your read-oriented logic here
impl Model {
    /// Find an image by its ID
    ///
    /// # Errors
    ///
    /// Returns an error if the image is not found or there is a database error
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let image = Entity::find_by_id(id).one(db).await?;
        image.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Find all images by a user's ID
    ///
    /// # Errors
    ///
    /// Returns an error if there is a database error
    pub async fn find_by_user_id(db: &DatabaseConnection, user_id: i32) -> ModelResult<Vec<Self>> {
        let images = Entity::find()
            .filter(images::Column::UserId.eq(user_id))
            .all(db)
            .await?;
        Ok(images)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
