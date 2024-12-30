# L4 Proxy (TCP)

A Layer 4 proxy with load balancing, access control, and connection forwarding between clients and backend servers.

## Goals

This project aims to:

1. Provide a basis for learning and experimentation with Rust for network programming.
2. Gather feedback and contributions to improve functionality, performance, and usability.

## Features

- **Access Control**: Restrict incoming connections based on an Access Control List (ACL).
- **Load Balancing**: Distributes traffic among backend servers using a connection count-based algorithm.
- **Connection Forwarding**: Handles bidirectional traffic between clients and backend servers.

## How It Works

1. Listens for client connections on a configurable port.
2. Verifies client IPs against an ACL. Unauthorized connections are denied with an error message.
3. Selects a backend server using a load balancer based on active connections.
4. Forwards client traffic to the backend server and vice versa, maintaining seamless communication.

## Project Structure

```
src/
├── main.rs            // Entry point of the proxy
├── proxy/             // Handles bidirectional traffic forwarding
├── config/            // Reads and parses configuration
├── balancer/          // Implements the load balancer
└── acl/               // Implements access control
```

## Example Use Cases

- Load balancing incoming TCP traffic for microservices or backend servers.
- Restricting access to internal systems based on IP addresses.
- Experimenting with and learning about Layer 4 proxy concepts.

.

### Configuration

Create a `config.json` file in the root directory with the following structure:

```json
{
  "LISTENER_PORT": "127.0.0.1:8080", 
  "BACKEND_PORTS": [
    "127.0.0.1:8081",
    "127.0.0.1:8082"
  ],
  "ACCESS_CONTROL_LIST": [
    "192.168.1.10",
    "192.168.1.11"
  ]
}
