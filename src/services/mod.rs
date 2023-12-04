use mongodb::{
    bson::{doc, Document},
    error::Error,
    options::{ClientOptions, FindOneOptions, FindOptions, ResolverConfig, UpdateOptions},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use tokio_stream::StreamExt;

use self::user_service::{User, UserService};

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

    pub async fn find_one(
        &self,
        filter: Document,
        opts: Option<FindOneOptions>,
    ) -> Result<Option<T>, Error>
    where
        T: serde::de::DeserializeOwned + Sync + Send + Unpin,
    {
        self.collection.find_one(filter, opts).await
    }

    pub async fn upsert(&self, filter: Document, update: Document) -> Result<(), Error>
    where
        T: serde::de::DeserializeOwned + Sync + Send + Unpin + serde::Serialize,
    {
        let options = UpdateOptions::builder().upsert(true).build();
        self.collection
            .update_one(filter, update, Some(options))
            .await?;

        Ok(())
    }
}

pub struct ServiceContainer {
    pub user_service: UserService,
}

pub async fn init_services() -> ServiceContainer {
    let client_uri = std::env::var("MONGODB_URI").expect("Missing MONGODB_URI in .env");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await
            .unwrap();

    let client = Client::with_options(options).unwrap();

    let user_collection = client.database("kogan").collection::<User>("users");

    ServiceContainer {
        user_service: user_service::UserService::new(user_collection),
    }
}
