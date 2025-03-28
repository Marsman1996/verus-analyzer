//! Basic enum/struct/fn for Verus Errors
//!
//! These are used to represent various errors from the verifier
//! There are three kinds: precondition Failure, postcondition failure, assertion failure
//!
//! For further reference, see `crates/rust-analyzer/verus_interaction`
//!

use text_edit::TextRange;

/// Verus Errors with three kinds: pre/post/assert
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum VerusError {
    Pre(PreFailure),
    Post(PostFailure),
    Assert(AssertFailure),
}

/// Precondition Failure contains
/// (1) the exact precondition that is failing
/// (2) the callsite that invoked this failure
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PreFailure {
    pub failing_pre: TextRange,
    pub callsite: TextRange,
}

/// Postcondition failure contains
/// (1) the exact postcondition that is failing
/// (2) the error span for the function name and mode from Verus
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PostFailure {
    pub failing_post: TextRange,
    pub func_name: TextRange,
}

/// Assertion failure contains
/// the asserted predicate
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AssertFailure {
    pub range: TextRange,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VerusQuantifier {
    pub exprs: Vec<TextRange>,
}

/// From a vector of VerusErrors,
/// filter only precondition failures
pub fn filter_pre_failuires(verus_errors: &Vec<VerusError>) -> Vec<PreFailure> {
    let mut pre_errs = vec![];
    for verr in verus_errors {
        if let VerusError::Pre(p) = verr {
            pre_errs.push(p.clone());
        }
    }
    pre_errs
}

/// From a vector of VerusErrors,
/// filter only postcondition failures
pub fn filter_post_failures(verus_errors: &Vec<VerusError>) -> Vec<PostFailure> {
    let mut post_errs = vec![];
    for verr in verus_errors {
        if let VerusError::Post(p) = verr {
            post_errs.push(p.clone());
        }
    }
    post_errs
}

/// just for writing testcases
#[cfg(test)]
pub fn mk_pre_failure(pre_start: u32, pre_end: u32, call_start: u32, call_end: u32) -> VerusError {
    VerusError::Pre(PreFailure {
        failing_pre: TextRange::new(pre_start.into(), pre_end.into()),
        callsite: TextRange::new(call_start.into(), call_end.into()),
    })
}
/// just for writing testcases
#[cfg(test)]
pub fn mk_post_failure(
    post_start: u32,
    post_end: u32,
    name_start: u32,
    name_end: u32,
) -> VerusError {
    VerusError::Post(PostFailure {
        failing_post: TextRange::new(post_start.into(), post_end.into()),
        func_name: TextRange::new(name_start.into(), name_end.into()),
    })
}
