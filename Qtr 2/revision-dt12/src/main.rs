fn main() {
    println!("Revsion after months!");
    let receive = utype(35);
    println!("  Returned u8 value    {} ", receive);
    let sreceive = stype(String::from("abc"));
    println!("  Returned String Value  {} ", sreceive);
    let greceive = gtype(35);
}

fn utype(udata: u8) -> u8 {
    udata
}

fn stype(sdata: String) -> String {
    sdata
}

fn gtype<I>(gdata: I) -> I {
    gdata
}
