# Reddit in the terminal

### How it looks:
- Need a working program first


### Setup:
1. Go to [app preferences](https://www.reddit.com/prefs/apps), and make a script with whatever name you'd like.
2. Copy the contents of `default-config.toml` into `~/.config/treddit/config.toml` if you're on Linux or MacOs. If on Windows, put this config file in `%appdata%\treddit\config.toml`.
3. Install the program with `cargo install reddit-tui`, and run with `treddit`. To get updates, simply do `cargo install reddit-tui` whenever you'd like.

If you'd like to read up even more, go to [Reddit's OAuth2 guide](https://github.com/reddit-archive/reddit/wiki/OAuth2-Quick-Start-Example).
