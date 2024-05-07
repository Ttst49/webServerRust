use std::net::TcpListener;


fn main() {
    let listener =
        TcpListener::bind("127.0.0.1:9999").unwrap();

    for flux in listener.incoming() {
        let flux = flux.unwrap();

        println!("Connexion established")
    }
}
