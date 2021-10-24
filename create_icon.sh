#!/usr/bin/env bash
convert -background none icon.svg -define icon:auto-resize static/favicon.ico
exit
i=0
icons=""
for size in $(identify public/favicon.ico | awk '{print $3}'); do
    echo "convert public/favicon.ico[$i] -thumbnail $size -flatten icon$size.png"
    convert public/favicon.ico[$i] -thumbnail $size -flatten icon$size.png
    icons="$icons icon$size.png"
    i=$((i + 1))
    if [ "$i" -eq "9" ]; then
        convert $icons icon16.png public/favicon.ico
        break
    fi
done
rm $icons
