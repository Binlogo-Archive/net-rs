use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

fn main() {
    let _v4 = Ipv4Net::new(Ipv4Addr::new(10, 1, 1, 0), 24);
    let _v6 = Ipv6Net::new(Ipv6Addr::new(0xfd, 0, 0, 0, 0, 0, 0, 0), 24);

    let _v4 = Ipv4Net::from_str("10.1.1.0/24");
    let _v6 = Ipv6Net::from_str("fd00::/24");

    let v4: Ipv4Net = "10.1.1.0/24".parse().unwrap();
    let _v6: Ipv6Net = "fd00::/24".parse().unwrap();

    let _net = IpNet::from(v4);
    let _net = IpNet::from_str("10.1.1.0/24").unwrap();
    let _net: IpNet = "10.1.1.0/24".parse().unwrap();

    println!("{}, hostmask = {}", _net, _net.hostmask());
    println!("{}, netmask = {}", _net, _net.netmask());

    assert_eq!(
        "192.168.12.34/16".parse::<IpNet>().unwrap().trunc(),
        "192.168.0.0/16".parse().unwrap()
    )
}
