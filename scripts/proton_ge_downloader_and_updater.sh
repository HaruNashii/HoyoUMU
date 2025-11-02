# ==== Variables ====
tmp_working_directory="/tmp/proton-ge-custom"
proton_ge_version_tar_file=$(curl -s https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases/latest | grep -oP '"name": "\K[^"]+' | grep .tar.gz)
proton_ge_version=$(echo $proton_ge_version_tar_file | sed 's/.tar.gz$//')
proton_folder="$HOME/.config/hoyoplay-umu"
file_to_download="$(curl -s https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases/latest | grep browser_download_url | cut -d\" -f4 | grep .tar.gz)"
#====================

# ==== Create Directory ====
mkdir -p $tmp_working_directory
mkdir -p $proton_folder
#===========================

cd $tmp_working_directory

# ==== Download Proton-GE ====
if [ ! -d "$proton_folder/$proton_ge_version" ]; then
	if [ ! -d "$proton_folder/Proton-GE_Latest" ]; then
		if [ ! -f "$tmp_working_directory/$proton_ge_version_tar_file" ]; then
			echo "Downloading $proton_ge_version, Please Wait..."
			curl -sLOJ $file_to_download
		fi 
	fi
fi
#============================

clear

if [ -d "$proton_folder/Proton-GE_Latest" ]; then
    echo "Your Proton-GE Is Already The Latest Version."
else
	if [ -d "$proton_folder/$proton_ge_version" ]; then 
		# ==== Rename Proton-GE_(Version) To Proton-GE_Latest ====
		echo "Renaming $proton_ge_version To Proton-GE_Latest"
		mv "$proton_folder/$proton_ge_version" "$proton_folder/Proton-GE_Latest"
		clear
		echo "$proton_ge_version Installed!"
		#=========================================================
	else
		# ==== Extract Proton-GE And Rename ====
		echo "Extracting Proton-GE To Proton Folder..."
		tar -xf GE-Proton*.tar.gz -C $proton_folder
		clear
		echo "Renaming $proton_ge_version To Proton-GE_Latest"
		mv "$proton_folder/$proton_ge_version" "$proton_folder/Proton-GE_Latest"
		clear
		echo "$proton_ge_version Installed!"
		#=======================================
	fi
fi

# ==== Remove Temporary Directory ====
rm -rf $tmp_working_directory
#=====================================

# ==== Return Home ====
cd $HOME
#======================
