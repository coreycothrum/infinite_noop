#[tokio::main(flavor = "current_thread")]
async fn main() {
    loop {
        let () = futures::future::pending().await;
        unreachable!();
    }
}
