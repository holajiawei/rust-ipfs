//! Persistent fs backed repo
use crate::block::{Cid, Block};
use crate::repo::{BlockStore, DataStore};
use std::collections::HashSet;
use futures::future::FutureObj;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
//use tokio::fs;

#[derive(Clone, Debug)]
pub struct FsBlockStore {
    path: PathBuf,
    cids: Arc<Mutex<HashSet<Cid>>>,
}

impl BlockStore for FsBlockStore {
    fn new(mut path: PathBuf) -> Self {
        path.push("blockstore");
        FsBlockStore {
            path,
            cids: Arc::new(Mutex::new(HashSet::new()))
        }
    }

    fn init(&self) -> FutureObj<'static, ()> {
        //FutureObj::new(Box::new(fs::create_dir_all(self.path.clone())))
        FutureObj::new(Box::new(futures::future::ready(()))
    }

    // TODO open

    fn contains(&self, cid: Cid) -> FutureObj<'static, bool> {
        let contains = self.cids.lock().unwrap().contains(&cid);
        FutureObj::new(Box::new(futures::future::ready(contains))
    }

    fn get(&self, _cid: Cid) -> FutureObj<'static, Option<Block>> {
        // TODO
        FutureObj::new(Box::new(futures::future::ready(None)))
    }

    fn put(&self, block: Block) -> FutureObj<'static, Cid> {
        let _path = block_path(self.path.clone(), &block);
        FutureObj::new(Box::new(futures::future::ready(block.cid())))
                 /*
            fs::File::open(path)
                .and_then(|file| {
                    tokio::io::write_all(file, *block.data())
                }).map(|_| {
                    self.cids.lock().unwrap().insert(block.cid());
                    block.cid()
                })
        )*/
    }

    fn remove(&self, _cid: Cid) -> FutureObj<'static, ()> {
        // TODO
        FutureObj::new(Box::new(futures::future::ready(())))
    }
}

#[derive(Clone, Debug)]
pub struct RocksDataStore {
    path: PathBuf,
}

impl DataStore for RocksDataStore {
    fn new(mut path: PathBuf) -> Self {
        path.push("datastore");
        RocksDataStore {
            path
        }
    }
}

fn block_path(mut base: PathBuf, block: &Block) -> PathBuf {
    let cid = block.cid();
    let mut file = cid.to_string();
    file.push_str(".data");
    base.push(file);
    base
}