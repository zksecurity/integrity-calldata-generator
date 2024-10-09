use swiftness_proof_parser::parse;
use itertools::chain;
use swiftness::vec252::VecFelt252;
use cairo_felt::Felt252;
use num_traits::Num;
use crate::transform::{Expr, StarkProofExprs};

pub fn serialize(input: String) -> anyhow::Result<String> {
    let mut parsed: StarkProofExprs = parse(input)?.into();

    let config: VecFelt252 = serde_json::from_str(&parsed.config.to_string()).unwrap();
    let public_input: VecFelt252 = serde_json::from_str(&parsed.public_input.to_string()).unwrap();
    let unsent_commitment: VecFelt252 =
        serde_json::from_str(&parsed.unsent_commitment.to_string()).unwrap();


    let fri_witness = match parsed.witness.0.pop().unwrap() {
        Expr::Array(witness) => { witness },
        _ => panic!("Expected witness to be an array"),
    };
    let mut fri_layers = vec![];
    let mut i = Felt252::from(0);
    let mut reach_0_count = 2;
    fri_witness.into_iter().for_each(|elem| {
        let elem = match elem {
            Expr::Value(s) => <Felt252 as Num>::from_str_radix(s.as_str(), 10).unwrap(),
            _ => panic!("Expected value"),
        };
        if i == Felt252::from(0) {
            if reach_0_count == 2 {
                fri_layers.push(vec![]);
                reach_0_count = 0;
            }
            reach_0_count += 1;

            i = elem.clone();
        } else {
            i -= Felt252::from(1);
        }
        fri_layers.last_mut().unwrap().push(elem);
    });

    // let fri_witness_parsed: VecFelt252 = serde_json::from_str(&fri_witness.to_string()).unwrap();
    parsed.witness.0.push(Expr::Array(vec![]));
    let witness: VecFelt252 = serde_json::from_str(&parsed.witness.to_string()).unwrap();

    let calldata = chain!(
        config.into_iter(),
        public_input.into_iter(),
        unsent_commitment.into_iter(),
        witness.into_iter()
    );

    let calldata_string = calldata
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    Ok(calldata_string)
}
