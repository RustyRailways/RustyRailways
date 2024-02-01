#!/bin/bash

# this is just a quick and dirty script to get the job done, TODO: remake this properly with checks and more scenarios covered!

SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
echo "[INFO] script directory: $SCRIPT_DIR"

path_to_godot_1=$(which godot)  # classic linux install
path_to_godot_2=$(which org.godotengine.Godot)  # flatpak linux install

if [ -x "$path_to_godot_1" ] ; then
    echo "[INFO] godot executable location: $path_to_godot_1"
    echo "[INFO] exporting/building for web..."
    godot --path $SCRIPT_DIR/../ --export-debug "web"
elif [ -x "$path_to_godot_2" ] ; then
    echo "[INFO] godot executable location: $path_to_godot_2"
    echo "[INFO] exporting/building for web..."
    org.godotengine.Godot --path $SCRIPT_DIR/../ --export-debug "web"
else
    echo "[CRITICAL] godot executable NOT FOUND! make sure to have '/var/lib/flatpak/exports/bin' in you PATH if you have installed Godot with flatpak! exiting..."
fi
