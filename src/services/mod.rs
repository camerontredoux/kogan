use mongodb::{
    bson::{doc, Bson, Document},
    error::Error,
    options::{FindOneAndUpdateOptions, FindOptions},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};
use tokio_stream::StreamExt;

use self::user_service::UserService;

pub mod user_service;

#[derive(Clone)]
pub struct BaseService<T> {
    collection: Collection<T>,
}

impl<T> BaseService<T> {
    pub fn new(collection: Collection<T>) -> BaseService<T> {
        BaseService { collection }
    }

    pub async fn insert(&self, document: T) -> Result<InsertOneResult, Error>
    where
        T: serde::Serialize,
    {
        self.collection.insert_one(document, None).await
    }

    pub async fn update(&self, filter: Document, update: Document) -> Result<UpdateResult, Error>
    where
        T: serde::Serialize,
    {
        let set = doc! {
            "$set": update,
        };
        self.collection.update_one(filter, set, None).await
    }

    pub async fn delete(&self, filter: Document) -> Result<DeleteResult, Error> {
        self.collection.delete_one(filter, None).await
    }

    pub async fn find(&self, filter: Document, opts: Option<FindOptions>) -> Result<Vec<T>, Error>
    where
        T: serde::de::DeserializeOwned + Sync + Send + Unpin,
    {
        let cursor = self.collection.find(filter, opts).await.unwrap();
        cursor.collect().await
    }

    pub async fn upsert(&self, filter: Document, update: Document) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + Sync + Send + Unpin + serde::Serialize,
    {
        let options = FindOneAndUpdateOptions::builder()
            .upsert(Some(true))
            .build();

        let user = self
            .collection
            .find_one_and_update(filter, update, options)
            .await
            .ok()
            .unwrap()
            .unwrap();

        Ok(user)
    }
}

pub struct ServiceContainer {
    pub user_service: UserService,
}
