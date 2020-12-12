use warp::Filter;
#[tokio::main]
async fn main() {
    let web = warp::get()
        .and(warp::fs::dir("/Users/atmaram/Documents/Projects/tech/rust/tambola/dist"));
    let site=web;

    let shutdown = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };
    let (_, serving) =
        warp::serve(site).bind_with_graceful_shutdown(([0, 0, 0, 0], 3030), shutdown);


    tokio::select! {
        _ = serving => {}
    }
}