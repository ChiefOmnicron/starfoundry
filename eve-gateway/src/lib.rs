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
/// useful utility functions
pub mod utils;

/// functions for fetching assets
pub mod asset;
/// public eve route for characters
pub mod character;
/// eve route for all contracts
pub mod contract;
/// routes for corporations
pub mod corporation;
/// all industry related routes
pub mod industry;
/// public eve route for items
pub mod item;
/// public eve route for market
pub mod market;
/// routes for searching
pub mod search;
/// routes for structure information
pub mod structure;
/// public eve route for universe
pub mod universe;

/// internal routes, primarily for other services
pub mod internal;
