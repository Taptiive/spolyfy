# **Spolyfy**

Spolyfy is a Polybar module written in Rust that provides current Spotify song's title, previous, next and play-pause buttons.

### **How to install**

You can install Spolyfy using [Cargo](https://www.rust-lang.org/tools/install) with the following command:

```console
$ cargo install spolyfy
```

### **How to use**

These are the modules you need for the polybar config

```ini
[module/spotify-current]
type=custom/script
format = <label>
label = %output%
exec = spolyfy -c 35 ;35 is the max number of characters allowed in the title
exec-if = [ $(spolyfy -l) = "yep" ]
interval = 3
```

```ini
[module/spotify-previous]
type=custom/script
format = <label>
label = %output%
exec = echo "玲" ;Previous symbol on Nerd Fonts
exec-if = [ $(spolyfy -l) = "yep" ]
click-left = spolyfy -p
```

```ini
[module/spotify-play-pause]
type=custom/script
format = <label>
label = %output%
click-left = spolyfy -s
exec = echo " " ;Play and pause symbols on Nerd Fonts
exec-if = [ $(spolyfy -l) = "yep" ]
```

```ini
[module/spotify-next]
type=custom/script
format = <label>
label = %output%
exec = echo "怜" ;Next symbol on Nerd Fonts
exec-if = [ $(spolyfy -l) = "yep" ]
click-left = spolyfy -n
```

Then you can add the modules in your bar anyway you want

**Example**

```ini
[bar/mybar]
modules-center = spotify-current spotify-previous spotify-play-pause spotify-next
```

![This is how it would look like](./example.png "Spolybar example")

### **Notes**

Im not very experienced in Polybar nor Rust, so please feel free to report any bug or improvement to the code ;)

