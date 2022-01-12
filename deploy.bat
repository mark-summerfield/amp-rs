SET CARGO_TARGET_DIR=target
SET RELEASE_DIR=%CARGO_TARGET_DIR%\release
cargo build --release -- %*
copy /Y target\release\amp-rs.exe amp.exe
rcedit amp.exe --set-icon images\amp.ico
copy /Y amp.exe C:\bin
