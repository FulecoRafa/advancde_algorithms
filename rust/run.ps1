param ([Parameter(Mandatory)][string]$aula, $programa)

write-host $(Test-Path -Path "..\exercises\$aula\$programa.in" -PathType Leaf)

if (Test-Path -Path "..\exercises\$aula\$programa.in" -PathType Leaf) {
    gc ..\exercises\$aula\$programa.in | cargo run --bin "$($aula)_$programa"
} else {
    cargo run --bin "$($aula)_$programa"
}