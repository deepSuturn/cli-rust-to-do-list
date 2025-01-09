#!/bin/bash




# Define variables
APP_NAME="to-do"
TARGET_DIR="/usr/local/bin"
BINARY_PATH="./$APP_NAME"


# Check if the user has sudo privileges
if [ "$EUID" -ne 0 ]; then
  echo "Please run this script as root or with sudo."
  exit 1
fi

cargo build --release

cd ./target/release


# Ensure the binary exists
if [ ! -f "$BINARY_PATH" ]; then
  echo "Error: Binary file '$BINARY_PATH' not found."
  exit 1
fi

# Copy the binary to the target directory
echo "Installing $APP_NAME to $TARGET_DIR..."
cp "$BINARY_PATH" "$TARGET_DIR"

# Set executable permissions
chmod +x "$TARGET_DIR/$APP_NAME"

# Verify installation
if command -v "$APP_NAME" >/dev/null 2>&1; then
  echo "$APP_NAME has been installed successfully!"
  echo "You can now use the command '$APP_NAME'."
else
  echo "Error: $APP_NAME installation failed."
  exit 1
fi
