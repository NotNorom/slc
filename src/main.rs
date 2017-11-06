extern crate clap;

use clap::*;
use std::net::UdpSocket;


fn fill_rgb(sock: &UdpSocket, address: &String, len: usize, seq_number: u8, r: u8, g: u8, b: u8) -> usize {
    if len == 0 {
        return 0;
    }
    let mut bytes = vec![0 as u8; 1+len*3];
    bytes[0] = seq_number;
    for i in 1..len {
        // the recieving strip recieves SEQ|G,R,B,G,...
        bytes[(i*3) + 1] = g;
        bytes[(i*3) + 2] = r;
        bytes[(i*3) + 0] = b;
    }
    return sock.send_to(&bytes, &address).expect("error sending data");
}

fn fill_rgbw(sock: &UdpSocket, address: &String, len: usize, seq_number: u8, r: u8, g: u8, b: u8, w: u8) -> usize {
    if len == 0 {
        return 0;
    }
    let mut bytes = vec![0 as u8; 1+len*4];
    bytes[0] = seq_number;
    for i in 1..len {
        // the recieving strip recieves SEQ|G,R,B,W,G,...
        bytes[(i*4) + 1] = g;
        bytes[(i*4) + 2] = r;
        bytes[(i*4) + 3] = b;
        bytes[(i*4) + 0] = w;
    }
    return sock.send_to(&bytes, &address).expect("error sending data");
}


fn main() {
    let socket = UdpSocket::bind("0.0.0.0:10400").expect(
        "Could not setup socket");
    socket.set_broadcast(true);
    let address: String;
    let number_of_leds: usize;
    let sequence_number: u8;
    let red_value: u8;
    let green_value: u8;
    let blue_value: u8;
    let white_value: u8;
    let dedicated_white_led: bool;

    let matches = App::new("simple-led-control")
                            .version("0.2")
                            .author("Andre Julius <noromoron@gmail.com>")
                            .about("Control led strips")
                            .arg(Arg::with_name("verbose")
                                .short("v")
                                .long("verbose")
                                .multiple(true)
                                .help("Show some debugging prints"))
                            .arg(Arg::with_name("number")
                                .short("n")
                                .long("number-of-leds")
                                .help("Sets the number of leds to fill")
                                .takes_value(true))
                            .arg(Arg::with_name("sequence")
                                .short("s")
                                .long("sequence-number")
                                .help("Sets the sequence number")
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
                            .arg(Arg::with_name("color-mode")
                                .short("c")
                                .long("color-mode")
                                .help("Sets the color mode. rgb, rgbw")
                                .takes_value(true))
                            .arg(Arg::with_name("address")
                                .help("Set the address of the led strip.")
                                .required(true)
                                .index(1))
                            .get_matches();

    address = value_t!(matches.value_of("address"), String).unwrap();
    number_of_leds = value_t!(matches.value_of("number"), usize).unwrap_or(0);

    sequence_number = value_t!(matches.value_of("sequence"), u8).unwrap_or(0);

    // Get all red, green and blue values
    red_value = value_t!(matches.value_of("red"),   u8).unwrap_or(0);
    green_value = value_t!(matches.value_of("green"), u8).unwrap_or(0);
    blue_value = value_t!(matches.value_of("blue"),  u8).unwrap_or(0);

    // 
    match matches.value_of("white") {
        Some(w) => {
            drop(w);
            dedicated_white_led = true;
            white_value = value_t!(matches.value_of("white"), u8).unwrap_or(0);
        },
        None => {
            dedicated_white_led = false;
            white_value = 0;
        },
    }

    let color_mode = matches.value_of("color_mode").unwrap_or("rgb");

    match matches.occurrences_of("v") {
        0 => {},
        1 => {
            println!("Using address: {}", address);
            println!("Value for red: {}, green: {}, blue: {}, white: {}", red_value, green_value, blue_value, white_value);
            println!("Using a dedicated white led? {}", dedicated_white_led);
            println!("Leds to fill: {}", number_of_leds);
        },
        2 | _ => {},
    }

    if dedicated_white_led {
        println!("Bytes sent: {}", fill_rgbw(&socket, &String::from(address),
            number_of_leds, sequence_number, red_value, green_value,
            blue_value, white_value));
    } else {
        println!("Bytes sent: {}", fill_rgb(&socket, &String::from(address),
            number_of_leds, sequence_number, red_value, green_value,
            blue_value));
    }
}