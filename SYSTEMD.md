# Running Lule as a Systemd Service

## Quick Fix Summary

The main issue preventing lule from running in systemd/background was the TTY detection logic that blocked execution when not connected to a terminal. The fixes include:

1. **Removed TTY blocking logic** in `create.rs` and `daemon.rs`
2. **Fixed relative path issues** for the logo file in `main.rs`
3. **Added TTY checks for output only** to prevent unnecessary logging when running as daemon

## Setup Instructions

### 1. Install the Binary

```bash
cargo build --release
sudo cp target/release/lule /usr/local/bin/
```

### 2. Create Required Directories

```bash
mkdir -p ~/.wallpaper
mkdir -p ~/.config/lule
mkdir -p ~/.cache/lule
```

### 3. Set Up Your Script

Copy and customize the example script:

```bash
cp scripts/apply_colors.sh ~/.config/lule/apply_colors.sh
chmod +x ~/.config/lule/apply_colors.sh
```

**Important**: Edit `~/.config/lule/apply_colors.sh` and replace hardcoded paths:
- Replace `/home/bresilla/` with `$HOME/` or your actual home directory
- Example: Change line 55 from:
  ```bash
  sed -i "s/fill=\"#\([^\"]*\)\"/fill=\"$col1\"/" /home/bresilla/.config/bresilla.svg
  ```
  to:
  ```bash
  sed -i "s/fill=\"#\([^\"]*\)\"/fill=\"$col1\"/" $HOME/.config/bresilla.svg
  ```

### 4. Install Systemd Service

Create the service file:

```bash
mkdir -p ~/.config/systemd/user/
cat > ~/.config/systemd/user/lule.service << 'EOF'
[Unit]
Description=Lule Color Daemon
After=graphical-session.target

[Service]
Type=simple
WorkingDirectory=%h

# Environment variables
Environment="LULE_W=%h/.wallpaper"
Environment="LULE_S=%h/.config/lule/apply_colors.sh"
Environment="LULE_A=%h/.cache/lule"

# Command
ExecStart=/usr/local/bin/lule daemon start

# Logging
StandardOutput=journal
StandardError=journal

# Restart on failure
Restart=on-failure
RestartSec=5

[Install]
WantedBy=default.target
EOF
```

### 5. Enable and Start the Service

```bash
# Reload systemd
systemctl --user daemon-reload

# Enable to start on boot
systemctl --user enable lule.service

# Start now
systemctl --user start lule.service

# Check status
systemctl --user status lule.service
```

### 6. View Logs

```bash
# Follow logs in real-time
journalctl --user -u lule.service -f

# View recent logs
journalctl --user -u lule.service -n 50
```

## Controlling the Daemon

### Change Wallpaper/Colors

```bash
lule daemon next
```

### Stop the Daemon

```bash
systemctl --user stop lule.service
# or
lule daemon stop
```

## Running in Background (Without Systemd)

You can also run lule in the background using the detach mode:

```bash
export LULE_W="$HOME/.wallpaper"
export LULE_S="$HOME/.config/lule/apply_colors.sh"
lule daemon detach
```

This will:
- Daemonize the process
- Write logs to `/tmp/daemon.out` and `/tmp/daemon.err`
- Create PID file in `/tmp/lule.pid`

## Troubleshooting

### Service fails to start

1. Check logs: `journalctl --user -u lule.service -n 50`
2. Verify environment variables are set correctly
3. Ensure directories exist: `~/.wallpaper`, `~/.cache/lule`
4. Make sure script is executable: `chmod +x ~/.config/lule/apply_colors.sh`

### No wallpapers found

- Put wallpaper images in `~/.wallpaper/`
- Or set `LULE_W` to your wallpaper directory

### Script not executing

- Check script permissions: `ls -l ~/.config/lule/apply_colors.sh`
- Test script manually: `bash ~/.config/lule/apply_colors.sh`
- Verify `LULE_S` environment variable is set correctly

### Colors not applying to terminals

When running as a daemon without active TTY sessions, the `/dev/pts/*` writing won't work. You may need to:
1. Source the colors in your shell's rc file (`.bashrc`, `.zshrc`)
2. Use a display manager hook to apply colors on login
3. Configure individual applications to read from `~/.cache/wal/` directory

## Technical Details

### What Was Fixed

1. **TTY Detection Blocking** (main issue):
   - Old code: `if atty::isnt(atty::Stream::Stdout) { /* skip logic */ }`
   - New code: Removed the blocking condition, only use TTY detection for output

2. **Relative Path Issues**:
   - Logo file now tries multiple paths including executable directory
   - Falls back gracefully when file not found

3. **Output Noise**:
   - Print statements only execute when connected to TTY
   - Reduces log spam when running as daemon

### Remaining Limitations

- The `apply_colors.sh` script writes to `/dev/pts/*` which requires active terminal sessions
- Some color changes may not apply to already-running terminal emulators
- Hardcoded user paths in script need manual adjustment
