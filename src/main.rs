use std::path::Path;

use anyhow::Result;
use cmd::{build_cli, BackupInfo};
use futures::StreamExt;
use kvproto::{
    brpb::{BackupRequest, StorageBackend},
    brpb_grpc::BackupClient,
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
    let kv = "127.0.0.1:20180";
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

    let mut req = BackupRequest::new();
    req.set_start_key(range.start_key);
    req.set_end_key(range.end_key);

    req.set_start_version(0);
    req.set_end_version(438366606733082625);
    // Set an unique path to avoid AlreadyExists error.
    req.set_storage_backend(make_local_backend(&Path::new(&external_storage)));
    let mut stream = client.backup(&req).unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    client.spawn(async move {
        let resp = stream.next().await;
        tx.send(resp);
    });

    loop {
        let x = rx.recv().unwrap();
        println!("{:?}", x);
        std::thread::sleep(std::time::Duration::new(10, 0));
    }
}
