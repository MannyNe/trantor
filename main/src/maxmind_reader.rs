use color_eyre::eyre::Context;
use domain::{async_trait::async_trait, tracing, GeoIpReader, GeoIpReaderError, Location};
use std::sync::Arc;

#[derive(Clone)]
pub struct MaxmindGeoIpReader {
    reader: Arc<maxminddb::Reader<Vec<u8>>>,
}

impl MaxmindGeoIpReader {
    pub fn new(database: &str) -> color_eyre::Result<Self> {
        let reader = maxminddb::Reader::open_readfile(database)
            .wrap_err_with(|| format!("couldn't open geolite2 city database file: {}", database))?;
        Ok(Self {
            reader: Arc::new(reader),
        })
    }
}

#[async_trait]
impl GeoIpReader for MaxmindGeoIpReader {
    async fn parse(&self, ip_addr: std::net::IpAddr) -> Result<Location, GeoIpReaderError> {
        let location = self
            .reader
            .lookup::<maxminddb::geoip2::City>(ip_addr)
            .map_err(MaxmindGeoIpReaderError::from)?;

        let country_code = location
            .country
            .and_then(|c| c.iso_code)
            .map(|iso| iso.to_owned());
        let city_name = location
            .city
            .and_then(|c| c.names)
            .and_then(|ns| ns.get("en").copied())
            .map(|n| n.to_owned());
        let continent_code = location
            .continent
            .and_then(|c| c.code)
            .map(|c| c.to_owned());

        Ok(Location::new(country_code, city_name, continent_code))
    }
}

struct MaxmindGeoIpReaderError(maxminddb::MaxMindDBError);

impl From<maxminddb::MaxMindDBError> for MaxmindGeoIpReaderError {
    fn from(err: maxminddb::MaxMindDBError) -> Self {
        Self(err)
    }
}

impl From<MaxmindGeoIpReaderError> for GeoIpReaderError {
    fn from(err: MaxmindGeoIpReaderError) -> Self {
        tracing::error!("error in geoip reader: {}", err.0);
        Self::Other
    }
}
