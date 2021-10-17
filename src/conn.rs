use dbus::arg::PropMap;
use dbus::blocking::{Connection as DbusCon, Proxy};
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;

type GenericResult<R> = Result<R, dbus::Error>;

pub(crate) trait SpotifyConn{
    fn play_pause(&self)-> GenericResult<()>;
    fn next(&self) -> GenericResult<()>;
    fn previous(&self) -> GenericResult<()>;
    fn metadata(&self) -> GenericResult<PropMap>;
    fn artist(&self) -> GenericResult<String>;
    fn title(&self) -> GenericResult<String>;
}

impl<'r> SpotifyConn for Proxy<'r, &'r DbusCon>{
    fn play_pause(&self) -> GenericResult<()>{
        self.method_call("org.mpris.MediaPlayer2.spotify", "org.mpris.MediaPlayer2.Player.PlayPause", ())
    }
    fn next(&self) -> GenericResult<()>{
        self.method_call("org.mpris.MediaPlayer2.spotify", "org.mpris.MediaPlayer2.Player.Next", ())
    }
    fn previous(&self) -> GenericResult<()>{
        self.method_call("org.mpris.MediaPlayer2.spotify", "org.mpris.MediaPlayer2.Player.Previous", ())
    }
    fn metadata(&self) -> GenericResult<PropMap> {
        self.get("org.mpris.MediaPlayer2.Player", "Metadata")
    }
    fn artist(&self) -> GenericResult<String>{
        let data = self.metadata()?;
        let artists = data.get("xesam:artist").ok_or(dbus::Error::new_custom("err", "Empty artists list"))?
            .0.as_iter().ok_or(
            dbus::Error::new_custom("err", "Could not iterate over artists")
        )?.map(|a|a.as_str().unwrap()).collect::<Vec<&str>>();
        Ok(artists.join(", ").trim_end_matches(", ").to_string())
    }
    fn title(&self) -> GenericResult<String>{
        let data = self.metadata()?;
        let title = data.get("xesam:title").ok_or(
            dbus::Error::new_custom("err", "Failed to retrieve title")
        )?.0.as_str().ok_or(
            dbus::Error::new_custom("err", "Failed to transform into str")
        )?;
        Ok(title.to_string())
    }
}