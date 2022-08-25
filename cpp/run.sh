aula=$1
programa=$2

# Check if variables are set
if [ -z "$aula" ] || [ -z "$programa" ]; then
    echo "Usage: $0 <aula> <programa>"
    exit 1
fi

# Flag if output is in file
while getopts "f" opt; do
    case $opt in
        f)
            output="file"
            ;;
        \?)
            echo "Invalid option: -$OPTARG" >&2
            exit 1
            ;;
    esac
done

# Compile program
g++ -Wall -Wextra -Werror -std=c++11 -o $aula/$programa.exe $aula/$programa.cpp

# Run program with input file if exists and output to file if flag is set
if [ -f $aula/$programa.in ]; then
    if [ "$output" = "file" ]; then
        ./$aula/$programa.exe < $aula/$programa.in > $aula/$programa.out
    else
        ./$aula/$programa.exe < $aula/$programa.in
    fi
else
    if [ "$output" = "file" ]; then
        ./$aula/$programa.exe > $aula/$programa.out
    else
        ./$aula/$programa.exe
    fi
fi
