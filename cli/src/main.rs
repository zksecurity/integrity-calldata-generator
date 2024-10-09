pub mod transform;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;
pub use swiftness_proof_parser::parse;
pub use swiftness_stark::config::StarkConfig;
use starknet_crypto::Felt;
use itertools::Itertools;

use swiftness_air::layout::dex::Layout as LayoutDex;
use swiftness_air::layout::recursive::Layout as LayoutRecursive;
use swiftness_air::layout::recursive_with_poseidon::Layout as LayoutRecursiveWithPoseidon;
use swiftness_air::layout::small::Layout as LayoutSmall;
use swiftness_air::layout::starknet::Layout as LayoutStarknet;
use swiftness_air::layout::starknet_with_keccak::Layout as LayoutStarknetWithKeccak;

use swiftness_air::public_memory::STONE_6_ENABLED;
use swiftness_fri::fri::{CONST_STATE, VAR_STATE, WITNESS};
use swiftness_commitment::table::decommit::{HASHER_BLAKE2S, HASHER_248_LSB};
use swiftness_stark::stark::Error;
use swiftness_stark::types::StarkProof;

mod serialize;
use crate::serialize::serialize;
use swiftness::transform_stark::TransformTo;
use std::fs::write;
use std::format;


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[clap(rename_all = "snake_case")]
enum Layout {
    Dex,
    Recursive,
    RecursiveWithPoseidon,
    Small,
    Starknet,
    StarknetWithKeccak,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum StoneVersion {
    Stone5,
    Stone6,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Hasher {
    #[clap(name = "keccak_160_lsb")]
    Keccak160Lsb,
    #[clap(name = "keccak_248_lsb")]
    Keccak248Lsb,
    #[clap(name = "blake2s_160_lsb")]
    Blake2s160Lsb,
    #[clap(name = "blake2s_248_lsb")]
    Blake2s248Lsb,
}

#[derive(Parser)]
#[command(author, version, about)]
struct CairoVMVerifier {
    /// Path to proof JSON file
    #[clap(short, long)]
    proof: PathBuf,

    /// Output directory for the generated files
    #[clap(short, long, default_value = "calldata")]
    out: PathBuf,

    /// Layout
    #[clap(short, long)]
    layout: Layout,

    /// Hasher
    #[clap(short, long)]
    hasher: Hasher,

    /// Stone version
    #[clap(short, long)]
    stone_version: StoneVersion,
}

fn verify_layout(
    layout: Layout,
    stark_proof: StarkProof,
    security_bits: Felt
) -> Result<(Felt, Felt), Error> {
    match layout {
        Layout::Dex => stark_proof.verify::<LayoutDex>(security_bits),
        Layout::Recursive => stark_proof.verify::<LayoutRecursive>(security_bits),
        Layout::RecursiveWithPoseidon => stark_proof.verify::<LayoutRecursiveWithPoseidon>(security_bits),
        Layout::Small => stark_proof.verify::<LayoutSmall>(security_bits),
        Layout::Starknet => stark_proof.verify::<LayoutStarknet>(security_bits),
        Layout::StarknetWithKeccak => stark_proof.verify::<LayoutStarknetWithKeccak>(security_bits),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CairoVMVerifier::parse();
    let input = std::fs::read_to_string(cli.proof)?;
    let stark_proof = parse(input.clone())?.transform_to();
    let security_bits = stark_proof.config.security_bits();
    let _result = verify_layout(cli.layout, stark_proof, security_bits)?;
    unsafe {
        STONE_6_ENABLED = cli.stone_version == StoneVersion::Stone6;
        HASHER_BLAKE2S = cli.hasher == Hasher::Blake2s160Lsb || cli.hasher == Hasher::Blake2s248Lsb;
        HASHER_248_LSB = cli.hasher == Hasher::Keccak248Lsb || cli.hasher == Hasher::Blake2s248Lsb;
    };

    let (const_state, mut var_state, mut witness) = unsafe {
        (CONST_STATE.clone(), VAR_STATE.clone(), WITNESS.clone())
    };
    let initial = serialize(input)?
        .split_whitespace()
        .map(|s| Felt::from_dec_str(s).unwrap().to_hex_string())
        .join(" ");

    let final_ = format!(
        "{} {} {}",
        const_state,
        var_state.pop().unwrap(),
        witness.pop().unwrap()
    );

    write(cli.out.join("initial"), initial)?;
    write(cli.out.join("final"), final_)?;

    for (i, (v, w)) in var_state.iter().zip(witness.iter()).enumerate() {
        write(
            cli.out.join(format!("step{}", i + 1)),
            format!("{} {} {}", const_state, v, w)
        )?;
    }
    print!("{}", var_state.len());
    Ok(())
}
