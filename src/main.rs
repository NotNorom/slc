use std::net::UdpSocket;
use std::env::args;
use std::num::ParseIntError;


fn fill_rgb(sock: &UdpSocket, address: &String, len: usize, r: u8, g: u8, b: u8) -> usize {
    if len == 0 {
        return 0
    }
    let mut bytes = vec![0 as u8; len*3];
    for i in 0..len {
        bytes[(i*3) + 0] = g;
        bytes[(i*3) + 1] = r;
        bytes[(i*3) + 2] = b;
    }
    sock.send_to(&bytes, &address).expect("error sending data") / 3
}


fn main() {
    let socket = UdpSocket::bind("0.0.0.0:10400").expect(
        "Could not setup socket");

    let address: String = args().nth(1).expect("No address");
    let len: Result<usize, ParseIntError> = args().nth(2).expect("No length").parse();
    let red: Result<u8, ParseIntError> = args().nth(3).expect("No red color value").parse();
    let gre: Result<u8, ParseIntError> = args().nth(4).expect("No green color value").parse();
    let blu: Result<u8, ParseIntError> = args().nth(5).expect("No blue color value").parse();

    let bytes_sent = fill_rgb(&socket, &address, len.unwrap(), red.unwrap(), gre.unwrap(), blu.unwrap());
    println!("sent: {:?}", bytes_sent);
}
