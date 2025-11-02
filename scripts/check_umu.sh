#!/bin/bash



# ==== Configuration ====
APP_NAME="umu-run"                 # App to check
WHICH_APP=$(which $APP_NAME)
# ========================

# Check if the app exists in PATH
if ! command -v "$APP_NAME" &> /dev/null; then

    clear
    echo "ERROR!!!: '$APP_NAME' is not installed, please install it before running this script again."
    exit 1

else
	if [[ "$WHICH_APP" != "/usr/bin/$APP_NAME" ]]; then

		while true; do
			read -rp "$APP_NAME Is Installed, But is not in the necessary path, would you like to copy it to the right path? (will be moved to: '/usr/bin/') [y/n]: " answer
			case "$answer" in
				[Yy]* )
					echo "Moving $APP_NAME To /usr/bin..."
					sudo mkdir -p /usr/bin
					sudo cp -f $WHICH_APP /usr/bin/$APP_NAME
					sudo mv $WHICH_APP $WHICH_APP-user
					clear
					echo "$APP_NAME Passed the Check!!! Yayy!!!"
					break
					;;
				[Nn]* )
					echo "Unfortunately the app won't run without umu-run in the /usr/bin"
					echo "So, bye bye o/"
					exit 0
					break
					;;
				* )
					echo "❌ Invalid input. Please enter 'y' or 'n'."
					;;
			esac
		done

	else 
		echo "$APP_NAME Passed the Check!!! Yayy!!!"
	fi
fi

