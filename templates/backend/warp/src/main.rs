use warp::Filter;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let routes = warp::path::end().map(||
        "Hello ".to_owned() + &Uuid::new_v4().to_string()
    );

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}
