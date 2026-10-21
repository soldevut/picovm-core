//! PicoVM · Sub-atomic WebAssembly runtime microkernel for autonomous AI agents on Solana.
//!
//! # Architecture
//! - 140KB micro-kernel memory footprint
//! - Microsecond instantiation latency (< 420µs)
//! - In-memory SQLite transaction journaling
//! - Embedded Solana RPC & CPI signer

pub mod runtime;
pub mod journal;
pub mod solana;

pub use runtime::{PicoRuntime, SandboxConfig, ExecutionResult};
pub use journal::MemoryJournal;
pub use solana::SolanaContext;

/// PicoVM version string
pub const PICOVM_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DEFAULT_GAS_LIMIT: u64 = 1_000_000;
pub const MAX_LINEAR_MEMORY_PAGES: u32 = 256; // 16 MB
