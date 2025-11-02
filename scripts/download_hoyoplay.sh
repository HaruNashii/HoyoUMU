# ==== Variables ====
app_data_path="$HOME/.config/hoyoplay-umu"
hoyoplay_setup_path="$app_data_path/Hoyoplay_setup.exe"
icon_path="/usr/share/icons/hicolor/256x256/apps"
# ==================

# ==== Download HoyoPlay Setup ====
if [ ! -f "$hoyoplay_setup_path" ]; then
	echo "Downloading HoyoPlay Setup..."
	curl -sL https://sg-public-api.hoyoverse.com/event/download_porter/trace/hyp_global/hyphoyoverse/default --output "$hoyoplay_setup_path"
else 
	echo "Latest HoyoPLay Setup Version Already Downloaded!!!"
fi
# =================================

# ==== Download HoyoPlay Icon ====
if [ ! -f "$icon_path" ]; then
	echo "Downloading Icon..."
	sudo mkdir -p $icon_path
	sudo curl -sL https://act-webstatic.hoyoverse.com/puzzle/hyp/pz_Bur_m6Btc7/resource/puzzle/2024/10/21/b9992eaa38d4b36641accee82ede7bd3_3188319471762169697.png --output "$icon_path/hoyoplay_icon.png"
else
	echo "Icon Already Downloaded"
fi
# ===============================
