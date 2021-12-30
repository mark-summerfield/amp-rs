cargo build --release -- %*
copy /Y target\release\amp-rs.exe amp.exe
rcedit amp.exe --set-icon images\amp.ico
copy /Y amp.exe C:\bin
