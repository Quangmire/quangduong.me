cd C:\Users\Quang\repos\quangmire.github.io\build
call jekyll build
robocopy "C:\Users\Quang\repos\quangmire.github.io\build\_site" "C:\Users\Quang\repos\quangmire.github.io" /s
"C:\Program Files (x86)\Git\bin\sh.exe"
pause