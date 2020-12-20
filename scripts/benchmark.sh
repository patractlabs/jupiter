#!/bin/bash
declare -a curves=(
    'bls12_377'
    'bls12_381'
    'bn254'
    'bw6_761'
    'cp6_782'
)

declare -a exs=(
    'add'
    'mul'
    'pairing_two'
    'pairing_six'
    'verify'
)

function bm() {
    ../target/release/node-template \
        benchmark \
        -p 'pallet_template' \
        -e "$1" "${@:2}" | grep 'Time ~=' -m 1
}


function main() {
    for p in "${curves[@]}"
    do
        echo "Curve: ${p}"
        echo '=========================µs'
        for q in "${exs[@]}"
        do
            echo "--->${q}"
            echo -n "Wasm:   "
            d=wasm_"${p}"_"${q}"
            bm "$d" '--execution=Wasm' '--wasm-execution=Interpreted' "-r=1000" # '|' 'grep' "'Time ~='" '-m' '1'

            # "RuIn" means the `runtime_interface` implementation in test
            echo -n "RuIn:   "
            d=native_"${p}"_"${q}"
            bm "$d" '--execution=Wasm' '--wasm-execution=Interpreted' "-r=1000" # '|' 'grep' "'Time ~='" '-m' '1'
#            bm wasm_"${p}"_"${q}" --execution Native | grep 'Time ~=' -m 1
        done
        echo ""
        echo ""
    done
}

main $@
