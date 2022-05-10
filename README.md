Because [osu!lazer](https://github.com/ppy/osu) on linux doesn't support for auto-update, that's why I made this

## Run
This is writing in [rust](https://www.rust-lang.org/), so download it first, then run clone this repo by
```sh
git clone https://github.com/tmokenc/osu-linux-launcher
cd osu-linux-launcher
```

After that install it by run this command
```sh
cargo install --path .
```

Now you can use the command `osu-linux-launcher` to launcher osu, it will check for update as well
You may add an alias `alias osu=osu-linux-launcher` to `~/.bashrc` to launch it with only `osu` command

## Todo
Maybe graphical interface for it, IDK
