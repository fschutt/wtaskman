# wtaskman

wtaskman is a Rust / webkit based task manager, similar to the Windows Task Manager

![wtaskman in action](https://i.imgur.com/n3kJ1MT.png)

Not yet finished, only in the prototype stage. 

A notable feature is that it uses the native Web-rendering engine, i.e. MSIE-Renderer on
Windows, webkit2gtk on Linux, Safari / Blink on Mac.

# Installation

Currently it only works on Linux, where you will need to install: `webkit2gtk`

```
sudo apt install libwebkit2gtk-4.0-dev
```