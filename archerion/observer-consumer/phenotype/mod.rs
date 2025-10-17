// archerion/observer-consumer/phenotype/mod.rs

/// Phenotype: Observable system state
pub trait Phenotype {
    type State;
    
    /// Capture observable state
    fn capture(&self) -> Self::State;
}

// archerion/observer-consumer/phenomemory/mod.rs

/// Phenomemory: Captured system memory with O(log n) guarantee
pub trait Phenomemory {
    type Memory;
    type Token; // Phenomemory token type
    
    /// Store captured state with O(log n) aux space
    fn store(&mut self, state: Self::Memory) -> Result<Self::Token, AuxSpaceViolation>;
    
    /// Retrieve with O(log n) lookup
    fn retrieve(&self, token: &Self::Token) -> Option<&Self::Memory>;
}

// archerion/observer-consumer/phenovalue/mod.rs

/// Phenovalue: Derived value from phenomemory
pub trait Phenovalue {
    type Value;
    
    /// Compute value with O(log n) complexity
    fn compute(&self, memory: &impl Phenomemory) -> Result<Self::Value, ComplexityViolation>;
}

// archerion/mod.rs - The complete pattern

use complexity_validator::ComplexityBound;

/// Archerion: Complete observer-consumer pattern
pub trait Archerion: ComplexityBound<LogN> {
    type P: Phenotype;
    type M: Phenomemory;
    type V: Phenovalue;
    
    /// Observe phenotype → capture phenomemory (O(log n))
    fn observe(&self, phenotype: Self::P) -> Result<Self::M, ObserveError>;
    
    /// Consume phenomemory → produce phenovalue (O(log n))
    fn consume(&self, memory: Self::M) -> Result<Self::V, ConsumeError>;
    
    /// Watch phenovalue → monitor HDIS topology (O(log n))
    fn watch(&self, value: Self::V) -> Result<(), SegmentFault>;
}
