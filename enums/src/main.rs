enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let ip_v4 = IpAddrKind::V4;
    let ip_v6 = IpAddrKind::V6;

    let home_v4 = IpAddr {
        kind: ip_v4,
        address: String::from("127.0.0.1"),
    };

    let home_v6 = IpAddr {
        kind: ip_v6,
        address: String::from("::1"),
    };

    println!("IP V4: {}", home_v4.address);
    println!("IP V6: {}", home_v6.address);
}
