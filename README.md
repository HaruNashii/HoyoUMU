# HoyoUMU
HoyoUMU Is An Simple Script That Download And Setup HoyoPlay Using UMU

> [!WARNING]
> You will need "umu-run" installed in the "/usr/bin" to run this script

# what this scripts does:
- Install the latest version of the Proton-GE
- Download The Latest Version of the HoyoPlay
- Download The Latest Version Of The HoyoPlay Icon
- Create An .desktop File Of The HoyoPlay
- Configure UMU With The genshin-umu GAMEID

# Dependencies:
- [UMU (Installed In "/usr/bin")](https://github.com/Open-Wine-Components/umu-launcher)
- curl (Already Installed on most distributions)
- tar (Already Installed on most distributions)

# How To Install:
Clone the repo and go to the repo folder
```git clone https://github.com/HaruNashii/HoyoUMU.git && cd HoyoUMU```
Give Permission To The Script To run and run it
```chmod a+x install.sh && ./install.sh```

# where things are installed:
- HoyoPlay Icon are installed in: "/usr/share/icons/hicolor/256x256/apps/hoyoplay_icon.png"
- HoyoPlay are installed in: "/home/{your_user}/.config/hoyoplay-umu/wine_prefix/drive_c/Program Files/HoyoPlay/launcher.exe"
- HoyoPlay Setup are installed in: "/home/{your_user}/.config/hoyoplay-umu/Hoyoplay_setup.exe"
- Proton-GE are installed in: "/home/{your_user}/.config/hoyoplay-umu/Proton-GE_Latest"
- Desktop File are installed in: "/home/{your_user}/.local/share/applications/Hoyoplay.desktop"
- UMU Config File are installed in: "/usr/share/hoyoplay_umu_config/umu_config.toml"

# Todo:
- Enable FeralGamemode

# Credits:
- [UMU](https://github.com/Open-Wine-Components/umu-launcher)
- [Proton-GE](https://github.com/GloriousEggroll/proton-ge-custom)
