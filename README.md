# EU4 save scum tool

![](https://github.com/Pzixel/easy_savescum/workflows/Publish/badge.svg)

Automate your save backuping shenanigans with power of Rust

![image](https://user-images.githubusercontent.com/11201122/77762830-2727cf80-704b-11ea-807c-d8db73167afc.png)

### How to use

You can download the app [in `releases` tab](https://github.com/Pzixel/easy_savescum/releases). There are Windows/Mac/Linux versions, although I've only tested a Windows one (I don't have Mac or Linux), but they should work. Then just downaload and execute.

If you're using a different folder for saves then you need to open `command prompt`/`powershell`/`...younameit` in directory with the app and run the program with arguments, e.g.

### Motivation 

I think I need to start with motivation "Why would anyone do such a terrible thing". So I'l tell a little story. I've played most countries in the game, and I wanted to try Ryukyu. But I knew it's a very hard campaign so I started watchin florryworry streams. Surprisingly, even this god-on-the-earth couldn't manage the campaign without ALT+F4. He's laughing at "birding" but anyway, he's doing it, and quite a lot. I think I counted tens of times on the first video. 

I'm no match to florry so I decided that I need a tool. I also wanted to play with programming such a tool, so there were two reasons, actually. Also, one year ago there was a post about [such a tool](https://www.reddit.com/r/eu4/comments/akxb5w/no_more_altf4_try_out_new_automatic_save_scumming/) which was warmly accepted, although people complained about several issues.

Now I present the tool:

### Tool

This is a program I've called `easy_savescum`. It does nothing special: it monitors your save directory and backup save files time to time. It doesn't have any fancy overlays or addition functionality like force making a save. Just a little tool in Unix philosophy style. Directory to monitor is configurable although it's set by default on your local save folder. Frequency is also configurable, by default it's 4 save ticks which means "yearly". It can also be dynamically changed if you're at important war and want to have more frequent saves or you don't like to feel like a cheater so you set it to 50 years so you won't fuck up in strategic terms, but you can afford tactical failures.


```bash
./easy_savescum-windows-refs.tags.v0.1.3-amd64 "C:\Program Files (x86)\and\so\on"
```

P.S. Is actually not specific for the EU, it can backup any game which has autosaves and allows to load them, such as CK2, Imperator, etc.
