cd C:/Users/Quang/repos/quangmire.github.io/build
jekyll build
robocopy "C:\Users\Quang\repos\quangmire.github.io\build\_site" "C:\Users\Quang\repos\quangmire.github.io" "/s"
cd C:/Users/Quang/repos/quangmire.github.io
eval $(ssh-agent -s)
git checkout
git add -A
git commit -am "Build blog."
git push origin
read -p "Press [Enter] key to start backup..."