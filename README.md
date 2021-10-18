# **Spolyfy**

Spolyfy is a Polybar module written in Rust that provides current Spotify song's title, previous, next and play-pause buttons.

### **How to install**

You can install Spolyfy using [Cargo](https://www.rust-lang.org/tools/install) with the following command:

```console
$ cargo install spolyfy
```

### **How to use**

You can setup the modules like this

```ini
[module/spotify-current]
type=custom/script
exec = spolyfy -c 35
exec-if = [ $(spolyfy -l) = "yep" ]
```

```ini
[module/spotify-previous]
type=custom/script
exec = spolyfy -a ;This command prints a Nerd Font icon
exec-if = [ $(spolyfy -l) = "yep" ]
click-left = spolyfy -p
```

```ini
[module/spotify-play-pause]
type=custom/script
exec = spolyfy -x ;This command prints a Nerd Font icon
exec-if = [ $(spolyfy -l) = "yep" ]
click-left = spolyfy -s
```

```ini
[module/spotify-next]
type=custom/script
exec = spolyfy -b ;This command prints a Nerd Font icon
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

