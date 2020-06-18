use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::{ipv4::Ipv4Packet, tcp::TcpPacket, Packet};

use std::env;

fn main() {
    let interface_name = env::args().nth(1).unwrap();
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(|i| i.name == interface_name)
        .next()
        .expect("Error get interface");
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Ignore"),
        Err(e) => panic!("Error: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                handle_packet(&packet);
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn handle_packet(ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => {
            if let Some(header) = Ipv4Packet::new(ethernet.payload()) {
                match header.get_next_level_protocol() {
                    IpNextHeaderProtocols::Tcp => {
                        if let Some(tcp) = TcpPacket::new(header.payload()) {
                            println!(
                                "Tcp packet, {}: {} to {}: {}",
                                header.get_source(),
                                tcp.get_source(),
                                header.get_destination(),
                                tcp.get_destination()
                            );
                        }
                    }
                    _ => println!("ignore other header"),
                }
            }
        }
        _ => println!("Ignore packet"),
    }
}
