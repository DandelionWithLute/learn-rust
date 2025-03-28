// enum is also a kind of route

fn enums() {
    enum IpAddrKind {
        V4,
        V6,
    }

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    fn route(_ip_kind: IpAddrKind) {}

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}
