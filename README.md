# 🎮 HoyoUMU

**HoyoUMU** is a simple shell script that **downloads and sets up HoYoPlay using UMU and Proton-GE** automatically — no manual tinkering required.

---

<br/>
<br/>
<br/>

## ⚠️ Requirements

> **Important:**
> You must have [`umu-run`](https://github.com/Open-Wine-Components/umu-launcher) installed and located at `/usr/bin/umu-run` before running this script.

---

<br/>
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
✅ Configures **UMU** with my personal `hoyoumu.py` setup to guarantee an smooth adventure
<br/>
✅ Update **Proton-GE** to the latest version anytime with the `update_proton.sh` script
<br/>
✅ Completely **uninstall** without remaining any traces with the `uninstall.sh` script
<br/>
✅ Check **UMU** availability and if it exist copy it to the right place with the `check_umu.sh` script
<br/>
✅ Optionally runs all your games with **Feral Gamemode** for a very smooth experience

---

<br/>
<br/>
<br/>

## 📦 Dependencies

| Dependency                                                  | Description                                           | Default Availability                |
| ----------------------------------------------------------- | ----------------------------------------------------- | ----------------------------------- |
| [UMU](https://github.com/Open-Wine-Components/umu-launcher) | UMU is a unified launcher for Windows games on Linux  | Must be installed manually          |
| [Gamemode](https://github.com/FeralInteractive/gamemode/) [OPTIONAL] | Optimise Linux system performance on demand  | Must be installed manually          |
| `curl`                                                      | Fetches downloads                                     | Pre-installed on most Linux distros |
| `tar`                                                       | Extracts Proton-GE archives                           | Pre-installed on most Linux distros |

---

<br/>
<br/>
<br/>

## 🚀 Installation

Clone the repository and navigate to it:

```bash
git clone https://github.com/HaruNashii/HoyoUMU.git && cd HoyoUMU
```

Grant execution permission and run the installer:

```bash
chmod a+x install.sh && ./install.sh
```

---

<br/>
<br/>
<br/>

## 📁 Installation Paths

| Component               | Path                                                                             |
| ----------------------- | -------------------------------------------------------------------------------- |
| **HoYoPlay Icon**       | `/usr/share/icons/hicolor/256x256/apps/hoyoplay_icon.png`                        |
| **HoYoPlay Games**      | `~/.config/hoyoplay-umu/wine_prefix/drive_c/Program Files/HoYoPlay/games/`       |
| **HoYoPlay Executable** | `~/.config/hoyoplay-umu/wine_prefix/drive_c/Program Files/HoYoPlay/launcher.exe` |
| **HoYoPlay Installer**  | `~/.config/hoyoplay-umu/Hoyoplay_setup.exe`                                      |
| **Proton-GE Build**     | `~/.config/hoyoplay-umu/Proton-GE_Latest`                                        |
| **Desktop Shortcut**    | `~/.local/share/applications/Hoyoplay.desktop`                                   |
| **UMU Config File**     | `/usr/share/hoyoplay_umu_config/umu_config.toml`                                 |

---

<br/>
<br/>
<br/>

## 🙌 Credits

* 🧩 [UMU Launcher](https://github.com/Open-Wine-Components/umu-launcher)
* ⚙️ [Proton-GE Custom](https://github.com/GloriousEggroll/proton-ge-custom)
* 🔥 [Feral Gamemode](https://github.com/FeralInteractive/gamemode)
---

<br/>
<br/>
<br/>

### 💡 Tip

After installation, you can find **HoYoPlay** directly in your app launcher — ready to start your next adventure!
