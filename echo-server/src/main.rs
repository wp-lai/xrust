use clap::{arg, App};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let matches = App::new("echo-server")
        .arg(arg!(-p --port "binding port").default_value("8000"))
        .get_matches();
    let port = matches.value_of("port").unwrap();
    let addr = format!("localhost:{}", port);

    let listener = TcpListener::bind(addr).await.unwrap();
    let (mut socket, _addr) = listener.accept().await.unwrap();

    loop {
        let mut buffer = [0u8; 1024];
        let bytes_read = socket.read(&mut buffer).await.unwrap();
        socket.write_all(&buffer[..bytes_read]).await.unwrap();
    }
}
