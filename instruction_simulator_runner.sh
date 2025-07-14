#!/bin/bash
# Allow script to continue even if a command fails
set +e

export DEFMT_LOG=info

[ -f Sim.traceinstr ] && rm Sim.traceinstr

# TSIM tracing disable faster execution
tsim16p_e -tc162p -config_file_path tsim-config-tc162 -s -x 565600000 -H -o "$1" > sim.out 2>error.txt

# TSIM Tracing enabled
# ./tsim16p_e.exe -tc162p -config_file_path tsim-config-tc162 -s -U -x 565600000 -e -H -o "$1" > sim.out 2>error.txt

local_error=$?

cat sim.out | defmt-print -v -e "$1"

[ -f memory_map.txt ] && rm memory_map.txt
rustfilt -i memory.map -o memory_map.txt

[ -f Sim_traceinstr.txt ] && rm Sim_traceinstr.txt
[ -f Sim.traceinstr ] && rustfilt -i Sim.traceinstr -o Sim_traceinstr.txt

exit $local_error