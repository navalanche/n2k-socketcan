// The tests here assume a running 'vcan0' device under Linux.
// The following commands start a vcan0 device:
// sudo modprobe vcan
// sudo ip link add dev vcan0 type vcan
// sudo ip link set up vcan0

extern crate n2k_base;
extern crate n2k_socketcan;

use n2k_socketcan::N2kSocket;
use n2k_base::n2k::j1393;

#[test]
fn open_vcan0() {
    assert!(N2kSocket::open("vcan0").is_ok());
}

#[test]
fn open_vcan0_multiple() {
    (0..5).for_each(
        |_| assert!(N2kSocket::open("vcan0").is_ok())
    );
}

// This test should fail on any machine that has no vcan1 device
#[test]
fn open_vcan1() {
    assert!(N2kSocket::open("vcan1").is_err());
}

#[test]
fn send_iso_address_claim() {
    let socket = N2kSocket::open("vcan0").unwrap();

    let socket2 = N2kSocket::open("vcan0").unwrap();

    let name = j1393::N2kName::new_from_parts(
        0, 0, 0, 0, 0, 0, 0, 0, 0xB0, 0
    );

    let message = j1393::create_iso_address_claim(0, 1, &name);
    assert!(socket.write_message(&message).is_ok());


    let received = socket2.receive_message();
}
