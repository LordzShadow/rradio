use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Station {
    url: String,
    name: String,
    uuid: String,
}

impl Station {
    pub fn new(url: &str, name: &str, uuid: &str) -> Self {
        Station {
            url: url.to_string(),
            name: name.to_string(),
            uuid: uuid.to_string(),
        }
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_uuid(&self) -> &str {
        &self.uuid
    }
}

pub fn get_stations() -> Vec<Station> {
    vec![
        Station::new(
            "https://router.euddn.net/8103046e16b71d15d692b57c187875c7/elmar.aac",
            "Elmar",
            "test-uuid-1",
        ),
        Station::new(
            "https://stream.skymedia.ee/live/relax",
            "Relax FM",
            "test-uuid-2",
        ),
    ]
}

pub fn get_station_by_uuid(uuid: &str) -> Option<Station> {
    get_stations()
        .iter()
        .find(|station| station.uuid == uuid)
        .cloned()
}
