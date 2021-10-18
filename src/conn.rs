use dbus::blocking::{Connection as DbusCon, Proxy};
use std::collections::HashMap;
use dbus::arg::{RefArg, Variant};
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;

type GenericResult<R> = Result<R, dbus::Error>;

pub(crate) trait SpotifyConn{
    fn play_pause(&self)-> GenericResult<()>;
    fn next(&self) -> GenericResult<()>;
    fn previous(&self) -> GenericResult<()>;
    fn get_metadata(&self) -> GenericResult<PropMap>;
    fn artist(&self) -> GenericResult<String>;
    fn title(&self) -> GenericResult<String>;
}

type PropMap = HashMap<String, Variant<Box<dyn RefArg>>>;

impl<'r> SpotifyConn for Proxy<'r, &'r DbusCon>{
    fn play_pause(&self) -> GenericResult<()>{
        self.method_call("org.mpris.MediaPlayer2.Player", "PlayPause", ())
    }
    fn next(&self) -> GenericResult<()>{
        self.method_call("org.mpris.MediaPlayer2.Player", "Next", ())
    }
    fn previous(&self) -> GenericResult<()>{
        self.method_call("org.mpris.MediaPlayer2.Player", "Previous", ())
    }
    fn get_metadata(&self) -> GenericResult<PropMap>{
        self.get("org.mpris.MediaPlayer2.Player", "Metadata")

    }
    fn artist(&self) -> GenericResult<String>{
        let data = self.get_metadata()?;
        let artists = data.get("xesam:artist").ok_or(dbus::Error::new_failed("Empty artists list"))?
            .0.as_iter().ok_or(
            dbus::Error::new_failed("Could not iterate over artists")
        )?.map(|a|a.as_str().unwrap()).collect::<Vec<&str>>();
        Ok(artists.join(", ").trim_end_matches(", ").to_string())
    }
    fn title(&self) -> GenericResult<String>{
        let data = self.get_metadata()?;
        let title = data.get("xesam:title").ok_or(
            dbus::Error::new_failed("Failed to retrieve title")
        )?.0.as_str().ok_or(
            dbus::Error::new_failed("Failed to transform into str")
        )?;
        Ok(title.to_string())
    }
}