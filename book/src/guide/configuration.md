# Configuration

## Config file

After running `treddit` for the first time, a config file will be generated at the following locations, depending on your OS:

- Linux/MacOS: `~/.config/treddit/config.toml`
- Windows: `%appdata%\treddit\config.toml`

You can find the default configuration values [here](https://github.com/Xithrius/reddit-tui/blob/main/default-config.toml).

## Auth

1. Go to [app preferences](https://www.reddit.com/prefs/apps), and make a script with whatever name you'd like. Set the redirect url to something like `http://127.0.0.1:65010/authorize_callback`.
2. Copy the credentials that you need into the previously stated generated config.
3. Run `treddit`.

If you'd like to read up even more, go to [Reddit's OAuth2 guide](https://github.com/reddit-archive/reddit/wiki/OAuth2-Quick-Start-Example).


## Run it

Run `treddit` in the terminal. For help, `treddit --help`.
