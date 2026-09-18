use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use tokio::time::Instant;

pub async fn handle(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let url = req.uri().clone();
    let start = Instant::now();

    let response = next.run(req).await;

    let elapsed = start.elapsed();
    println!("Request [{} {}] {:?}", method, url, elapsed);

    response
}