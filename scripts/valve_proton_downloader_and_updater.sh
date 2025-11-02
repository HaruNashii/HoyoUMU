# ==== Variables ====
tmp_working_directory="/tmp/hoyo_proton"
proton_version_tar_file=$(curl -s https://api.github.com/repos/Kron4ek/proton-archive/releases/latest | grep -oP '"name": "\K[^"]+' | grep -m 1 '.tar.xz')
proton_version=$(echo $proton_version_tar_file | sed 's/.tar.xz$//')
proton_folder="$HOME/.config/hoyoplay-umu"
file_to_download="$(curl -s https://api.github.com/repos/Kron4ek/proton-archive/releases/latest | grep browser_download_url | cut -d\" -f4 | grep .tar.xz)"
#====================

# ==== Create Directory ====
mkdir -p $tmp_working_directory
mkdir -p $proton_folder
#===========================

cd $tmp_working_directory

# ==== Download Proton ====
if [ ! -d "$proton_folder/$proton_version" ]; then
	if [ ! -d "$proton_folder/ProtonLatest" ]; then
		if [ ! -f "$tmp_working_directory/$proton_version_tar_file" ]; then
			echo "Downloading $proton_version, Please Wait..."
			curl -sLOJ $file_to_download
		fi 
	fi
fi
#============================

clear

if [ -d "$proton_folder/ProtonLatest" ]; then
    echo "Your Proton Is Already The Latest Version."
else
	if [ -d "$proton_folder/$proton_version" ]; then 
		# ==== Rename Proton_(Version) To ProtonLatest ====
		echo "Renaming $proton_version To ProtonLatest"
		mv "$proton_folder/$proton_version" "$proton_folder/ProtonLatest"
		clear
		echo "$proton_version Installed!"
		#=========================================================
	else
		# ==== Extract Proton And Rename ====
		echo "Extracting Proton To Proton Folder..."
		tar -xf proton*.tar.xz -C $proton_folder
		clear
		echo "Renaming $proton_version To ProtonLatest"
		mv "$proton_folder/$proton_version" "$proton_folder/ProtonLatest"
		clear
		echo "$proton_version Installed!"
		#=======================================
	fi
fi

# ==== Remove Temporary Directory ====
rm -rf $tmp_working_directory
#=====================================

# ==== Return Home ====
cd $HOME
#======================

