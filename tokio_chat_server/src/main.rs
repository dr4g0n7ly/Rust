use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on 127.0.0.1:8080");

    let (mut socket, _) = listener.accept().await.unwrap();
    handle_client(&mut socket).await;
}

async fn handle_client(socket: &mut TcpStream) {
    let mut buffer = vec![0; 1024];
    let nbytes = match socket.read(&mut buffer).await {
        Ok(n) if n == 0 => return,
        Ok(n) => n,
        Err(_) => {
            eprintln!("Failed to read from socket");
            return;
        }
    };

    let message = String::from_utf8_lossy(&buffer[..nbytes]);
    println!("Received message: {}", message);
}
