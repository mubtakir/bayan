//! AlBayan Runtime - Logic Engine and Knowledge Base
//! Expert recommendation: Priority 2 - Build Logic Core

pub mod knowledge_base;
pub mod unification;
pub mod solver;
pub mod api;
pub mod ai;  // Expert recommendation: Priority 1 - AI Module
pub mod torch;
pub mod shape_inference;  // Expert recommendation: Priority 2 - PyTorch Training
pub mod knowledge_engine;  // Expert recommendation: Priority 2 - Semantic Knowledge Base
pub mod linguistic_intelligence;  // User specification: Letter-based word analysis system

pub use knowledge_base::*;
pub use unification::*;
pub use solver::*;
pub use api::*;
pub use ai::*;
pub use torch::*;
pub use knowledge_engine::*;
pub use linguistic_intelligence::*;

/// Runtime initialization
pub fn init_runtime() {
    // Initialize global knowledge base
    initialize_semantic_kb();
    // Initialize linguistic intelligence engine
    initialize_linguistic_intelligence();
    // This will be called from generated LLVM code
}

/// Runtime cleanup
pub fn cleanup_runtime() {
    // Cleanup global resources
}
