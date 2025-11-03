#!/bin/bash



# ==== Configuration ====
APP_NAME="gamemoderun"
desktop_file_path="$HOME/.local/share/applications"
# ========================

# Check if the app exists in PATH
if ! command -v "$APP_NAME" &> /dev/null; then
	cp "$PWD/assets/Hoyoplay.desktop" "$desktop_file_path"
else
	while true; do
		read -rp "Gamemode installation detected, would you like to enable gamemode in all of your future games? [y/n]: " answer
		case "$answer" in
			[Yy]* )
				cp "$PWD/assets/Hoyoplay_gamemode.desktop" "$desktop_file_path"
				echo "Gamemode configured in your launcher, enjoy!!! 0/"
				echo "Desktop file created"
				kill $$
				;;
			[Nn]* )
				cp "$PWD/assets/Hoyoplay.desktop" "$desktop_file_path"
				echo "Desktop file created"
				kill $$
				;;
			* )
				echo "❌ Invalid input. Please enter 'y' or 'n'."
				;;
		esac
	done
fi


