#!/bin/bash
# OmaTunes volume control using playerctl

# Increase or decrease OmaTunes volume by 5%
if [ "$1" == "up" ]; then
    playerctl --player=omatunes volume 0.05+  # increase 5%
elif [ "$1" == "down" ]; then
    playerctl --player=omatunes volume 0.05-  # decrease 5%
fi
