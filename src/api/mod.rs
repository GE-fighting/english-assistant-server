//! API Layer Module
//!
//! This module handles all external HTTP interactions and provides the web interface
//! for the application. It includes:
//!
//! - Request handlers (`handler/`): Process incoming HTTP requests
//! - Data Transfer Objects (`dto/`): Define request/response data structures
//! - Routes (`routes/`): Define and configure API endpoints
//!
//! The API layer acts as a boundary between the external world and our domain logic,
//! ensuring proper request validation and response formatting.

pub mod dto;
pub mod handler;
pub mod routes;

pub use routes::configure_routes;
