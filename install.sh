# ==== Variables ====
app_data_path="$HOME/.config/hoyoplay-umu"
wine_prefix="$app_data_path/wine_prefix"
desktop_file_path="$HOME/.local/share/applications"
hoyoplay_setup_path="$app_data_path/Hoyoplay_setup.exe"
umu_config_path="/usr/share/hoyoplay_umu_config"
icon_path="/usr/share/icons/hicolor/256x256/apps"
localfix_path="$HOME/.config/protonfixes/localfixes"
# ===================


# ==== Check if an old installation exist ====
if [ -d "$app_data_path" ]; then
	while true; do
		read -rp "WARNING!!!: Detected an installation of this launcher, if you proceed all your games and launcher data will be deleted, you want to continue? [y/n]: " answer
		case "$answer" in
			[Yy]* )
				break
				;;
			[Nn]* )
				clear
				echo "Okay, Exited without removing any data"
				exit 0
				;;
			* )
				echo "❌ Invalid input. Please enter 'y' or 'n'."
				;;
		esac
	done
fi
# ===========================================


# ==== Remove Old Data ====
rm -rf "$app_data_path"
sudo rm -rf "$umu_config_path"
rm -f "$desktop_file_path/Hoyoplay.desktop"
rm -f "$localfix_path/hoyoumu.py"
sudo rm -f "$icon_path/hoyoplay_icon.png"
# =========================


# ==== Create Directory ====
mkdir -p $app_data_path
mkdir -p $wine_prefix
mkdir -p $desktop_file_path
mkdir -p $localfix_path
sudo mkdir -p $umu_config_path
sudo mkdir -p $icon_path
# =========================


# ==== Scripts ====
./scripts/check_umu.sh
./scripts/kill_genshin.sh
#./scripts/valve_proton_downloader_and_updater.sh - Currently Not Working
./scripts/proton_ge_downloader_and_updater.sh
./scripts/download_hoyoplay.sh
./scripts/gamemode.sh
# =================


# ==== Copy Necessary Data ====
sudo cp "$PWD/assets/umu_config.toml" "$umu_config_path/umu_config.toml"
sudo cp "$PWD/assets/hoyoumu.py" "$localfix_path/hoyoumu.py"
# ============================


# ==== Setup Winetricks ====
echo "Setting Up WineTricks, Please Wait..."
umu-run winetricks -f vcrun2019 dxvk > /dev/null 2>&1
#===========================


# ==== Run HoyoPlay Setup ====
if [ ! -f "$wine_prefix/drive_c/Program Files/HoYoPlay/launcher.exe" ]; then
	chmod a+rwx $hoyoplay_setup_path
	export WINEPREFIX="$wine_prefix"
	export GAMEID="umu-genshin"
	export PROTONPATH="$app_data_path/ProtonLatest"
	echo "Running HoyoPlay Setup... (Please Don't Select An Different Path In The Launcher Installation!!!!!)"
	umu-run "$hoyoplay_setup_path" > /dev/null 2>&1
else
	echo "HoyoPlay Already Installed."
fi
# ============================

clear
echo "All Done!!! :) Yayy"
