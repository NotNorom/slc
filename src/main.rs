use std::net::UdpSocket;


fn fill_rgb(sock: &UdpSocket, address: &String, len: usize, r: u8, g: u8, b: u8) -> usize {
    let mut bytes = vec![0 as u8; len*3];
    for i in 0..len {
        bytes[(i*3) + 0] = g;
        bytes[(i*3) + 1] = r;
        bytes[(i*3) + 2] = b;
    }
    sock.send_to(&bytes, &address).expect("error sending data")
}


fn main() {
    let socket = UdpSocket::bind("0.0.0.0:10400").expect(
        "Could not connect");

    let bytes_sent = fill_rgb(&socket, &"nomled:1234".to_string(), 20, 200, 100, 20);
    println!("Total bytes sent: {:?}", bytes_sent);
}
