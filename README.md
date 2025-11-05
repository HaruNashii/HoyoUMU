# 🎮 HoyoUMU

**HoyoUMU** is a application built in rust that **downloads and sets up HoYoPlay using UMU and Proton-GE** automatically — no manual tinkering required.

---

<br/>
<br/>

## 🧪 Games Tested
- ✅ Genshin Impact (Runned Without Any Flaws)
- ✅ Zenless Zone Zero (Runned Without Any Flaws)
- ❌ Honkai Star Rail (Currently Borked/Not Working)
- ❌ Honkai Impact 3rd (Currently Borked Because of anti-cheat)

---

<br/>
<br/>

## 🧰 What This Script Does

✅ Installs the **latest version of Proton-GE**
<br/>
✅ Downloads the **latest version of HoYoPlay**
<br/>
✅ Fetches and installs the **official HoYoPlay icon**
<br/>
✅ Creates a working **.desktop launcher** for easy access
<br/>
✅ Configures **UMU** with my personal [hoyoumu.py](https://github.com/HaruNashii/HoyoUMU/blob/script_version/assets/hoyoumu.py) setup to guarantee an smooth adventure
<br/>
✅ Update **Proton-GE** to the latest version anytime
<br/>
✅ Completely **uninstall** without remaining any traces
<br/>
✅ Optionally runs all your games with **Feral Gamemode** for a very smooth experience

---

<br/>
<br/>

## 📦 Dependencies - Normal/Gui Version

| Dependency - Normal/Gui Version                                      | Description                                            | Default Availability                |
| -------------------------------------------------------------------- | ------------------------------------------------------ | ----------------------------------- |
| [UMU](https://github.com/Open-Wine-Components/umu-launcher)          | UMU is a unified launcher for Windows games on Linux   | Must be installed manually          |
| [Gamemode](https://github.com/FeralInteractive/gamemode/) [OPTIONAL] | Optimise Linux system performance on demand            | Must be installed manually          |
| [Cantarell](https://fonts.google.com/specimen/Cantarell) [OPTIONAL]  | An very modern and sleek font used by default on GNOME | Must be installed manually          |


| Dependency - Bash/Shell Script Version                               | Description                                            | Default Availability                |
| -------------------------------------------------------------------- | ------------------------------------------------------ | ----------------------------------- |
| [UMU](https://github.com/Open-Wine-Components/umu-launcher)          | UMU is a unified launcher for Windows games on Linux   | Must be installed manually          |
| [Gamemode](https://github.com/FeralInteractive/gamemode/) [OPTIONAL] | Optimise Linux system performance on demand            | Must be installed manually          |
| `curl`                                                               | Fetches downloads                                      | Pre-installed on most Linux distros |
| `tar`                                                                | Extracts Proton-GE archives                            | Pre-installed on most Linux distros |

---

<br/>
<br/>

## 🚀 Installation
1. Download the Latest Version In The [Releases Page](https://github.com/HaruNashii/HoyoUMU/releases)
2. Give It Permission To Run With: ```chmod a+x rwx /path/to/application"``` (Replace "/path/to/application", with the application location, probabily like: /home/username/Downloads/application_here)
2. Run The App And Enjoy :).

or if your prefer download with just one command, you can just run the following commands to download HoyoUMU as an bash script: 
```bash
git clone --branch script_version --single-branch https://github.com/HaruNashii/HoyoUMU.git && cd HoyoUMU && chmod a+x install.sh && ./install.sh
```


---

<br/>
<br/>

## 📁 Installation Paths

| Component               | Path                                                                             |
| ----------------------- | -------------------------------------------------------------------------------- |
| **HoYoPlay Icon**       | `~/.config/hoyoplay-umu/assets/hoyoplay_icon.png`                                |
| **HoYoPlay Games**      | `~/.config/hoyoplay-umu/wine_prefix/drive_c/Program Files/HoYoPlay/games/`       |
| **HoYoPlay Executable** | `~/.config/hoyoplay-umu/wine_prefix/drive_c/Program Files/HoYoPlay/launcher.exe` |
| **HoYoPlay Installer**  | `~/.config/hoyoplay-umu/Hoyoplay_setup.exe`                                      |
| **Proton-GE Build**     | `~/.config/hoyoplay-umu/ProtonLatest`                                            |
| **UMU Config File**     | `~/.config/hoyoplay-umu/umu_config/umu_config.toml`                                 |
| **Desktop Shortcut**    | `~/.local/share/applications/Hoyoplay.desktop`                                   |

---

<br/>
<br/>

## 👾 known Issues
- White bars in the border of the launcher
- Launcher flickering black some times
- Tray options not working when the app is open

---

<br/>
<br/>

## 🙌 Credits

* 🧩 [UMU Launcher](https://github.com/Open-Wine-Components/umu-launcher)
* ⚙️ [Proton-GE Custom](https://github.com/GloriousEggroll/proton-ge-custom)
* 🔥 [Feral Gamemode](https://github.com/FeralInteractive/gamemode)
---

<br/>
<br/>

### 💡 Tip

After installation, you can find **HoYoPlay** directly in your app launcher — ready to start your next adventure!

---

<br/>
<br/>

### 🤝 Contributions
Just me for now 0-0

---
