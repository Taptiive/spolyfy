mod conn;

use std::collections::HashMap;
use clap::{App, Arg};
use dbus::ffidisp::{Connection as DbusCon, Connection, ConnPath};
use dbus::ffidisp::stdintf::org_freedesktop_dbus::Peer;
use conn::SpotifyConn;

fn trim_title(chars:usize, title:String) -> String{
    if title.len() <=chars {
        return title
    }
    let trimmed = &title.trim()[0..chars];
    format!("{}...", trimmed)
}

fn init() -> App<'static>{
    App::new("Spolyfy").version("1.0.0")
        .author("Taptiive <aalexius912@gmail.com>")
        .about("Spotify module for Polybar")
        .arg(
            Arg::new("play-pause")
                .short('s')
                .long("play-pause")
                .about("Plays or pauses the current spotify song")
        ).arg(
        Arg::new("next")
            .short('n')
            .long("next")
            .about("Plays the next song")
    ).arg(
        Arg::new("previous")
            .short('p')
            .long("previous")
            .about("Plays the previous song")
    ).arg(
        Arg::new("current")
            .short('c')
            .long("current")
            .about("Prints the current song's name and artist")
            .takes_value(true)
    ).arg(
        Arg::new("alive")
            .short('l')
            .long("alive")
            .about("Checks if spotify is running or not")
    ).arg(
        Arg::new("previous-icon")
            .short('a')
            .long("previcon")
            .about("Prints previous track icon (In Nerd Fonts)")
    ).arg(
        Arg::new("next-icon")
            .short('b')
            .long("nexticon")
            .about("Prints next track icon (In Nerd Fonts)")
    ).arg(
        Arg::new("play-pause-icon")
            .short('x')
            .long("playpause")
            .about("Prints play and pause icon (In Nerd Fonts)")
    )
}

fn current<'r>(proxy:&ConnPath<'r, &'r DbusCon>, chars:usize){
    let title = proxy.title().expect("Error: could not retrieve title");
    let artists = proxy.artist().expect("Error: could not retrieve artists");
    println!("{} by {}", trim_title(chars, title), artists);
}

fn main() {
    let matches = init().get_matches();
    let con = DbusCon::new_session().expect("Error opening dbus session");
    let prox = con.with_path("org.mpris.MediaPlayer2.spotify", "/org/mpris/MediaPlayer2", 5000);
    let play_pause = |a:&ConnPath<&Connection>|{
        a.play_pause().expect("Error: could not stop the song");
    };
    let next = |a:&ConnPath<&Connection>|a.next().expect("Error playing the next song");
    let previous = |a:&ConnPath<&Connection>|a.previous().expect("Error playing the previous song");
    let alive = |a:&ConnPath<&Connection>| {
        if a.ping().is_err(){
            println!("nope");
            return
        }
        println!("yep");
    };
    let mut funcs_map:HashMap<&str, Box<dyn Fn(&ConnPath<&Connection>)>> = HashMap::new();
    funcs_map.insert("play-pause", Box::new(play_pause));
    funcs_map.insert("previous", Box::new(previous));
    funcs_map.insert("next", Box::new(next));
    funcs_map.insert("alive", Box::new(alive));
    let mut icons_map = HashMap::new();
    icons_map.insert("previous-icon", "玲");
    icons_map.insert("next-icon", "怜");
    icons_map.insert("play-pause-icon", "");
    if matches.is_present("current") {
        let chars = matches.value_of("current").unwrap().parse::<usize>().expect("Error: argument must be a number");
        current(&prox, chars);
    }
    icons_map.iter().for_each(|(key, value)|{
        if matches.is_present(key) {println!("{}", value)}
    });
    funcs_map.iter().for_each(|(key, func)|{
        if matches.is_present(key){func(&prox)}
    });
}
