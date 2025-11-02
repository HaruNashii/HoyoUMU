# ==== Variables ====
app_data_path="$HOME/.config/hoyoplay-umu"
umu_config_path="/usr/share/hoyoplay_umu_config"
desktop_file_path="$HOME/.local/share/applications"
icon_path="/usr/share/icons/hicolor/256x256/apps"
# ===================

rm -rf "$app_data_path"
sudo rm -rf "$umu_config_path"
rm -f "$desktop_file_path/Hoyoplay.desktop"
sudo rm -f "$icon_path/hoyoplay_icon.png"

clear
echo "Everything removed :)"
