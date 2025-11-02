# ==== Variables ====
app_data_path="$HOME/.config/hoyoplay-umu"
wine_prefix="$app_data_path/wine_prefix"
desktop_file_path="$HOME/.local/share/applications"
hoyoplay_setup_path="$app_data_path/Hoyoplay_setup.exe"
umu_config_path="/usr/share/hoyoplay_umu_config"
icon_path="/usr/share/icons/hicolor/256x256/apps"
localfix_path="$HOME/.config/protonfixes/localfixes"
# ===================


# ==== Remove Old Data ====
rm -rf "$app_data_path"
sudo rm -rf "$umu_config_path"
rm -f "$desktop_file_path/Hoyoplay.desktop"
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
./scripts/kill_genshin.sh
#./scripts/valve_proton_downloader_and_updater.sh - Currently Not Working
./scripts/proton_ge_downloader_and_updater.sh
./scripts/download_hoyoplay.sh
# =================


# ==== Copy Necessary Data ====
cp "$PWD/assets/Hoyoplay.desktop" "$desktop_file_path"
sudo cp "$PWD/assets/umu_config.toml" "$umu_config_path/umu_config.toml"
# ============================


# ==== Setup Winetricks ====
umu-run winetricks -f vcrun2019 dxvk
#===========================


# ==== Setup ProtonFixes ====
curl -L "https://raw.githubusercontent.com/Open-Wine-Components/umu-protonfixes/master/gamefixes-umu/umu-genshin.py" -o "$localfix_path/umu-genshin.py"
#============================


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


echo "All Done!!! :) Yayy"
