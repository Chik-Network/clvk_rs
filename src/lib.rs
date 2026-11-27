pub mod allocator;
pub mod bls_ops;
pub mod chik_dialect;
pub mod core_ops;
pub mod cost;
#[cfg(feature = "pre-eval")]
pub mod diagnostics;
pub mod dialect;
pub mod error;
pub mod f_table;
pub mod keccak256_ops;
pub mod more_ops;
pub mod number;
pub mod op_utils;
pub mod reduction;
pub mod run_program;
pub mod runtime_dialect;
pub mod secp_ops;
pub mod serde;
pub mod serde_2026;
pub mod sha_tree_op;
pub mod traverse_path;
pub mod treehash;

pub use allocator::{Allocator, Atom, NodePtr, ObjectType, SExp};
pub use chik_dialect::ChikDialect;
pub use run_program::{run_program, run_program_with_timeout};

pub use chik_dialect::{ClvkFlags, MEMPOOL_MODE};

#[cfg(feature = "counters")]
pub use run_program::run_program_with_counters;

#[cfg(feature = "pre-eval")]
pub use run_program::run_program_with_pre_eval;

#[cfg(feature = "pre-eval")]
pub use diagnostics::{EvalFailure, EvalFrame, run_program_with_diagnostics};

#[cfg(feature = "counters")]
pub use run_program::Counters;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_ops;
