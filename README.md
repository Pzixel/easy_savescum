# Ironman backup utility

![](https://github.com/Pzixel/easy_savescum/workflows/Publish/badge.svg)

Automate your save backuping shenanigans with power of Rust

### How to use

You can download the app [in `releases` tab](https://github.com/Pzixel/easy_savescum/releases).

If you're using a different folder for saves then you need to open `command prompt`/`powershell`/`...younameit` in directory with the app and run the program with arguments, e.g.

### Tool

This is a program I've called `easy_savescum`. It does nothing special: it monitors your save directory and backup save files time to time. It doesn't have any fancy overlays or addition functionality like force making a save. Just a little tool in Unix philosophy style. Directory to monitor is configurable although it's set by default on your local save folder. Frequency is also configurable, by default it's 4 save ticks which means "yearly". It can also be dynamically changed if you're at important war and want to have more frequent saves or you don't like to feel like a cheater so you set it to 50 years so you won't fuck up in strategic terms, but you can afford tactical failures.


```bash
./easy_savescum-windows-refs.tags.v0.1.4-amd64 \
  --path '/Users/pzixel/Library/Application Support/Feral Interactive/XCOM 2 WotC/VFS/Local/my games/XCOM2 War of the Chosen/XComGame/SaveData' \
  --savescum-dir-path '/Users/pzixel/Library/Application Support/Feral Interactive/XCOM 2 WotC/VFS/Local/my games/XCOM2 War of the Chosen/XComGame/SaveScum'
```

