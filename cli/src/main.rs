use std::path::PathBuf;
pub use swiftness_proof_parser::*;
pub use swiftness_stark::*;

#[cfg(feature = "dex")]
use swiftness_air::layout::dex::Layout;
#[cfg(feature = "recursive")]
use swiftness_air::layout::recursive::Layout;
#[cfg(feature = "recursive_with_poseidon")]
use swiftness_air::layout::recursive_with_poseidon::Layout;
#[cfg(feature = "small")]
use swiftness_air::layout::small::Layout;
#[cfg(feature = "starknet")]
use swiftness_air::layout::starknet::Layout;
#[cfg(feature = "starknet_with_keccak")]
use swiftness_air::layout::starknet_with_keccak::Layout;

use swiftness_fri::fri::{CONST_STATE, VAR_STATE, WITNESS};

use clap::Parser;
use starknet_crypto::poseidon_hash_many;

mod serialize;
use crate::serialize::{serialize, CairoVersion};
use std::fs::write;
use std::format;


#[derive(Parser)]
#[command(author, version, about)]
struct CairoVMVerifier {
    /// Path to proof JSON file
    #[clap(short, long)]
    proof: PathBuf,

    /// Cairo version - public memory pattern
    #[clap(value_enum, short, long, default_value_t=CairoVersion::Cairo0)]
    cairo_version: CairoVersion,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CairoVMVerifier::parse();
    let input = std::fs::read_to_string(cli.proof)?;
    let stark_proof = parse(input.clone())?;
    let security_bits = stark_proof.config.security_bits();
    let result = stark_proof.verify::<Layout>(security_bits)?;

    let (const_state, var_state, witness) = unsafe {
        (CONST_STATE.clone(), VAR_STATE.clone(), WITNESS.clone())
    };
    let initial = serialize(input, cli.cairo_version.into())?;

    write("calldata/initial", initial)?;

    for (i, (v, w)) in var_state.iter().zip(witness.iter()).enumerate() {
        write(
            if i+1 == var_state.len() { format!("calldata/final") } else { format!("calldata/step{}", i+1) },
            format!("{} {} {}", const_state, v, w)
        )?;
    }
    Ok(())
}
