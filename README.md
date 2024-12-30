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
```
1. The `LISTENER_PORT` determines where the proxy will listen for incoming connections.
2. The `BACKEND_PORTS` field defines the backend servers that the proxy will load balance traffic between.
3. The `ACCESS_CONTROL_LIST` restricts which client IP addresses are allowed to connect to the proxy.

# TODO

This document outlines the tasks and improvements planned for the L4 Proxy project.

---

## General Improvements

- [ ] **Improve Configuration Handling**:
  - Validate the configuration file at runtime for required fields and correct formats.

- [ ] **Logging**:
  - Replace `println!` with `tokio-tracing` for structured and efficient logging.
  - Add logging levels (e.g., INFO, DEBUG, ERROR) for better observability.

- [ ] **Code Quality**:
  - Refactor code to follow idiomatic Rust practices (e.g., use camelCase for struct fields).

---

## Features

- [ ] **Testing**:
  - Add unit tests for:
    - Access Control List (ACL) validation.
    - Load balancing logic (e.g., backend selection).
  - Add integration tests to validate:
    - Successful forwarding of client traffic to backend servers.
    - ACL enforcement (e.g., denied connections).
  - Add benchmark tests to measure:
    - Concurrency and throughput under various conditions.
    - Resource utilization during high traffic.

- [ ] **Environment Support**:
  - Optionally support reading configuration from environment variables for containerization (although for now `config.json` is being used).

- [ ] **Error Handling**:
  - Improve error handling across all modules with meaningful messages.
  - Ensure graceful shutdown on critical errors.

---

## Benchmarking

- [ ] Develop a benchmarking suite to:
  - Test throughput under various concurrency levels.
  - Measure CPU and memory usage during sustained traffic.

- [ ] Document results for different scenarios (e.g., high client load, many backends).


