//! Domain Layer Module
//!
//! This module contains the core business logic and rules of the application.
//! It is independent of external concerns and frameworks, containing:
//!
//! - Domain Models (`models/`): Core business entities and value objects
//! - Service Traits (`services/`): Business logic interfaces
//! - Domain Events (`events/`): Business event definitions
//!
//! The domain layer represents the heart of the application, encoding business rules
//! and ensuring they are enforced consistently throughout the system.

pub mod models;
pub mod services;
