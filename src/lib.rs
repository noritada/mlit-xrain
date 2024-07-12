#[derive(Debug)]
pub enum Area {
    Kanto = 0x81,
    Kyushu,
    Hokkaido,
    Tohoku,
    Hokuriku,
    Chubu,
    Kinki,
    Chugoku,
    Shikoku,
    Okinawa,
}
use Area::*;

impl TryFrom<u8> for Area {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let area = match value {
            0x81 => Kanto,
            0x82 => Kyushu,
            0x83 => Hokkaido,
            0x84 => Tohoku,
            0x85 => Hokuriku,
            0x86 => Chubu,
            0x87 => Kinki,
            0x88 => Chugoku,
            0x89 => Shikoku,
            0x8a => Okinawa,
            _ => return Err(format!("unknown id: {:#04x}", value)),
        };
        Ok(area)
    }
}

#[derive(Debug)]
pub enum RadarSite {
    // C
    PINNESHIRI,
    OTOBEDAKE0,
    MURIYAMA00,
    HAKODEKE00,
    NISIDAKE00,
    MONOMI0000,
    SIRATAKA00,
    AKAGI00000,
    MITSUTOGE0,
    OOKUSU0000,
    TAKASUZU00,
    HOUTATSU00,
    YAKUSHI000,
    HIJIRI0000,
    GOZAISHO00,
    JYATOUGE00,
    MIYAMA0000,
    JOGAMORI00,
    RKN0000000,
    OWA0000000,
    MYOUJIN000,
    TAKASHIRO0,
    SYAKA00000,
    KUNIMI0000,
    HAPPONGI00,
    YAE0000000,

    // X
    ISHIKARI00,
    KITAHIRO00,
    MORIOKA000,
    TAKANOSU00,
    WAKUYA0000,
    IWANUMA000,
    DATE000000,
    TAMURA0000,
    UJIIE00000,
    YATAJIIMA0,
    KANTOU0000,
    FUNABASHI0,
    SHINYOKO00,
    KYOUGASE00,
    NAKANOKUTI,
    MIZUHASHI0,
    NOUMI00000,
    BISAI00000,
    FUJINOMIYA,
    KANUKI0000,
    SHIZUKITA0,
    SUZUKA0000,
    ANJOU00000,
    HAMAMATSU0,
    JUUBUSAN00,
    TANOKUCHI0,
    ROKKO00000,
    KATSURAGI0,
    KUMAYAMA00,
    TSHNEYAMA0,
    USHIO00000,
    NOGAIBARA0,
    KAZASI0000,
    FURUTSUKI0,
    SUGADAKE00,
    KUSENBU000,
    YAMAGA0000,
    UKI0000000,
    SAKURAJIMA,
}
use RadarSite::*;

impl RadarSite {
    pub fn band(&self) -> Band {
        match self {
            PINNESHIRI | OTOBEDAKE0 | MURIYAMA00 | HAKODEKE00 | NISIDAKE00 | MONOMI0000
            | SIRATAKA00 | AKAGI00000 | MITSUTOGE0 | OOKUSU0000 | TAKASUZU00 | HOUTATSU00
            | YAKUSHI000 | HIJIRI0000 | GOZAISHO00 | JYATOUGE00 | MIYAMA0000 | JOGAMORI00
            | RKN0000000 | OWA0000000 | MYOUJIN000 | TAKASHIRO0 | SYAKA00000 | KUNIMI0000
            | HAPPONGI00 | YAE0000000 => Band::C,
            ISHIKARI00 | KITAHIRO00 | MORIOKA000 | TAKANOSU00 | WAKUYA0000 | IWANUMA000
            | DATE000000 | TAMURA0000 | UJIIE00000 | YATAJIIMA0 | KANTOU0000 | FUNABASHI0
            | SHINYOKO00 | KYOUGASE00 | NAKANOKUTI | MIZUHASHI0 | NOUMI00000 | BISAI00000
            | FUJINOMIYA | KANUKI0000 | SHIZUKITA0 | SUZUKA0000 | ANJOU00000 | HAMAMATSU0
            | JUUBUSAN00 | TANOKUCHI0 | ROKKO00000 | KATSURAGI0 | KUMAYAMA00 | TSHNEYAMA0
            | USHIO00000 | NOGAIBARA0 | KAZASI0000 | FURUTSUKI0 | SUGADAKE00 | KUSENBU000
            | YAMAGA0000 | UKI0000000 | SAKURAJIMA => Band::X,
        }
    }
}

impl TryFrom<(u8, u8)> for RadarSite {
    type Error = String;

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        let (area_id, num) = value;
        let site = match (Area::try_from(area_id), num) {
            (Ok(Kanto), 1) => AKAGI00000,
            (Ok(Kanto), 2) => MITSUTOGE0,
            (Ok(Kanto), 3) => OOKUSU0000,
            (Ok(Kanto), 4) => TAKASUZU00,
            (Ok(Kanto), 5) => KANTOU0000,
            (Ok(Kanto), 6) => SHINYOKO00,
            (Ok(Kanto), 7) => UJIIE00000,
            (Ok(Kanto), 8) => YATAJIIMA0,
            (Ok(Kanto), 9) => FUNABASHI0,
            (Ok(Kyushu), 1) => SYAKA00000,
            (Ok(Kyushu), 2) => KUNIMI0000,
            (Ok(Kyushu), 3) => HAPPONGI00,
            (Ok(Kyushu), 5) => KUSENBU000,
            (Ok(Kyushu), 6) => SUGADAKE00,
            (Ok(Kyushu), 7) => FURUTSUKI0,
            (Ok(Kyushu), 8) => KAZASI0000,
            (Ok(Kyushu), 9) => SAKURAJIMA,
            (Ok(Kyushu), 10) => YAMAGA0000,
            (Ok(Kyushu), 11) => UKI0000000,
            (Ok(Hokkaido), 1) => PINNESHIRI,
            (Ok(Hokkaido), 2) => OTOBEDAKE0,
            (Ok(Hokkaido), 3) => MURIYAMA00,
            (Ok(Hokkaido), 4) => HAKODEKE00,
            (Ok(Hokkaido), 5) => KITAHIRO00,
            (Ok(Hokkaido), 6) => ISHIKARI00,
            (Ok(Tohoku), 1) => NISIDAKE00,
            (Ok(Tohoku), 2) => MONOMI0000,
            (Ok(Tohoku), 3) => SIRATAKA00,
            (Ok(Tohoku), 7) => WAKUYA0000,
            (Ok(Tohoku), 8) => IWANUMA000,
            (Ok(Tohoku), 9) => DATE000000,
            (Ok(Tohoku), 10) => TAMURA0000,
            (Ok(Tohoku), 11) => MORIOKA000,
            (Ok(Tohoku), 12) => TAKANOSU00,
            (Ok(Hokuriku), 1) => HOUTATSU00,
            (Ok(Hokuriku), 2) => YAKUSHI000,
            (Ok(Hokuriku), 3) => HIJIRI0000,
            (Ok(Hokuriku), 5) => MIZUHASHI0,
            (Ok(Hokuriku), 6) => NOUMI00000,
            (Ok(Hokuriku), 7) => KYOUGASE00,
            (Ok(Hokuriku), 8) => NAKANOKUTI,
            (Ok(Chubu), 1) => GOZAISHO00,
            (Ok(Chubu), 2) => JYATOUGE00,
            (Ok(Chubu), 5) => BISAI00000,
            (Ok(Chubu), 6) => ANJOU00000,
            (Ok(Chubu), 7) => SUZUKA0000,
            (Ok(Chubu), 8) => SHIZUKITA0,
            (Ok(Chubu), 9) => KANUKI0000,
            (Ok(Chubu), 10) => FUJINOMIYA,
            (Ok(Chubu), 11) => HAMAMATSU0,
            (Ok(Kinki), 1) => MIYAMA0000,
            (Ok(Kinki), 2) => JOGAMORI00,
            (Ok(Kinki), 5) => ROKKO00000,
            (Ok(Kinki), 6) => KATSURAGI0,
            (Ok(Kinki), 7) => JUUBUSAN00,
            (Ok(Kinki), 8) => TANOKUCHI0,
            (Ok(Chugoku), 1) => RKN0000000,
            (Ok(Chugoku), 2) => OWA0000000,
            (Ok(Chugoku), 5) => KUMAYAMA00,
            (Ok(Chugoku), 6) => TSHNEYAMA0,
            (Ok(Chugoku), 7) => NOGAIBARA0,
            (Ok(Chugoku), 8) => USHIO00000,
            (Ok(Shikoku), 1) => MYOUJIN000,
            (Ok(Shikoku), 2) => TAKASHIRO0,
            (Ok(Okinawa), 1) => YAE0000000,
            _ => return Err(format!("unknown id: ({:#04x}, {:#04x})", area_id, num)),
        };
        Ok(site)
    }
}

#[derive(Debug)]
pub enum Band {
    C,
    X,
}

#[derive(Debug)]
pub struct DateTime(pub u16, pub u8, pub u8, pub u8, pub u8);

impl std::str::FromStr for DateTime {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(s: &str) -> Option<DateTime> {
            let year = s[0..4].parse::<u16>().ok()?;
            let month = s[5..7].parse::<u8>().ok()?;
            let day = s[8..10].parse::<u8>().ok()?;
            let hour = s[11..13].parse::<u8>().ok()?;
            let minute = s[14..16].parse::<u8>().ok()?;
            Some(DateTime(year, month, day, hour, minute))
        }
        parse(s).ok_or_else(|| Error {
            message: "parsing datetime failed".to_owned(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
