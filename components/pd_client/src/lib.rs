use anyhow::Result;
use tikv_client::Timestamp;
use tikv_client::TransactionClient;

pub fn current_timestamp(pd: &str) -> Result<Timestamp> {
    let tso_worker = tokio::runtime::Builder::new_multi_thread()
        .thread_name("tso-worker")
        .enable_io()
        .enable_time()
        .worker_threads(1)
        .build();
    let (tx, mut rx) = tokio::sync::oneshot::channel();

    tso_worker.unwrap().block_on(async {
        let client = TransactionClient::new(vec![pd]).await.unwrap();
        println!("{}", 456);
        let timestamp = client.current_timestamp().await.unwrap();
        println!("{:#?}", timestamp.clone());
        println!("{}", 123);
        tx.send(timestamp)
    });

    let tso = rx.try_recv();
    Ok(tso?)
}
