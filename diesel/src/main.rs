#[macro_use]
extern crate diesel;

use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// Find users by userid. Returning the first match?
#[get("/user/{user_id}")]
async fn get_user(pool: web::Data<DbPool>, uid: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let user_id = uid.into_inner();

    let conn = pool.get().expect("Failed to establish db connection");

    // As diesel does not support tokio, spawning a new thread and doing the blocking diesel call in it, so as to not block the server thread.
    // This is a handler registered in the httpserver application context. It will be running on a server thread.
    // web::block - execute blocking function on a thread pool, returns future that resolves to result on fn execution.
    let user = web::block(move || actions::find_user_by_id(user_id, &conn))
        .await
        .map_err(|err| { println!("Error finding user with uid -> {:?}", err);
                         HttpResponse::InternalServerError().finish()})?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::NotFound().body(format!("No user found with uid: {}", user_id)))
    }
}

// Insert new user into the database
#[post("/user")]
async fn add_user(pool: web::Data<DbPool>, user: web::Json<models::NewUser>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Error connecting to database");
    let user = web::block(move || actions::insert_new_user(&user.name, &conn))
        .await
        .map_err(|err| {
            println!("Error adding new user -> {:?}", err);
            HttpResponse::InternalServerError().finish()
         })?;

    Ok(HttpResponse::Ok().json(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv::dotenv().ok();

    // set up a database connection pool
    let conn = std::env::var("DATABASE_URL").expect("DATABASE_URL missing..");
    let manager = ConnectionManager::<SqliteConnection>::new(conn);
    let pool = r2d2::Pool::builder()
                           .build(manager)
                           .expect("Failed to create r2d2 pool manager");

    let bind = "127.0.0.1:8080";

    println!("Starting test server at : {}", &bind);

    //Actix::HttpServer
    HttpServer::new(move || {
        App::new().data(pool.clone())
                  .wrap(middleware::Logger::default())
                  .service(get_user)
                  .service(add_user)
    })
    .bind(&bind)?
    //.bind("127.0.0.1:7878")?
    .run()
    .await
}
