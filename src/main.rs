
use std::error::Error;
use tokio::{self, io::{AsyncReadExt, AsyncWriteExt}};


mod routes;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (client_connection, address) = listener.accept().await.unwrap();
        println!("new connection from {}", address);
        tokio::spawn(async move {
            if let Err(e) = process(client_connection).await {
                println!("Error processing connection: {}", e);
            }
        });
    }
}

async fn process(mut client: tokio::net::TcpStream) -> Result<(), Box<dyn Error>> {
    println!("Processing connection");

    // Establish connection to the backend server
    let mut server = tokio::net::TcpStream::connect("127.0.0.1:3000").await?;
    println!("Connected to backend server at 127.0.0.1:3000");

    let mut client_buf = vec![0u8; 1024]; // Buffer for reading client data
    let mut server_buf = vec![0u8; 1024]; // Buffer for reading server data
    loop {
        tokio::select! {
            // Read data from the client and forward to the backend
            client_read = client.read(&mut client_buf) => {
                match client_read {
                    Ok(0) => {
                        // Client closed the connection
                        println!("Client closed the connection");
                        server.shutdown().await.unwrap();
                        break;
                    }
                    Ok(n) => {
                        // Client sent data, forward it to the backend
                        println!("Client message: {}", String::from_utf8_lossy(&client_buf[..n]));
                        if let Err(e) = server.write_all(&client_buf[..n]).await {
                            println!("Error forwarding data to backend: {}", e);
                        }
                        println!("Forwarded {} bytes from client to backend", n);
                    }
                    Err(e) => {
                        // Error reading from the client
                        println!("Error reading from client: {}", e);
                        break;
                    }
                }
            },
            // Read data from the backend and forward to the client
            server_read = server.read(&mut server_buf) => {
                match server_read {
                    Ok(0) => {
                        // Server closed the connection , ideally in TCPStream, read/recv will return 0 on EOF/closed connection
                        println!("Server closed the connection");
                        client.shutdown().await.unwrap();
                        break;
                    }
                    Ok(n) => {
                        // Backend sent data, forward it to the client
                        println!("Server message: {}", String::from_utf8_lossy(&server_buf[..n]));
                        if let Err(e) = client.write_all(&server_buf[..n]).await {
                            println!("Error forwarding data to client: {}", e);
                        }
                        println!("Forwarded {} bytes from backend to client", n);
                    }
                    Err(e) => {
                        // Error reading from the backend
                        println!("Error reading from backend: {}", e);
                    }
                }
            }
        }
    }

    Ok(())
}