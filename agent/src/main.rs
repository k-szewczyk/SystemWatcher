extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::any::Any;
use std::thread::sleep;
use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

#[derive(Serialize, Deserialize)]
struct Node {
    address: String,
    name: String,
}

#[tokio::main]
async fn main() -> () {
    let node = Node { address: String::from("127.0.0.1"), name: String::from("test") };
    let client = reqwest::Client::new();
    let resp = client.post("http://localhost:8000/nodes")
        .json(&node)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{:#?}", resp);
    return
    // println!("{:#?}", resp);
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    // let mut sys = System::new_all();
    //
    // // First we update all information of our `System` struct.
    // loop {
    //     sys.refresh_all();
    //
    //     // We display all disks' information:
    //     println!("=> disks:");
    //     for disk in sys.disks() {
    //         println!("{:?}", disk);
    //     }
    //
    //     // Network interfaces name, data received and data transmitted:
    //     println!("=> networks:");
    //     for (interface_name, data) in sys.networks() {
    //         println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
    //     }
    //
    //     // Components temperature:
    //     println!("=> components:");
    //     for component in sys.components() {
    //         println!("{:?}", component);
    //     }
    //
    //     println!("=> system:");
    //     // RAM and swap information:
    //     println!("total memory: {} KB", sys.total_memory());
    //     println!("used memory : {} KB", sys.used_memory());
    //     println!("total swap  : {} KB", sys.total_swap());
    //     println!("used swap   : {} KB", sys.used_swap());
    //
    //     // Display system information:
    //     println!("System name:             {:?}", sys.name());
    //     println!("System kernel version:   {:?}", sys.kernel_version());
    //     println!("System OS version:       {:?}", sys.os_version());
    //     println!("System host name:        {:?}", sys.host_name());
    //
    //     // Number of processors:
    //     println!("NB processors: {}", sys.processors().len());

    //
    //     sleep(Duration::from_secs(1))
    // }
}
