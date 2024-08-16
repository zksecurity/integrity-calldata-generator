#!/usr/bin/env bash

if [ $# -ne 5 ]; then
    echo "Usage: $0 <job_id> <layout> <hasher> <security_bits> <version>"
    exit 1
fi

arg_1=$1
arg_2=$2
arg_3=$3
arg_4=$4
arg_5=$5

send_transaction() {
    local retries=5
    local count=0
    local status=0

    while [[ $count -lt $retries ]]; do
        sncast \
            --wait \
            invoke \
            --contract-address "$(<calldata/contract_address)" \
            --function "$1" \
            --calldata "$arg_1 $(<$2) $arg_2 $arg_3 $arg_4 $arg_5"

        sleep 5 # extra delay to make sure the transaction is registered

        status=$?

        if [[ $status -eq 0 ]]; then
            return 0
        else
            echo "Transaction failed with status $status. Retrying... ($((count + 1))/$retries)"
        fi

        count=$((count + 1))
    done

    echo "Transaction failed after $retries attempts."
    return $status
}

echo ""
echo "Sending verify_proof_initial"
send_transaction "verify_proof_initial" "calldata/initial"

i=1
while true; do
    filename="calldata/step${i}"

    if [[ -e "$filename" ]]; then
        echo ""
        echo "Sending verify_proof_step (${i})"
        send_transaction "verify_proof_step" "$filename"
    else
        break
    fi

    ((i++))
done

echo ""
echo "Sending verify_proof_final_and_register_fact"
send_transaction "verify_proof_final_and_register_fact" "calldata/final"
