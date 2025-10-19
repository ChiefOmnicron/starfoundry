//! EVE-Gateway services.
//! 
//! The service sis designed to abstract everything away from the EVE-API.
//! 
//! Current supported features:
//! - Login with EVE SSO
//! - Wraps a small set of EVE-API calls (see the OpenAPI Documentation)
//! - Caching of routes
//! 
/// general helper function and structs for presenting the api documentation
pub mod api_docs;
/// general authentication module
pub mod auth;
/// loading and validating from configurations
pub mod config;
/// wrapper client for the eve api
pub mod eve_client;
/// application health checks
pub mod healthcheck;
/// application metrics
pub mod metrics;
/// state for the web application
pub mod state;

/// public eve route for characters
pub mod character;
/// public eve route for items
pub mod item;
/// routes for structure information
pub mod structure;
/// public eve route for universe
pub mod universe;
