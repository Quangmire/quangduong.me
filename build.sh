cd C:/Users/Quang/repos/quangmire.github.io/build
jekyll build
cp -rp "C:/Users/Quang/repos/quangmire.github.io/build/_site" "C:/Users/Quang/repos/quangmire.github.io"
cd C:/Users/Quang/repos/quangmire.github.io
eval $(ssh-agent -s)
git checkout
git add -A
git commit -am "Build blog."
git push origin
read -p "Press [Enter] key to start backup..."