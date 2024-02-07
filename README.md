

<img align="right" width="26%" src="./resources/LOGO.png">

lule
===

A command line tool to generate 8bit ANSI colors from wallpaper (an enhanced version of pywal but in rust)
There is the old bash version in: https://github.com/warpwm/lule_bash

```
lule create -- set
```
<hr>

![](./resources/a_gif.gif)

<hr>

## Features

In order for lule to work properly, you need to set the following environment variables:
- `LULE_W` : The path to the wallpaper
- `LULE_S` : The path to the script that will be run after the colors are generated
(please check the `scripts/lule_colors.sh` file for an example)

```
export LULE_W="~/.wallpaper"
export LULE_S="~/.func/lule_colors.sh"
export LULE_C="/home/bresilla/.config/lule/configs.json"

lule create -- set
```