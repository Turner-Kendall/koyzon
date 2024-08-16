mod handler;
mod model;
mod response;

use model::{QueryOptions, DB};
use warp::{http::Method, Filter, Rejection};

type WebResult<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }

    pretty_env_logger::init();
    
    let db = model::koyzon_db();

    let file_router = warp::path!("api" / "file");

    let koyzon_router = warp::path!("api" / "tasks");
    let koyzon_router_id = warp::path!("api" / "tasks" / String);

    let ping = warp::path!("api" / "ping")
        .and(warp::get())
        .and_then(handler::ping);

    // you don't have to use the route - this works without the api path
    let test = warp::path!( "test")
        .and(warp::get())
        .and_then(handler::test);

    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origins(vec!["http://localhost:3000/", "http://localhost:8000/"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);

    let file_route = file_router
        .and(warp::post())
        // .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::file_save_handler);
        // .or(file_router
        //     .and(warp::get())
        //     .and(warp::query::<QueryOptions>())
        //     .and(with_db(db.clone()))
        //     .and_then(handler::tasks_list_handler));

    let koyzon_route = koyzon_router
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_task_handler)
        .or(koyzon_router
            .and(warp::get())
            .and(warp::query::<QueryOptions>())
            .and(with_db(db.clone()))
            .and_then(handler::tasks_list_handler));

    let koyzon_route_id = koyzon_router_id
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::edit_task_handler)
        .or(koyzon_router_id
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::get_task_handler))
        .or(koyzon_router_id
            .and(warp::delete())
            .and(with_db(db.clone()))
            .and_then(handler::delete_task_handler));

    let routes = koyzon_route
        .with(cors)
        .with(warp::log("api"))
        .or(koyzon_route_id)
        .or(test)
        .or(ping);

    println!("🚀 Server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;

}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
