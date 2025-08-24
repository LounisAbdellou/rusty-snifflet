use pcap::{Capture, Device};

fn main() {
    let main_device = match Device::lookup() {
        Ok(device) => device,
        Err(err) => panic!("{}", err),
    }
    .expect("No suitable device found.");

    let mut capture = Capture::from_device(main_device)
        .unwrap()
        .promisc(true)
        .snaplen(5000)
        .open()
        .unwrap();

    while let Ok(packet) = capture.next_packet() {
        println!("received packet! {:?}", packet);
    }
}
