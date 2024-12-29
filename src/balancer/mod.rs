use std::collections::HashMap;

#[derive(Debug)]
pub struct LoadBalancer {
    // Map of backend addresses to the count of active connections
    active_connections: HashMap<String, usize>,
}

impl LoadBalancer {
    // Constructor to create a new Balancer with a list of backend addresses
    pub fn new(backends: Vec<String>) -> Self {
        let active_connections = backends.into_iter().map(|backend| (backend, 0)).collect();
        LoadBalancer { active_connections }
    }

    // Method to increment the connection count for a backend
    pub fn increment_connection(&mut self, backend_address: &str) {
        if let Some(count) = self.active_connections.get_mut(backend_address) {
            *count += 1;
        }
    }

    // Method to decrement the connection count for a backend
    pub fn decrement_connection(&mut self, backend_address: &str) {
        if let Some(count) = self.active_connections.get_mut(backend_address) {
            *count = count.saturating_sub(1);
        }
    }

    // Method to select a backend with the least active connections
    pub fn select_backend(&self) -> Option<String> {
        self.active_connections
            .iter()
            .min_by_key(|&(_, count)| count)
            .map(|(backend, _)| backend.clone())
    }

    // Method to get the active connection count for a specific backend
    pub fn get_connection_count(&self, backend_address: &str) -> Option<usize> {
        self.active_connections.get(backend_address).copied()
    }
}
