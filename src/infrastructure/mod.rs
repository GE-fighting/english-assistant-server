//! Infrastructure Layer Module
//!
//! This module implements technical concerns and external integrations, including:
//!
//! - Database Access (`database/`): Database connections and operations
//! - Caching (`cache/`): Redis and other caching mechanisms
//! - External Services (`services/`): Implementation of domain service interfaces
//! - Third-party Integrations (`external/`): External API clients and adapters
//!
//! The infrastructure layer provides concrete implementations of interfaces defined
//! in the domain layer and handles all external resource interactions.

pub mod cache;
pub mod database;
pub mod dto;
pub mod llm;
pub mod third_party;
