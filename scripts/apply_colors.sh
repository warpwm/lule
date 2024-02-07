#!/bin/bash

cp $TEMP/lule_colors $HOME/.cache/wal/colors
cp $TEMP/lule_theme $HOME/.cache/wal/theme
cp $TEMP/lule_wallpaper $HOME/.cache/wal/wallpaper

create_files(){
    ### === PRINTING THINGS TO FILES === ###
    wallpaper="none"
    theme="none"
    fore=$(sed -n '16p' $HOME/.cache/wal/colors)
    back=$(sed -n '1p' $HOME/.cache/wal/colors)
    col1=$(sed -n '2p' $HOME/.cache/wal/colors)
    # COLORS.SH (used by shells - bash,zsh,fish...)
    printf "foreground=\"$fore\"\nbackground=\"$back\"\ncursor=\"$col1\"\n" > $HOME/.cache/wal/colors.sh
    awk '{print "color" NR-1 "=\"" $0 "\""}' $HOME/.cache/wal/colors >> $HOME/.cache/wal/colors.sh

    # COLORS.JSON (used by FREFOX)
    printf "{\n\t\"wallpaper\": \"$wallpaper\",\n\t\"theme\": \"$theme\",\n" > $HOME/.cache/wal/colors.json
    printf "\t\"special\": {\n\t\t\"background\": \"$back\",\n\t\t\"foreground\": \"$fore\",\n\t\t\"cursor\": \"$col1\"\n\t},\n\t\"colors\": {\n" >> $HOME/.cache/wal/colors.json
    awk '{print "\t\t\"color" NR-1 "\": \"" $0 "\","}' $HOME/.cache/wal/colors >> $HOME/.cache/wal/colors.json
    truncate -s-2 $HOME/.cache/wal/colors.json; printf "\n\t}\n}" >> $HOME/.cache/wal/colors.json

    # COLORS.YAML (used by ALACRITY)
    # cat ~/.cache/wal/colors.json | jq | yq -y > $HOME/.cache/wal/colors.yml
    printf "special:\n  background: \"$back\"\n  foreground: \"$fore\"\n  cursor: \"$col1\"\n\ncolors:\n" > $HOME/.cache/wal/colors.yml
    awk '{print "  color" NR-1 ": \"" $0 "\""}' $HOME/.cache/wal/colors >> $HOME/.cache/wal/colors.yml

    # COLORS.INI (used by POLYBAR)
    printf "[colors]\n\tforeground=$fore\n\tbackground=$back\n\tcursor=$col1\n" > $HOME/.cache/wal/colors.ini
    awk '{print "\tcolor" NR-1 "=" $0 }' $HOME/.cache/wal/colors >> $HOME/.cache/wal/colors.ini
 
    # COLORS.SCSS (used for STYLING)
    printf "\$foreground: $fore;\n\$background: $back;\n\n" > $HOME/.cache/wal/colors.scss
    awk '{print "$color" NR-1 ": " $0 ";"}' $HOME/.cache/wal/colors >> $HOME/.cache/wal/colors.scss
 
    # COLORS.RASI (used for STYLING)
    printf "* {\n\tforeground: $fore;\n\tbackground: $back;\n" > $HOME/.cache/wal/colors.rasi
    awk '{print "\tcolor" NR-1 ": " $0 ";"}' $HOME/.cache/wal/colors >> $HOME/.cache/wal/colors.rasi
    printf "}" >> $HOME/.cache/wal/colors.rasi

    # COLORS.CONF (used by KITTY)
    printf "foreground\t$fore\nbackground\t$back\ncursor\t$col1\n\n" > $HOME/.cache/wal/colors.conf
    awk '{print "color" NR-1 "\t " $0}' $HOME/.cache/wal/colors >> $HOME/.cache/wal/colors.conf

    # COLORS_W.CONF (used by HYPR)
    printf "\$foreground=rgb(${fore//#})\n\$background=rgb(${back//#})\n\n" > $HOME/.cache/wal/colors_hypr.conf
    awk '{print "$color" NR-1 "=rgb("substr($0, 2)")"}' $HOME/.cache/wal/colors >> $HOME/.cache/wal/colors_hypr.conf

    # SEQUENCES (escape codes sent to all '/dev/pts/*')
    printf "]10;$fore\\]11;$back\\]12;$col1\\]13;$back\\]17;$col1\\]19;$back\\" > $HOME/.cache/wal/sequences
    awk '{print "]4;" NR-1 ";" $0"\\"}' ORS='' $HOME/.cache/wal/colors >> $HOME/.cache/wal/sequences

    #update logo colors
    sed -i "s/fill=\"#\([^\"]*\)\"/fill=\"$col1\"/" /home/bresilla/.config/bresilla.svg
}

create_files

#sent color sequences to all ppts (most important piece of this script)
for tt in /dev/pts/*; do
    re='^[0-9]+$'; [[ $(basename $tt) =~ $re ]] && cat $HOME/.cache/wal/sequences > $tt;
done