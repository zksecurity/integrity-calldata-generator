1. Generate split calldata

```
cd cli
```

Make sure that there is no outputted files in `calldata` directory

```
rm -f calldata/final calldata/initial calldata/step* calldata/full
```

```
cargo run --release --bin swiftness -- --layout recursive --hasher keccak_160_lsb --stone-version stone5 --proof ../examples/proofs/recursive/cairo0_stone5_example_proof.json
```

2. (optional) Configure verifier address

You can modify verifier address in `cli/calldata/contract_address` file.

3. Modify starknet account in `snfoundry.toml`

4. Send verification transactions

```
./verify.sh <job_id> <layout> <hasher> <stone_version> <memory_verification>
```

For example

```
./verify.sh 0 recursive keccak_160_lsb stone5 cairo0
```

`job_id` is a unique identified of the verification. You can randomize it or pass any arbitrary value as long as it's not used by any other user.
