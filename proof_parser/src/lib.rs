mod annotations;
mod ast;
mod builtins;
mod conversion;
mod json_parser;
mod layout;
mod stark_proof;
mod utils;

pub use crate::ast::{Expr, Exprs};

use crate::{json_parser::ProofJSON, stark_proof::StarkProof};
use std::convert::TryFrom;
extern crate clap;
extern crate num_bigint;
extern crate regex;
extern crate serde;
use swiftness_stark::types::StarkProof as StarkProofFromVerifier;

pub struct ParseStarkProof {
    pub config: Exprs,
    pub public_input: Exprs,
    pub unsent_commitment: Exprs,
    pub witness: Exprs,
}

pub fn parse(input: String) -> anyhow::Result<StarkProofFromVerifier> {
    let proof_json = serde_json::from_str::<ProofJSON>(&input)?;
    let stark_proof = StarkProof::try_from(proof_json)?;
    let stark_proof_verifier: StarkProofFromVerifier = stark_proof.into();
    Ok(stark_proof_verifier)
}

pub fn parse_as_exprs(input: String) -> anyhow::Result<ParseStarkProof> {
    let proof_json = serde_json::from_str::<ProofJSON>(&input)?;
    let stark_proof = StarkProof::try_from(proof_json)?;
    Ok(ParseStarkProof {
        config: Exprs::from(stark_proof.config),
        public_input: Exprs::from(stark_proof.public_input),
        unsent_commitment: Exprs::from(stark_proof.unsent_commitment),
        witness: Exprs::from(stark_proof.witness),
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("../../examples/proofs/recursive/cairo0_example_proof.json");
        let proof_json = serde_json::from_str::<ProofJSON>(input).unwrap();
        let stark_proof = StarkProof::try_from(proof_json).unwrap();
        let _: StarkProofFromVerifier = stark_proof.into();
    }

    #[test]
    fn test_parse_as_exprs() {
        let input = include_str!("../../examples/proofs/recursive/cairo0_example_proof.json");
        let _: ParseStarkProof = parse_as_exprs(input.to_string()).unwrap();
    }
}
