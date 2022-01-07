use std::{io, net::TcpListener};

use zero2prod::run;

#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Listening to http://{}/", listener.local_addr().unwrap());
    run(listener)?.await
}
