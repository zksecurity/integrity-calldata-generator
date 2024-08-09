#!/usr/bin/env bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <job_id>"
    exit 1
fi

send_transaction() {
    sncast \
        --wait \
        invoke \
        --contract-address "$(<calldata/contract_address)" \
        --function "$1" \
        --calldata "$3 $(<$2)"
}

echo "Sending verify_proof_initial"
send_transaction "verify_proof_initial" "calldata/initial" $1

job_id=$1
i=1
while true; do
    filename="calldata/step${i}"

    if [[ -e "$filename" ]]; then
        echo "Sending verify_proof_step (${i})"
        send_transaction "verify_proof_step" "$filename" "$job_id"
    else
        break
    fi

    ((i++))
done

echo "Sending verify_proof_final"
send_transaction "verify_proof_final" "calldata/final" $1
