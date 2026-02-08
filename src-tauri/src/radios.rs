use serde::Serialize;

#[derive(Serialize)]
pub struct Station {
    url: String,
    name: String,
}

pub fn get_stations() -> Vec<Station> {
    vec![
        Station {
            url: String::from(
                "https://router.euddn.net/8103046e16b71d15d692b57c187875c7/elmar.aac",
            ),
            name: String::from("Elmar"),
        },
        Station {
            url: String::from("https://stream.skymedia.ee/live/relax"),
            name: String::from("Relax FM"),
        },
    ]
}
