use std::collections::HashMap;
use serde::{Serialize, Deserialize};
#![feature(once_cell)]
use std::{lazy::SyncLazy, sync::Mutex};

mod server {
    static SERVER: SyncLazy<Mutex<Server>> = SyncLazy::new(|| Mutex::new(Server::new()));

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Server {
        currentURL: String;
    }

    impl Server {
        pub fn join(url: &'static str) {
            SERVER.lock().unwrap().currentURL = String::from(url);
        }
    }
}