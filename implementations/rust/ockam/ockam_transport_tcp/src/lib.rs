//! TCP Transport utilities for Ockam's routing framework
//!
//! The `ockam_node` crate sits at the core
//! of the Ockam routing framework, with transport specific
//! abstraction plugins.  This crate implements a TCP connection
//! plugin for this architecture.
//!
//! You can use Ockam's routing mechanism for cryptographic protocols,
//! key lifecycle, credential exchange, enrollment, etc, without having
//! to worry about the transport specifics.
#![deny(unsafe_code)]
#![warn(
    missing_docs,
    dead_code,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

#[cfg(feature = "alloc")]
extern crate alloc;

mod options;
mod portal;
mod registry;
mod transport;

use ockam_core::TransportType;
pub use options::*;
pub use portal::*;
pub use registry::*;
pub use transport::*;

mod workers;
pub(crate) use workers::*;

pub(crate) const CLUSTER_NAME: &str = "_internals.transport.tcp";

/// Transport type for TCP addresses
pub const TCP: TransportType = TransportType::new(1);
