#!/bin/bash
declare -a curves=(
    'bls12_377'
    'bls12_381'
    'bn254'
    'bw6_761'
)


declare -a exs=(
    'add'
    'mul'
    'pairing_two'
)


function bm() {
    target/release/jupiter-dev \
        benchmark \
        -p 'pallet_template' \
        -e "$1" "${@:2}" | grep 'Time ~=' -m 1
}


function ops() {
    for p in "${curves[@]}"
    do
        echo "Curve: ${p}"
        echo '=========================µs'
        for q in "${exs[@]}"
        do
            echo "--->${q}"
            printf "Wasm:   "
            d=wasm_"${p}"_"${q}"
            bm "$d" '--execution=Wasm' '--wasm-execution=Compiled' '-r=3'
            printf "RuIn:   "
            d=native_"${p}"_"${q}"
            bm "$d" '--execution=Wasm' '--wasm-execution=Compiled' '-r=3'
        done
        echo ""
        echo ""
    done
}


function groth16() {
    echo '=========================µs'
    for p in "${curves[@]}"
    do
        echo "---> ${p}"
        bm "${p}"_verify '--execution=Wasm' '--wasm-execution=Compiled' '-r=3'
    done
}

function verify() {
    echo "Wasm groth16 verification:"
    JIO_MODE=WASM groth16

    printf "\n\n"

    echo "NATIVE groth16 verification:"
    JIO_MODE=NATIVE groth16
}

function main() {
    case $1 in
        groth16)
            verify
            ;;
        *)
            ops
            ;;
    esac

}

main $@
