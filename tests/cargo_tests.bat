@echo off
setlocal enabledelayedexpansion

REM Set the directory containing the .rs files
set "rs_dir=%~dp0"

REM Set the output file
set "output_file=%~dp0cargo.out"

REM Initialize the output file
echo [tests] > "%output_file%"

REM Loop through each .rs file in the directory
for %%f in ("%rs_dir%*.rs") do (
    set "filename=%%~nf"
    echo [[test]] >> "%output_file%"
    echo name = "!filename!" >> "%output_file%"
    echo harness = false >> "%output_file%"
    REM print newline
    echo. >> "%output_file%"

)

echo Tests have been generated in %output_file%