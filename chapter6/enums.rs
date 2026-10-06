enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

struct Ipv4Addr {
}

struct Ipv6Addr {
}

enum IpAddr2 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Option<T> {
    None,
    Some(T),
}


fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}