@_default:
    just --list

_build day profile:
    #!/usr/bin/env bash
    set -euo pipefail

    bin="day$(printf "%02d" "{{ day }}")"
    if [[ "{{ profile }}" == release ]]; then
        cargo build --release --bin "$bin"
    else
        cargo build --bin "$bin"
    fi
    echo "$PWD/target/{{ profile }}/$bin"

download day:
    #!/usr/bin/env bash
    set -euo pipefail

    mkdir -p .inputs
    if [[ ! -f .inputs/{{ day }}.txt ]]; then
        curl \
            -A "Private script by github.com/Felerius (david@david-stangl.com)" \
            -b "session={{ env_var("AOC_SESSION") }}" \
            "https://adventofcode.com/2023/day/{{ day }}/input" \
            > ".inputs/{{ day }}.txt"
    fi

@setup day:
    cp .template.rs "src/bin/day$(printf "%02d" "{{ day }}").rs"

@run day: (download day)
    "$(just _build "{{ day }}" debug)" < ".inputs/{{ day }}.txt"

@runr day: (download day)
    "$(just _build "{{ day }}" release)" < ".inputs/{{ day }}.txt"

@run-input day:
    pbpaste | "$(just _build "{{ day }}" debug)"

bench day: (download day)
    #!/usr/bin/env bash
    set -euo pipefail

    bin="$(just _build "{{ day }}" release)"
    hyperfine --warmup 100 --shell none --input '.inputs//{{ day }}.txt' "$bin"
