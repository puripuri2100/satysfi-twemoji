DIR=emoji_images/twemoji_svg

for pathfile in $DIR/*.svg; do
    echo $pathfile >> list.txt
done