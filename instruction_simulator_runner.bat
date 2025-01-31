@echo off
SETLOCAL
set DEFMT_LOG=info
if exist Sim.traceinstr del Sim.traceinstr
REM TSIM tracing disable faster execution
tsim16p_e.exe -tc162p -config_file_path tsim-config-tc162  -s  -x 565600000 -H -o %1 > sim.out 2>error.txt
REM TSIM Tracing enabled
REM .\TC_MODELS_1.18.196\bin\Win64\tsim16p_e.exe -tc162p -config_file_path tsim-config-tc162  -s -U  -x 565600000 -e -H -o %1 > sim.out 2>error.txt
set _LOCALERROR=%ERRORLEVEL%
type sim.out | defmt-print.exe -v -e %1
del memory_map.txt
rustfilt -i memory.map -o memory_map.txt
if exist Sim_traceinstr.txt del Sim_traceinstr.txt
if exist Sim.traceinstr rustfilt -i Sim.traceinstr -o Sim_traceinstr.txt
EXIT %_LOCALERROR%
ENDLOCAL