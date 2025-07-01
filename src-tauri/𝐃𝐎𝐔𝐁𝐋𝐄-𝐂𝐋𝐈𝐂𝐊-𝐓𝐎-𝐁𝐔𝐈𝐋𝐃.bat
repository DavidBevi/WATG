@echo off
REM Change directory to the target folder (modifica qui la cartella)
cd /d "D:\watg\src-tauri"

REM Run the build command exactly like manual
cargo tauri build --features unstable

REM Pause to keep the console window open at the end
echo.
echo Press any key to close this window...
pause > nul
