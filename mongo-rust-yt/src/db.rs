use std::str::FromStr;

use crate::{error::Error::*, handler::BookRequest, Book, Result};
use chrono::prelude::*;
use futures::StreamExt;

use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
// use mongodb::Cursor;
use mongodb::{options::ClientOptions, Client, Collection};

const DB_NAME: &str = "booky";
const COLL: &str = "books";

const NAME: &str = "name";
const AUTHOR: &str = "author";
const NUM_PAGES: &str = "num_pages";
const ADDED_AT: &str = "added_at";
const TAGS: &str = "tags";

#[derive(Debug, Clone)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options =
            ClientOptions::parse("mongodb://root:example@localhost:27017").await?;
        client_options.app_name = Some("booky".to_string());

        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }
    fn get_collection(&self) -> Collection<Book> {
        self.client.database(DB_NAME).collection(COLL)
    }
    pub async fn create_book(&self, entry: &BookRequest) -> Result<()> {
        let doc = Book {
            name: entry.name.clone(),
            author: entry.author.clone(),
            num_pages: entry.num_pages,
            added_at: Utc::now(),
            tags: entry.tags.clone(),
        };
        self.get_collection()
            .insert_one(doc)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn delete_book(&self, id: &str) -> Result<()> {
        let oid = ObjectId::from_str(id).unwrap().to_string();
        let filter = doc! {
            "_id": oid,
        };
        self.get_collection()
            .delete_one(filter)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn edit_book(&self, id: &str, entry: &BookRequest) -> Result<()> {
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()));
        let filter = doc! {
            "_id": oid?,
        };
        let update = doc! {
            "name": entry.name.clone().to_string(),
            "author": entry.author.clone().to_string(),
            "num_pages": entry.num_pages.clone().to_string(),
            "added_at": Utc::now().to_string(),
            "tags": entry.tags.clone(),
        };

        self.get_collection()
            .update_one(filter, update)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    fn doc_to_book(&self, doc: &Document) -> Result<Book> {
        let name = doc.get_str(NAME)?;
        let author = doc.get_str(AUTHOR)?;
        let num_pages = doc.get_i32(NUM_PAGES)?;
        let added_at = doc.get_datetime(ADDED_AT)?;
        let tags = doc.get_array(TAGS)?;

        let book = Book {
            name: name.to_owned(),
            author: author.to_owned(),
            num_pages: num_pages as usize,
            added_at: chrono::DateTime::from_timestamp_millis(added_at.timestamp_millis()).unwrap(),
            tags: tags
                .iter()
                .filter_map(|entry| match entry {
                    Bson::String(v) => Some(v.to_owned()),
                    _ => None,
                })
                .collect(),
        };
        Ok(book)
    }

    pub async fn fetch_books(&self) -> Result<Vec<Book>> {
        // let filter = mongodb::bson::Document::new();
        let mut cursor = self
            .get_collection()
            .find(doc! {})
            .await
            .map_err(MongoQueryError)?;
        let mut result: Vec<Book> = Vec::new();
        while let Some(doc) = cursor.next().await {
            // let mut v = Vec::new();
            let book = doc?;
            // let doc_new = Document::from_reader(std::io::Cursor::new(book));
            let mut doc_new = Document::new();
            doc_new.insert("name", book.name.clone());
            doc_new.insert("author", book.author.clone());
            doc_new.insert("num_pages", book.num_pages.clone().to_string());
            doc_new.insert("added_at", book.added_at.clone().to_string());
            doc_new.insert("tags", book.tags.clone());
            result.push(self.doc_to_book(&doc_new)?);
        }
        Ok(result)
    }
}
