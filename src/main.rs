use std::path::Path;

use anyhow::Result;
use cmd::{build_cli, BackupInfo};
use futures::channel::mpsc as future_mpsc;
use futures::StreamExt;
use kvproto::{
    brpb::Error_oneof_detail,
    brpb::{BackupRequest, StorageBackend},
    brpb_grpc::BackupClient,
    kvrpcpb::ApiVersion,
};
use mbr::Range;
// use pd_client::;

// use tidb_query_datatype::codec::table;
// use tikv_client::Timestamp;

fn main() -> Result<()> {
    let cmd = build_cli();
    run_backup(cmd?);
    Ok(())
}

pub fn run_backup(_backup_cmd: BackupInfo) {
    let pd = "127.0.0.1:2379";
    let kv = "127.0.0.1:20160";
    let path = "/Users/suzhipeng/Desktop/dump";
    // let tso = pd_client::current_timestamp(pd);
    // println!("{:#?}", tso);
    let table_range = Range::new(70);

    backup_range(table_range, kv.to_owned(), path.to_owned());
}

pub fn make_local_backend(path: &Path) -> StorageBackend {
    let path = path.display().to_string();
    let mut backend = StorageBackend::default();
    backend.mut_local().set_path(path);
    backend
}

pub fn backup_range(range: Range, store_addr: String, external_storage: String) {
    let env = std::sync::Arc::new(grpcio::EnvBuilder::new().build());
    let ch = grpcio::ChannelBuilder::new(env).connect(&store_addr);
    let client = BackupClient::new(ch);

    let mut req = BackupRequest::default();

    req.set_cluster_id(7184112658150093742);
    req.set_rate_limit(1);
    req.set_start_key(range.start_key);
    req.set_end_key(range.end_key);
    req.set_dst_api_version(ApiVersion::V1);

    req.set_start_version(0);
    req.set_end_version(438500808740569089);
    req.set_concurrency(1);
    req.set_compression_type(kvproto::brpb::CompressionType::Lz4);
    req.set_compression_level(1);
    req.set_is_raw_kv(false);
    req.set_cf(String::from("default"));
    // Set an unique path to avoid AlreadyExists error.
    req.set_storage_backend(make_local_backend(&Path::new(&external_storage)));
    let mut stream = client.backup(&req).unwrap();
    // let (tx, rx) = std::sync::mpsc::channel();
    client.spawn(async move {
        let resp = stream.next().await;
        println!("{:?}", resp);
        // tx.send(resp);
    });

    loop {
        // let x = rx.recv().unwrap();
        // println!("{:?}", x);
        // std::thread::sleep(std::time::Duration::new(10, 0));
    }
}
