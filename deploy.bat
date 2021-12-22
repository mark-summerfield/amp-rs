cargo build --release -- %*
copy /Y target\release\amp.exe .
rcedit amp.exe --set-icon images\amp.ico
copy /Y amp.exe C:\bin
