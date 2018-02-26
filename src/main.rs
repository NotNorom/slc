extern crate clap;

use clap::*;
use std::net::UdpSocket;

fn send(sock: &UdpSocket, address: &String, seq: u8, hid: u8, ofs:u8, len: u16,
        r: u8, g: u8, b: u8, w: u8) -> std::io::Result<usize> {
    if len == 0 {
        return Ok(0);
    }
    let header_len = 4;
    let mut packet = vec![0 as u8; (header_len + len*4) as usize];
    // VER|SEQ|HID|OFS|G,R,B,W,G,...
    packet[0] = 2; // version
    packet[1] = seq; // sequence
    packet[2] = hid; // host id
    packet[3] = ofs; // offset
    for i in 0..len {
        packet[(header_len + (i*4) + 0) as usize] = g;
        packet[(header_len + (i*4) + 1) as usize] = r;
        packet[(header_len + (i*4) + 2) as usize] = b;
        packet[(header_len + (i*4) + 3) as usize] = w;
    }

    return sock.send_to(&packet, &address);
}


fn main() {
    let socket = UdpSocket::bind("0.0.0.0:10400").expect(
        "Could not setup socket");
    drop(socket.set_broadcast(true));
    let address: String;
    let sequence_number: u8;
    let hostid: u8;
    let offset: u8;
    let number_of_leds: u16;
    let red: u8;
    let green: u8;
    let blue: u8;
    let white: u8;

    let matches = App::new("simple-led-control")
                            .version("0.3")
                            .author("Andre Julius <noromoron@gmail.com>")
                            .about("Control led strips")
                            .arg(Arg::with_name("verbose")
                                .short("v")
                                .long("verbose")
                                .multiple(true)
                                .help("Show some debugging prints"))
                            .arg(Arg::with_name("sequence")
                                .short("s")
                                .long("sequence-number")
                                .help("Sets the sequence number")
                                .takes_value(true))
                            .arg(Arg::with_name("hostid")
                                .short("h")
                                .long("hostid")
                                .help("Sets the host id")
                                .takes_value(true))
                            .arg(Arg::with_name("offset")
                                .short("o")
                                .long("offset")
                                .help("Sets the offset")
                                .takes_value(true))
                            .arg(Arg::with_name("number")
                                .short("n")
                                .long("number-of-leds")
                                .help("Sets the number of leds to fill")
                                .takes_value(true))
                            .arg(Arg::with_name("red")
                                .short("r")
                                .long("color-red")
                                .help("Sets the red value")
                                .takes_value(true))
                            .arg(Arg::with_name("green")
                                .short("g")
                                .long("color-green")
                                .help("Sets the green value")
                                .takes_value(true))
                            .arg(Arg::with_name("blue")
                                .short("b")
                                .long("color-blue")
                                .help("Sets the blue value")
                                .takes_value(true))
                            .arg(Arg::with_name("white")
                                .short("w")
                                .long("color-white")
                                .help("Sets the white value")
                                .takes_value(true))
                            .arg(Arg::with_name("address")
                                .help("Set the address of the led strip.")
                                .required(true)
                                .index(1))
                            .get_matches();

    address = value_t!(matches.value_of("address"), String).unwrap();

    sequence_number = value_t!(matches.value_of("sequence"), u8).unwrap_or(0);
    hostid = value_t!(matches.value_of("hostid"), u8).unwrap_or(0);
    offset = value_t!(matches.value_of("offset"), u8).unwrap_or(0);
    number_of_leds = value_t!(matches.value_of("number"), u16).unwrap_or(0);

    // Get all red, green and blue values
    red = value_t!(matches.value_of("red"),   u8).unwrap_or(0);
    green = value_t!(matches.value_of("green"), u8).unwrap_or(0);
    blue = value_t!(matches.value_of("blue"),  u8).unwrap_or(0);
    white = value_t!(matches.value_of("white"), u8).unwrap_or(0);

    match matches.occurrences_of("v") {
        0 => {},
        1 => {
            println!("Using address: {}", address);
            println!("Value for red: {}, green: {}, blue: {}, white: {}",
                     red, green, blue, white);
            println!("Leds to fill: {}", number_of_leds);
        },
        2 | _ => {},
    }

    match send(&socket, &String::from(address), sequence_number, hostid,
        offset, number_of_leds, red, green, blue, white) {
        Err(e) => {
            match e.get_ref() {
                Some(inner_e) => println!("Error: {}", inner_e.description()),
                None => println!("{:?}", e),
            }
        },
        Ok(u) => println!("Bytes sent: {}", u),
    };
}