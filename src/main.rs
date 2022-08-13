
extern crate network;
use network::ThreadPool;

mod router;

fn main() {
    // router listen
    router::router_main::router();
}