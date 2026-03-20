use std::path::{Path, PathBuf};

use rocket::{form::Form, fs::NamedFile, futures::StreamExt, response::Redirect};
use rocket_db_pools::{Connection, sqlx};
use rocket_dyn_templates::{Template, context};
use sqlx::{Row, postgres::PgRow};

use crate::model;

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {
        title: "Feed",
    })
}

#[get("/static/<file..>")]
pub async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[get("/post/<id>")]
pub async fn details(id: i32, mut db: Connection<model::DB>) -> Option<Template> {
    let post: model::Post = sqlx::query_as::<_, model::Post>("SELECT * FROM posts WHERE id = $1").bind(id).fetch_one(&mut **db).await.ok()?;

    Some(Template::render("details", context! {
        title: post.title.clone(),
        post,
    }))

}

#[get("/create")]
pub async fn create_get() -> Template {
    Template::render("create", context! {
        title: "New post",
    })
}

#[post("/create", data = "<post>")]
pub async fn create_post(post: Form<model::Post>, mut db: Connection<model::DB>) -> Option<Redirect> {
    let result = sqlx::query_as::<_, model::Post>("INSERT INTO posts (title, text) VALUES ($1, $2) RETURNING *").bind(post.title.clone()).bind(post.text.clone()).fetch_one(&mut **db).await.ok()?;
    Some(Redirect::to(uri!(details(result.id?))))
}

fn map_row(row: PgRow) -> model::PostPointer {
        let id = row.get("id");
        model::PostPointer {
            id,
            title: row.get("title"),
            url: uri!(details(id)).to_string(),
        }
    }

#[get("/feed")]
pub async fn feed(mut db: Connection<model::DB>) -> Option<Template> {
    let posts = sqlx::query("SELECT id, title FROM posts").map(map_row).fetch_all(&mut **db).await.ok()?;
    println!("{:?}", posts);
    Some(Template::render("feed", context! {
        title: "Posts",
        posts,
    }))
}
