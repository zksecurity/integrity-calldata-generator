1. Generate split calldata

```
cd cli
```

```
cargo run --release --bin swiftness --features recursive,keccak --no-default-features -- --proof ../examples/proofs/recursive/cairo0_example_proof.json
```

2. (optional) Configure verifier address

You can modify verifier address in `cli/calldata/contract_address` file.

3. Modify starknet account in `snfoundry.toml`

4. Send verification transactions

```
./verify.sh <job_id> <layout> <hasher> <security_bits> <version>
```

`job_id` is a unique identified of the verification. You can randomize it or pass any arbitrary value as long as it's not used by any other user.

All possible configurations of `layout`, `hasher`, `security_bits` and `version` can be found in [VerifierRegistered events](https://sepolia.voyager.online/contract/0x038f56b1cc89f907a5ddd9343b83bd97b85366c86b99f156f5ca28fb6494ba83#events)
