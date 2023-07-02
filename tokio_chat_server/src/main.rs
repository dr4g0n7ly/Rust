use tokio::{
    io::{BufReader, AsyncBufReadExt, AsyncWriteExt}, 
    net::TcpListener,
};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
    
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
        
            loop {
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                if bytes_read == 0 {
                    break;
                }
        
                writer.write_all(line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }


    // handle_client(&mut socket).await;
}

// async fn handle_client(socket: &mut TcpStream) {
//     let mut buffer = vec![0; 1024];
//     let nbytes = match socket.read(&mut buffer).await {
//         Ok(n) if n == 0 => return,
//         Ok(n) => n,
//         Err(_) => {
//             eprintln!("Failed to read from socket");
//             return;
//         }
//     };

//     let message = String::from_utf8_lossy(&buffer[..nbytes]);
//     println!("Received message: {}", message);
// }
