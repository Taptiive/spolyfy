use std::collections::HashMap;
use clap::{App, Arg};
use dbus::arg::RefArg;
use dbus::ffidisp::{Connection, ConnPath};
use dbus::ffidisp::stdintf::org_freedesktop_dbus::Peer;
use mpris::generated::media_player_player::OrgMprisMediaPlayer2Player;

fn trim_title(chars:usize, title:String) -> String{
    if title.len() <=chars {
        return title
    }
    let trimmed = &title.trim()[0..chars];
    format!("{}...", trimmed)
}

fn status(proxy:&ConnPath<&Connection>) -> String{
    let status = proxy.get_playback_status().expect("Error getting playback status");
    return match status.trim() == "Playing"{
        true => format!(""),
        false => format!("")
    }
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
            .long("live")
            .about("Checks if spotify is running or not")
    )
}

fn current(proxy:&ConnPath<&Connection>, chars:usize){
    let metadata = proxy.get_metadata().expect("Error getting metadata");
    let title = metadata.get("xesam:title").unwrap().as_str().unwrap();
    println!("{} by {}", trim_title(chars, title.to_string()), metadata.get("xesam:artist").unwrap()
        .0.as_iter().
        unwrap().
        map(|a|a.as_str().unwrap()).
        collect::<Vec<&str>>().
        join(", ").
        trim_end_matches(", "));
}

fn main() {
    let matches = init().get_matches();
    let con = Connection::new_session().expect("Error opening connection");
    let proxy = con.with_path("org.mpris.MediaPlayer2.spotify", "/org/mpris/MediaPlayer2", 5000);
    let play_pause = |a:&ConnPath<&Connection>|{
        a.play_pause().expect("Error pausing the current song");
        println!("{}", status(a));
    };
    let next = |a:&ConnPath<&Connection>|a.next().expect("Error playing the next song");
    let previous = |a:&ConnPath<&Connection>|a.previous().expect("Error playing the previous song");
    let live = |a:&ConnPath<&Connection>| {
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
    funcs_map.insert("live", Box::new(live));
    if matches.is_present("current"){
        let chars = matches.value_of("current").unwrap().parse::<usize>().expect("Error: argument must be a number");
        current(&proxy, chars);
    }
    for (key, func) in funcs_map.iter(){
        if matches.is_present(key){
            func(&proxy);
        }
    }
}
