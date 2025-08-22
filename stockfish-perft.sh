#! /bin/sh

cat <<EOF | stockfish | grep -Eoe '^[a-h][1-8][a-h][1-8][nbrq]?: [0-9]+'
position startpos moves $1
go perft $2
EOF