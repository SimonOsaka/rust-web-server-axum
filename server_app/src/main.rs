use server_lib::start;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    start().await;
}
