// Enum
// A versatile tool used to represent a type that can take one of several possible variables

/*
enum IpAddrKind {
    V4,
    V6,
}
*/

fn main() {
    enum IpAddrKind {
        V4(),
        V6(),
    }

let _four: IpAddrKind = IpAddrKind::V4;
let _six: IpAddrKind = IpAddrKind::V6;

    fn route(_ip_kind: IpAddrKind){}

    route(_ip_kind: IpAddrKind::V4);
    route(_ip_kind: IpAddrKind::V6);


    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }

    // Using Enums
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback: IpAddr = IpAddr::V6(String::from("::1"));


    // Enhanced Enums:
    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }
    let home = IpAddr::V4(127,0,0,1);
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));


    /*
    // Using structs
    struct IpAddr{
        kind: IpAddrKind,
        address: String,
    }

    let home: IpAddr = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback: IpAddr = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

*/
}
