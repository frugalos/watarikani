extern crate cannyls;
extern crate cannyls_rpc;
extern crate fibers;
extern crate fibers_rpc;
extern crate futures;
#[macro_use]
extern crate trackable;

use cannyls::device::DeviceBuilder;
use cannyls::lump::{LumpData, LumpId};
use cannyls::nvm::MemoryNvm;
use cannyls::storage::StorageBuilder;
use cannyls_rpc::{Client, DeviceId, DeviceRegistry, Server};
use fibers::{Executor, InPlaceExecutor, Spawn};
use fibers_rpc::client::ClientService;
use fibers_rpc::server::ServerBuilder;
use futures::{Async, Future};
use std::thread;

macro_rules! wait {
    ($future:expr) => {{
        let mut f = $future;
        loop {
            if let Async::Ready(item) = track_try_unwrap!(f.poll()) {
                break item;
            }
        }
    }};
}

fn to_device_id(d: &str) -> DeviceId {
    DeviceId::new(d)
}

fn main() {
    let server_addr = "127.0.0.1:14278".parse().unwrap();
    
    let executor = track_try_unwrap!(track_any_err!(InPlaceExecutor::new()));

    let service = ClientService::new(executor.handle());
    let service_handle = service.handle();
    executor.spawn(service.map_err(|e| panic!("{}", e)));

    thread::spawn(move || {
        if let Err(e) = executor.run() {
            panic!("{}", e);
        }
    });
    
    let client = Client::new(server_addr, service_handle);
    let request = client.request();

    wait!(request.list_lumps(to_device_id("test")));
}
