use std::sync::Arc;

use acl::is_allowed;
use balancer::LoadBalancer;
use config::Config;
use proxy::process;
use tokio::{io::AsyncWriteExt, sync::Mutex};

mod acl;
mod balancer;
mod config;
mod proxy;

#[tokio::main]
async fn main() {
    // Load configuration
    let config = Config::read_from_file("config.json").unwrap();
    println!("Loaded Config: {:?}", config);

    let backends: Vec<String> = config
        .BACKEND_PORTS
        .iter()
        .map(|backend| backend.to_string())
        .collect();
    let load_balancer = Arc::new(Mutex::new(LoadBalancer::new(backends)));
    // Start the TCP listener
    let listener = tokio::net::TcpListener::bind(&config.LISTENER_PORT)
        .await
        .unwrap();
    println!("Listening on {}", config.LISTENER_PORT);

    loop {
        let (mut client_connection, address) = listener.accept().await.unwrap();
        println!("New connection from {}", address);

        // Check ACL
        if !is_allowed(&config, address.ip()) {
            println!("Connection from {} denied", address);
            let message = "Access Denied: Your IP is not authorized";
            client_connection
                .write_all(message.as_bytes())
                .await
                .unwrap();
            println!("Unauthorized access attempt from: {}", address);
            client_connection.shutdown().await.unwrap();
            continue;
        }

        let load_balancer = Arc::clone(&load_balancer);

        // Spawn a task to handle the connection
        tokio::spawn(async move {
            let selected_server = load_balancer.lock().await.select_backend();

            if let Some(server) = selected_server {
                println!("Selected backend server: {}", server);
                load_balancer.lock().await.increment_connection(&server);
                if let Err(e) = process(client_connection, &server).await {
                    println!("Error processing connection: {}", e);
                }
                load_balancer.lock().await.decrement_connection(&server);
            }
        });
    }
}
