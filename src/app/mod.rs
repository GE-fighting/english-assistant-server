//! Application Core Module
//!
//! This module serves as the central hub of the application, managing core components
//! and orchestrating the interaction between different layers. It provides:
//!
//! - Dependency Injection Container
//! - Application Configuration
//! - Core Application Bootstrapping
//!
//! The `app` module maintains the runtime state of the application and ensures
//! proper initialization and lifecycle management of all services.

mod handler_factory;
mod redis_factory;
mod repository_factory;
mod request_logger;
mod service_container;

pub use handler_factory::HandlerFactory;
pub use request_logger::RequestLogger;
pub use service_container::ServiceContainer;
