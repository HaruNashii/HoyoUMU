# 🎮 HoyoUMU

**HoyoUMU** is a simple shell script that **downloads and sets up HoYoPlay using UMU** automatically — no manual tinkering required.

---

## ⚠️ Requirements

> **Important:**
> You must have [`umu-run`](https://github.com/Open-Wine-Components/umu-launcher) installed and located at `/usr/bin/umu-run` before running this script.

---

## 🧰 What This Script Does

✅ Installs the **latest version of Proton-GE**
✅ Downloads the **latest version of HoYoPlay**
✅ Fetches and installs the **official HoYoPlay icon**
✅ Creates a working **.desktop launcher** for easy access
✅ Configures **UMU** with the `GAMEID=umu-genshin` setup

---

## 📦 Dependencies

| Dependency                                                  | Description                      | Default Availability                |
| ----------------------------------------------------------- | -------------------------------- | ----------------------------------- |
| [UMU](https://github.com/Open-Wine-Components/umu-launcher) | Unified Modding Utility launcher | Must be installed manually          |
| `curl`                                                      | Fetches downloads                | Pre-installed on most Linux distros |
| `tar`                                                       | Extracts Proton-GE archives      | Pre-installed on most Linux distros |

---

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

## 📁 Installation Paths

| Component               | Path                                                                             |
| ----------------------- | -------------------------------------------------------------------------------- |
| **HoYoPlay Icon**       | `/usr/share/icons/hicolor/256x256/apps/hoyoplay_icon.png`                        |
| **HoYoPlay Executable** | `~/.config/hoyoplay-umu/wine_prefix/drive_c/Program Files/HoYoPlay/launcher.exe` |
| **HoYoPlay Installer**  | `~/.config/hoyoplay-umu/Hoyoplay_setup.exe`                                      |
| **Proton-GE Build**     | `~/.config/hoyoplay-umu/Proton-GE_Latest`                                        |
| **Desktop Shortcut**    | `~/.local/share/applications/Hoyoplay.desktop`                                   |
| **UMU Config File**     | `/usr/share/hoyoplay_umu_config/umu_config.toml`                                 |

---

## 🧩 To-Do

* [ ] Enable **Feral Gamemode** for performance boost
* [ ] Add error handling for network failures

---

## 🙌 Credits

* 🧩 [UMU Launcher](https://github.com/Open-Wine-Components/umu-launcher)
* 🥚 [Proton-GE Custom](https://github.com/GloriousEggroll/proton-ge-custom)

---

### 💡 Tip

After installation, you can find **HoYoPlay** directly in your app launcher — ready to start your next adventure!
