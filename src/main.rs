mod conn;

use std::collections::HashMap;
use clap::{App, Arg};
use dbus::blocking::{Connection as DbusCon, Proxy};
use std::time::Duration;
use dbus::blocking::stdintf::org_freedesktop_dbus::Peer;
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
        Arg::new("live")
            .short('l')
            .long("alive")
            .about("Checks if spotify is running or not")
    )
}

fn current<'r>(proxy:&Proxy<'r, &'r DbusCon>, chars:usize){
    let title = proxy.title().expect("Error: could not retrieve title");
    let artists = proxy.artist().expect("Error: could not retrieve artists");
    println!("{} by {}", trim_title(chars, title), artists);
}

fn main() {
    let matches = init().get_matches();
    let con = DbusCon::new_session().expect("Error opening dbus session");
    let prox = con.with_proxy("org.mpris.MediaPlayer2.spotify", "/org/mpris/MediaPlayer2", Duration::from_secs(5000));
    let play_pause = ||{
        prox.play_pause().expect("Error: could not stop the song");
    };
    let next = ||prox.next().expect("Error playing the next song");
    let previous = ||prox.previous().expect("Error playing the previous song");
    let alive = || {
        if prox.ping().is_err(){
            println!("nope");
            return
        }
        println!("yep");
    };
    let mut funcs_map:HashMap<&str, Box<dyn Fn()>> = HashMap::new();
    funcs_map.insert("play-pause", Box::new(play_pause));
    funcs_map.insert("previous", Box::new(previous));
    funcs_map.insert("next", Box::new(next));
    funcs_map.insert("alive", Box::new(alive));
    if matches.is_present("current"){
        let chars = matches.value_of("current").unwrap().parse::<usize>().expect("Error: argument must be a number");
        current(&prox, chars);
    }
    for (key, func) in funcs_map.iter(){
        if matches.is_present(key){
            func();
        }
    }
}
