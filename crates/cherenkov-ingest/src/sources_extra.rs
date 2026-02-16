use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::time::Duration;
use tracing::{info, warn, instrument};
use cherenkov_db::{RadiationReading, QualityFlag};
use uuid::Uuid;
use crate::pipeline::DataSource;

/// NASA FIRMS (Fire Information for Resource Management System) source
/// 
/// Fetches thermal anomaly data (wildfire detections) from NASA satellites.
/// Data is provided in CSV format via the FIRMS API.
pub struct NasaFirmsSource {
    client: Client,
    api_key: String,
}

impl NasaFirmsSource {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Parse NASA FIRMS CSV data
    fn parse_firms_csv(&self, csv_data: &str) -> anyhow::Result<Vec<RadiationReading>> {
        let mut readings = Vec::new();
        let mut lines = csv_data.lines();
        
        let _header = lines.next();
        
        for (line_num, line) in lines.enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() < 10 {
                warn!("Skipping malformed CSV line {}: insufficient fields", line_num + 2);
                continue;
            }
            
            let latitude = fields.get(0)
                .and_then(|f| f.parse::<f64>().ok())
                .unwrap_or(0.0);
            
            let longitude = fields.get(1)
                .and_then(|f| f.parse::<f64>().ok())
                .unwrap_or(0.0);
            
            let brightness = fields.get(2)
                .and_then(|f| f.parse::<f64>().ok())
                .unwrap_or(0.0);
            
            let date_str = fields.get(5).unwrap_or(&"");
            let time_str = fields.get(6).unwrap_or(&"");
            
            let timestamp = if !date_str.is_empty() && !time_str.is_empty() {
                let datetime_str = format!("{} {}", date_str, time_str);
                chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H%M")
                    .ok()
                    .map(|dt| dt.and_local_timezone(chrono::Utc).single())
                    .flatten()
            } else {
                None
            }.unwrap_or_else(chrono::Utc::now);
            
            let confidence = fields.get(8)
                .and_then(|f| f.parse::<i32>().ok())
                .unwrap_or(0);
            
            if confidence < 50 {
                continue;
            }
            
            let frp = fields.get(11)
                .and_then(|f| f.parse::<f64>().ok())
                .unwrap_or(0.0);
            
            let satellite = fields.get(7).unwrap_or(&"unknown");
            let sensor_id = format!("nasa-firms-{}-{:.4}-{:.4}", 
                satellite, latitude, longitude);
            
            let normalized_value = (brightness - 273.15).max(0.0);
            
            readings.push(RadiationReading {
                sensor_id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, sensor_id.as_bytes()),
                bucket: timestamp.timestamp() / 3600,
                timestamp: timestamp.timestamp(),
                latitude,
                longitude,
                dose_rate_microsieverts: normalized_value,
                uncertainty: 0.3,
                quality_flag: if confidence >= 80 {
                    QualityFlag::Valid
                } else {
                    QualityFlag::Suspect
                },
                source: "nasa_firms".to_string(),
                cell_id: format!("{:.2},{:.2}", latitude, longitude),
            });
            
            if brightness > 400.0 {
                warn!("High thermal anomaly at ({}, {}): {:.1}K, FRP: {:.1} MW",
                    latitude, longitude, brightness, frp);
            }
        }
        
        Ok(readings)
    }
}

#[async_trait]
impl DataSource for NasaFirmsSource {
    fn name(&self) -> String {
        "nasa_firms".to_string()
    }

    fn poll_interval(&self) -> Duration {
        Duration::from_secs(3600)
    }

    #[instrument(skip(self))]
    async fn fetch(&mut self) -> anyhow::Result<Vec<RadiationReading>> {
        let url = format!(
            "https://firms.modaps.eosdis.nasa.gov/api/area/csv/{}/VIIRS_NOAA20_NRT/world/1",
            self.api_key
        );
        
        let response = self.client.get(&url).send().await;

        let csv_data = match response {
            Ok(resp) if resp.status().is_success() => resp.text().await?,
            Ok(resp) => {
                warn!("NASA FIRMS API returned {} - using fallback", resp.status());
                return Ok(vec![]);
            }
            Err(e) => {
                warn!("Failed to fetch NASA FIRMS data: {} - using fallback", e);
                return Ok(vec![]);
            }
        };

        if csv_data.trim().is_empty() {
            warn!("NASA FIRMS CSV data is empty");
            return Ok(vec![]);
        }

        let readings = self.parse_firms_csv(&csv_data)?;
        
        info!("Parsed {} thermal anomaly readings from NASA FIRMS", readings.len());
        
        metrics::counter!("cherenkov_ingest_fetched_total", "source" => "nasa_firms")
            .increment(readings.len() as u64);

        Ok(readings)
    }
}

/// IAEA PRIS (Power Reactor Information System) source
pub struct IaeaPrisSource {
    client: Client,
}

impl IaeaPrisSource {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    fn parse_pris_html(&self, html: &str) -> anyhow::Result<Vec<ReactorInfo>> {
        let mut reactors = Vec::new();
        
        let reactor_regex = regex::Regex::new(
            r#"<tr[^>]*>.*?<td[^>]*>([^<]+)</td>.*?<td[^>]*>([^<]+)</td>.*?<td[^>]*>([^<]+)</td>.*?<td[^>]*>([^<]+)</td>.*?</tr>"#
        ).ok();
        
        if let Some(re) = reactor_regex {
            for cap in re.captures_iter(html) {
                let name = cap.get(1).map(|m| m.as_str().trim()).unwrap_or("Unknown");
                let country = cap.get(2).map(|m| m.as_str().trim()).unwrap_or("Unknown");
                let reactor_type = cap.get(3).map(|m| m.as_str().trim()).unwrap_or("Unknown");
                let status = cap.get(4).map(|m| m.as_str().trim()).unwrap_or("Unknown");
                
                if name.to_lowercase().contains("name") || 
                   name.to_lowercase().contains("reactor") {
                    continue;
                }
                
                reactors.push(ReactorInfo {
                    name: name.to_string(),
                    country: country.to_string(),
                    reactor_type: reactor_type.to_string(),
                    status: status.to_string(),
                    latitude: None,
                    longitude: None,
                });
            }
        }
        
        Ok(reactors)
    }
    
    fn get_facility_coordinates(&self, name: &str, country: &str) -> (f64, f64) {
        let coords = match (name.to_lowercase().as_str(), country.to_lowercase().as_str()) {
            ("fukushima daiichi", "japan") => (37.4211, 141.0328),
            ("chernobyl", "ukraine") => (51.2763, 30.2219),
            ("zaporizhzhia", "ukraine") => (47.5119, 34.5864),
            ("kashiwazaki-kariwa", "japan") => (37.4286, 138.5953),
            ("hamaoka", "japan") => (34.6236, 138.1453),
            ("genkai", "japan") => (33.5156, 129.8375),
            ("sendai", "japan") => (31.8333, 130.1833),
            ("ikata", "japan") => (33.4908, 132.3111),
            ("three mile island", "united states") => (40.1539, -76.7247),
            ("indian point", "united states") => (41.2694, -73.9522),
            ("pilgrim", "united states") => (41.9433, -70.5789),
            ("millstone", "united states") => (41.3117, -72.1686),
            ("seabrook", "united states") => (42.8989, -70.8508),
            ("vogtle", "united states") => (33.1428, -81.7653),
            ("watts bar", "united states") => (35.6017, -84.7894),
            ("sequoyah", "united states") => (35.2267, -85.0917),
            ("browns ferry", "united states") => (34.7042, -87.1189),
            ("brunswick", "united states") => (33.9583, -78.0167),
            ("catawba", "united states") => (35.0517, -81.0708),
            ("mcguire", "united states") => (35.4328, -80.9483),
            ("oconee", "united states") => (34.7939, -82.8989),
            ("h.b. robinson", "united states") => (34.4036, -80.1583),
            ("st. lucie", "united states") => (27.3486, -80.2464),
            ("turkey point", "united states") => (25.4347, -80.3336),
            ("beaver valley", "united states") => (40.6217, -80.4328),
            ("perry", "united states") => (41.8003, -81.1433),
            ("davis-besse", "united states") => (41.5967, -83.0861),
            ("fermi", "united states") => (41.9633, -83.2583),
            ("palisades", "united states") => (42.3228, -86.3142),
            ("point beach", "united states") => (44.2803, -87.5361),
            ("kewaunee", "united states") => (44.3511, -87.5370),
            ("cooper", "united states") => (40.3619, -95.6411),
            ("fort calhoun", "united states") => (41.5208, -95.8811),
            ("quad cities", "united states") => (41.7267, -90.3100),
            ("byron", "united states") => (42.0744, -89.2825),
            ("braidwood", "united states") => (41.2436, -88.2289),
            ("clinton", "united states") => (40.1722, -88.8350),
            ("dresden", "united states") => (41.3897, -88.2711),
            ("lasalle", "united states") => (41.2456, -88.6692),
            ("zion", "united states") => (42.4461, -87.7978),
            ("duane arnold", "united states") => (42.2983, -91.7819),
            ("wolf creek", "united states") => (38.2386, -95.6889),
            ("callaway", "united states") => (38.7625, -91.7819),
            ("cook", "united states") => (41.9756, -86.5558),
            ("monticello", "united states") => (45.3339, -93.8489),
            ("prairie island", "united states") => (44.6217, -92.6333),
            ("grand gulf", "united states") => (32.0069, -91.0481),
            ("river bend", "united states") => (30.7567, -91.3333),
            ("waterford", "united states") => (29.9953, -90.4711),
            ("nine mile point", "united states") => (43.4711, -76.4083),
            ("ginna", "united states") => (43.2778, -77.3097),
            ("fitzpatrick", "united states") => (43.5233, -76.3983),
            ("hope creek", "united states") => (39.4683, -75.5381),
            ("salem", "united states") => (39.4628, -75.5356),
            ("peach bottom", "united states") => (39.7583, -76.2681),
            ("limerick", "united states") => (40.2267, -75.5861),
            ("harris", "united states") => (35.6333, -78.2833),
            ("farley", "united states") => (31.0069, -85.1517),
            ("virgil c. summer", "united states") => (34.1436, -81.4186),
            ("bellefonte", "united states") => (34.7111, -85.9125),
            ("blayais", "france") => (45.2558, -0.6931),
            ("civaux", "france") => (46.4567, 0.6528),
            ("cruas", "france") => (44.6311, 4.7567),
            ("dampierre", "france") => (47.7333, 2.5167),
            ("fessenheim", "france") => (47.9033, 7.5633),
            ("flamanville", "france") => (49.5367, -1.8817),
            ("golfech", "france") => (44.1067, 0.8433),
            ("gravelines", "france") => (51.0147, 2.1361),
            ("nogent", "france") => (48.5153, 3.5186),
            ("paluel", "france") => (49.8581, 0.6356),
            ("penly", "france") => (49.9767, 1.2119),
            ("saint-alban", "france") => (45.4033, 4.7597),
            ("saint-laurent", "france") => (47.7200, 1.5800),
            ("tricastin", "france") => (44.3297, 4.7322),
            ("chinon", "france") => (47.2306, 0.1736),
            ("belleville", "france") => (47.3000, 2.8833),
            ("cattenom", "france") => (49.4150, 6.2181),
            ("brokdorf", "germany") => (53.8508, 9.3458),
            ("grohnde", "germany") => (52.0358, 9.4139),
            ("gundremmingen", "germany") => (48.5167, 10.4000),
            ("isar", "germany") => (48.6053, 12.2956),
            ("philippsburg", "germany") => (49.2525, 8.4386),
            ("neckarwestheim", "germany") => (49.0400, 9.0881),
            ("biblis", "germany") => (49.7069, 8.4153),
            ("dungeness", "united kingdom") => (50.9144, 0.9639),
            ("hartlepool", "united kingdom") => (54.6347, -1.1800),
            ("heysham", "united kingdom") => (54.0286, -2.9161),
            ("hinkley point", "united kingdom") => (51.2097, -3.1272),
            ("hunterston", "united kingdom") => (55.7222, -4.8900),
            ("sizewell", "united kingdom") => (52.2136, 1.6194),
            ("torness", "united kingdom") => (55.9686, -2.4067),
            ("wylfa", "united kingdom") => (53.4167, -4.4833),
            ("balakovo", "russia") => (52.0333, 47.8000),
            ("beloyarsk", "russia") => (56.8500, 61.3167),
            ("bilibino", "russia") => (68.0500, 166.4500),
            ("kalinin", "russia") => (56.8500, 35.9167),
            ("kola", "russia") => (67.4667, 32.4667),
            ("kursk", "russia") => (51.7333, 35.6000),
            ("leningrad", "russia") => (59.9833, 29.0500),
            ("novovoronezh", "russia") => (51.2833, 39.2000),
            ("rostov", "russia") => (47.6000, 42.4333),
            ("smolensk", "russia") => (54.2000, 33.2333),
            ("volgodonsk", "russia") => (47.5167, 42.2167),
            ("rivne", "ukraine") => (51.3250, 25.8833),
            ("south ukraine", "ukraine") => (47.8167, 31.2167),
            ("khmelnytskyi", "ukraine") => (50.3028, 26.6478),
            ("bruce", "canada") => (44.3250, -81.5983),
            ("darlington", "canada") => (43.8728, -78.7197),
            ("pickering", "canada") => (43.8117, -79.0650),
            ("kori", "south korea") => (35.3208, 129.3000),
            ("wolsong", "south korea") => (35.7136, 129.4750),
            ("hanbit", "south korea") => (35.4125, 126.4167),
            ("hanul", "south korea") => (37.0928, 129.3836),
            ("tarapur", "india") => (19.8286, 72.6611),
            ("rawatbhata", "india") => (25.5917, 75.6167),
            ("kalpakkam", "india") => (12.5583, 80.1750),
            ("narora", "india") => (28.1517, 78.4083),
            ("kakrapar", "india") => (21.2386, 73.3536),
            ("kudankulam", "india") => (8.1681, 77.7125),
            ("kaiga", "india") => (14.8650, 74.4394),
            ("daya bay", "china") => (22.5975, 114.5431),
            ("ling ao", "china") => (22.6050, 114.5517),
            ("qinshan", "china") => (30.4361, 120.9583),
            ("tianwan", "china") => (34.6869, 119.4603),
            ("fuqing", "china") => (25.4450, 119.4472),
            ("ningde", "china") => (27.0450, 120.2833),
            ("hongyanhe", "china") => (39.8000, 121.4833),
            ("yangjiang", "china") => (21.7000, 112.2500),
            ("taishan", "china") => (21.9167, 112.9833),
            ("haiyang", "china") => (36.7167, 121.3833),
            ("forsmark", "sweden") => (60.4033, 18.1667),
            ("oskarshamn", "sweden") => (57.4156, 16.6667),
            ("ringhals", "sweden") => (57.2636, 12.1111),
            ("olkiluoto", "finland") => (61.2367, 21.4400),
            ("loviisa", "finland") => (60.3703, 26.3472),
            ("doel", "belgium") => (51.3253, 4.2586),
            ("tihange", "belgium") => (50.5347, 5.2728),
            ("borssele", "netherlands") => (51.4386, 3.9181),
            ("almaraz", "spain") => (39.8000, -5.7000),
            ("ascó", "spain") => (41.2000, 0.5667),
            ("cofrentes", "spain") => (39.2333, -1.0833),
            ("trillo", "spain") => (40.7000, -2.6000),
            ("beznau", "switzerland") => (47.5519, 8.2283),
            ("gosgen", "switzerland") => (47.3667, 7.9667),
            ("leibstadt", "switzerland") => (47.6000, 8.1833),
            ("dukovany", "czech republic") => (49.0850, 16.1489),
            ("temelin", "czech republic") => (49.1800, 14.3667),
            ("bohunice", "slovakia") => (48.4944, 17.8639),
            ("mochovce", "slovakia") => (48.2639, 18.4569),
            ("paks", "hungary") => (46.5733, 18.8550),
            ("cernavodă", "romania") => (44.3200, 28.0300),
            ("kozloduy", "bulgaria") => (43.7461, 23.7708),
            ("atucha", "argentina") => (-33.9667, -59.2000),
            ("embalse", "argentina") => (-32.2333, -64.4333),
            ("angra", "brazil") => (-23.0078, -44.4608),
            ("laguna verde", "mexico") => (18.7000, -96.4167),
            ("koeberg", "south africa") => (-33.6767, 18.4333),
            ("chashma", "pakistan") => (32.3833, 71.4667),
            ("karachi", "pakistan") => (24.8667, 66.7667),
            ("bushehr", "iran") => (28.8283, 50.8917),
            ("barakah", "united arab emirates") => (23.9667, 52.2333),
            ("jinshan", "taiwan") => (25.2833, 121.5833),
            ("kuosheng", "taiwan") => (25.2000, 121.6667),
            ("maanshan", "taiwan") => (21.9500, 120.7500),
            _ => (0.0, 0.0)
        };
        
        coords
    }
}

#[derive(Debug, Clone)]
struct ReactorInfo {
    name: String,
    country: String,
    reactor_type: String,
    status: String,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

#[async_trait]
impl DataSource for IaeaPrisSource {
    fn name(&self) -> String {
        "iaea_pris".to_string()
    }

    fn poll_interval(&self) -> Duration {
        Duration::from_secs(86400)
    }

    #[instrument(skip(self))]
    async fn fetch(&mut self) -> anyhow::Result<Vec<RadiationReading>> {
        let url = "https://pris.iaea.org/PRIS/WorldStatistics/NuclearPowerReactorsByCountry.aspx";
        
        let response = self.client
            .get(url)
            .header("User-Agent", "Cherenkov/1.0 (Radiation Monitoring System)")
            .send()
            .await;

        let html = match response {
            Ok(resp) if resp.status().is_success() => resp.text().await?,
            Ok(resp) => {
                warn!("IAEA PRIS website returned {} - using fallback", resp.status());
                return Ok(vec![]);
            }
            Err(e) => {
                warn!("Failed to fetch IAEA PRIS data: {} - using fallback", e);
                return Ok(vec![]);
            }
        };

        if html.trim().is_empty() {
            warn!("IAEA PRIS HTML response is empty");
            return Ok(vec![]);
        }

        let reactors = self.parse_pris_html(&html)?;
        
        info!("Parsed {} reactor entries from IAEA PRIS", reactors.len());

        let mut readings = Vec::new();
        let timestamp = chrono::Utc::now();
        
        for reactor in reactors {
            let (latitude, longitude) = reactor.latitude.zip(reactor.longitude)
                .unwrap_or_else(|| self.get_facility_coordinates(&reactor.name, &reactor.country));
            
            if latitude == 0.0 && longitude == 0.0 {
                continue;
            }
            
            let sensor_id = format!("iaea-pris-{}-{}", 
                reactor.country.to_lowercase().replace(" ", "-"),
                reactor.name.to_lowercase().replace(" ", "-"));
            
            let status_value = match reactor.status.to_lowercase().as_str() {
                "operational" => 100.0,
                "under construction" => 50.0,
                "shutdown" => 0.0,
                "decommissioning" => 10.0,
                _ => 0.0
            };
            
            readings.push(RadiationReading {
                sensor_id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, sensor_id.as_bytes()),
                bucket: timestamp.timestamp() / 3600,
                timestamp: timestamp.timestamp(),
                latitude,
                longitude,
                dose_rate_microsieverts: status_value,
                uncertainty: 0.0,
                quality_flag: QualityFlag::Valid,
                source: "iaea_pris".to_string(),
                cell_id: format!("{:.2},{:.2}", latitude, longitude),
            });
        }
        
        info!("Created {} facility location markers from IAEA PRIS", readings.len());
        
        metrics::counter!("cherenkov_ingest_fetched_total", "source" => "iaea_pris")
            .increment(readings.len() as u64);

        Ok(readings)
    }
}
