#!/bin/bash

# Define variables
APP_NAME="to-do"
TARGET_DIR="/usr/local/bin"
BINARY_PATH="./target/release/$APP_NAME"
SQLITE_DB_PATH="/var/lib/$APP_NAME/to_do.db"

# Check if the user has sudo privileges
if [ "$EUID" -ne 0 ]; then
  echo "Please run this script as root or with sudo."
  exit 1
fi

# Function to check and install SQLite3
install_sqlite3() {
  if ! command -v sqlite3 >/dev/null 2>&1; then
    echo "SQLite3 is not installed. Installing SQLite3..."
    
    # Check the package manager
    if command -v apt >/dev/null 2>&1; then
      sudo apt update
      sudo apt install -y sqlite3
    elif command -v yum >/dev/null 2>&1; then
      sudo yum install -y sqlite3
    elif command -v dnf >/dev/null 2>&1; then
      sudo dnf install -y sqlite3
    elif command -v pacman >/dev/null 2>&1; then
      sudo pacman -S --noconfirm sqlite
    else
      echo "Package manager not supported."
      exit 1
    fi
    
    echo "SQLite3 installed successfully."
  else
    echo "SQLite3 is already installed."
  fi
}

install_sqlite3

# Build the application in release mode
echo "Building $APP_NAME..."
cargo build --release

# Ensure the binary exists
if [ ! -f "$BINARY_PATH" ]; then
  echo "Error: Binary file '$BINARY_PATH' not found. Build failed?"
  exit 1
fi

# Copy the binary to the target directory
echo "Installing $APP_NAME to $TARGET_DIR..."
cp "$BINARY_PATH" "$TARGET_DIR"

# Set executable permissions
chmod +x "$TARGET_DIR/$APP_NAME"

# Set up SQLite database
echo "Setting up SQLite database..."
if [ ! -d "/var/lib/$APP_NAME" ]; then
  mkdir -p "/var/lib/$APP_NAME"
fi

if [ ! -f "$SQLITE_DB_PATH" ]; then
  touch "$SQLITE_DB_PATH"
  echo "SQLite database created at $SQLITE_DB_PATH."
else
  echo "SQLite database already exists at $SQLITE_DB_PATH."
fi

# Set appropriate permissions for the SQLite database
chmod 644 "$SQLITE_DB_PATH"
chown "$USER":"$USER" "$SQLITE_DB_PATH"

# Run Diesel migrations
echo "Running Diesel migrations..."
diesel migration run --database-url=sqlite://$SQLITE_DB_PATH

# Check and install SQLite3


# Verify installation
if command -v "$APP_NAME" >/dev/null 2>&1; then
  echo "$APP_NAME has been installed successfully!"
  echo "You can now use the command '$APP_NAME'."
  echo "SQLite database is located at $SQLITE_DB_PATH."
else
  echo "Error: $APP_NAME installation failed."
  exit 1
fi
