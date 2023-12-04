use crate::services::BaseService;
use mongodb::{
    bson::{doc, Document},
    error::Error,
    Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    #[serde(
        rename(serialize = "_id", deserialize = "_id"),
        skip_serializing_if = "String::is_empty"
    )]
    pub id: String,
    pub animes: Vec<String>,
}

impl User {
    pub fn new(id: String, animes: Vec<String>) -> User {
        User { id, animes }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpsertUserReq {
    pub id: String,
    pub animes: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUsersReq {
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteUserReq {
    pub id: String,
}

#[derive(Clone)]
pub struct UserService {
    base: BaseService<User>,
}

impl UserService {
    pub fn new(collection: Collection<User>) -> UserService {
        UserService {
            base: BaseService::new(collection),
        }
    }

    // pub async fn create_user(&self, req: UpsertUserReq) -> Result<User, Error> {
    //     match self.base.insert(user).await {
    //         Ok(_) => Ok(user),
    //         Err(e) => Err(e),
    //     }
    // }

    pub async fn upsert_user(&self, req: UpsertUserReq) -> Result<(), Error> {
        let filter = doc! {
            "_id": req.id.clone()
        };
        let mut req_doc = Document::new();
        req_doc.insert("_id", req.id);
        req_doc.insert("animes", req.animes);

        let mut update = Document::new();
        update.insert("$set", req_doc);

        self.base.upsert(filter, update).await
    }

    pub async fn get_user(&self, id: String) -> Result<Option<User>, Error> {
        let filter = doc! {
            "_id": id
        };

        match self.base.find_one(filter, None).await {
            Ok(user) => Ok(user),
            Err(e) => Err(e),
        }
    }

    // pub async fn search_users(
    //     &self,
    //     req: SearchUsersReq,
    //     options: Option<FindOptions>,
    // ) -> Result<Vec<User>, Error> {
    //     let mut filter = Document::new();
    //     if let Some(name) = req.name {
    //         filter.insert("name", name);
    //     }
    //     if let Some(email) = req.email {
    //         filter.insert("email", email);
    //     }

    //     match self.base.find(filter, options).await {
    //         Ok(users) => Ok(users),
    //         Err(e) => Err(e),
    //     }
    // }

    // pub async fn update_user(&self, req: UpdateUserReq) -> Result<User, Error> {
    //     let filter = doc! {
    //         "_id": req.id.clone()
    //     };
    //     let mut update = Document::new();
    //     if let Some(name) = req.name {
    //         update.insert("name", name);
    //     }
    //     if let Some(email) = req.email {
    //         update.insert("email", email);
    //     }
    //     if let Some(password) = req.password {
    //         update.insert("password", password);
    //     }

    //     match self.base.update(filter.clone(), update).await {
    //         Ok(_) => match self.base.find(filter.clone(), None).await {
    //             Ok(users) => Ok(users[0].copy()),
    //             Err(e) => Err(e),
    //         },
    //         Err(e) => Err(e),
    //     }
    // }

    pub async fn delete_user(&self, req: DeleteUserReq) -> Result<User, Error> {
        let mut filter = Document::new();
        filter.insert("_id", req.id.clone());
        let user = match self.base.find(filter.clone(), None).await {
            Ok(users) => users[0].clone(),
            Err(e) => return Err(e),
        };

        match self.base.delete(filter).await {
            Ok(_) => Ok(user),
            Err(e) => Err(e),
        }
    }
}
