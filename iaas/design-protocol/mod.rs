// iaas/design-protocol/mod.rs

/// Design protocol for seamless communication
pub trait DesignProtocol {
    type Problem: ProblemDomain;
    type Solution: DesignSolution;
    type QA: QAMatrix;
    
    /// Define WHAT problem we're solving
    fn problem(&self) -> &Self::Problem;
    
    /// Define HOW we solve it (O(log n) required)
    fn solution(&self) -> &Self::Solution;
    
    /// QA matrices for isomorphic mapping
    fn qa_matrix(&self) -> &Self::QA;
    
    /// Verify HDIS topology compatibility
    fn verify_hdis(&self) -> Result<(), TopologyError>;
}

/// Problem domain with isomorphic mapping
pub trait ProblemDomain {
    /// Map problem to solution space
    fn map_to_solution(&self) -> SolutionSpace;
}

/// Design solution with O(log n) guarantee
pub trait DesignSolution {
    /// Get complexity bound
    fn complexity(&self) -> Complexity;
    
    /// Must be O(log n)
    fn validate_complexity(&self) -> Result<(), ComplexityViolation> {
        match self.complexity() {
            Complexity::LogN => Ok(()),
            other => Err(ComplexityViolation::Expected {
                expected: Complexity::LogN,
                actual: other,
            }),
        }
    }
}
