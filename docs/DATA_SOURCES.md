# Data Sources

Cherenkov aggregates radiation data from 50,000+ sensors across multiple public and private sources.

## Active Sources

### Safecast
- **URL**: https://api.safecast.org/
- **Sensors**: 5,000+
- **Coverage**: Global
- **Update Frequency**: 5 minutes
- **Format**: JSON API
- **License**: CC0

### uRADMonitor
- **URL**: https://www.uradmonitor.com/
- **Sensors**: 10,000+
- **Coverage**: Global
- **Update Frequency**: 1 minute
- **Format**: JSON API
- **License**: Commercial (API key required)

### EPA RadNet
- **URL**: https://www.epa.gov/radnet
- **Sensors**: 140+
- **Coverage**: United States
- **Update Frequency**: Hourly
- **Format**: XML/CSV
- **License**: Public Domain

### EURDEP
- **URL**: https://eurdep.jrc.ec.europa.eu/
- **Sensors**: 5,000+
- **Coverage**: European Union
- **Update Frequency**: 10 minutes
- **Format**: XML
- **License**: Public

### IAEA PRIS
- **URL**: https://pris.iaea.org/
- **Facilities**: 440 nuclear power plants
- **Coverage**: Global
- **Update Frequency**: Daily
- **Format**: JSON API
- **License**: Public

## Seismic Correlation

### USGS Earthquake Hazards
- **URL**: https://earthquake.usgs.gov/fdsnws/event/1/
- **Coverage**: Global
- **Update Frequency**: Real-time
- **Purpose**: Seismic-radiation event correlation

## Weather Data

### NOAA GFS
- **URL**: https://www.ncei.noaa.gov/products/weather-climate-models/global-forecast
- **Resolution**: 0.25 degree
- **Update Frequency**: 6 hours
- **Purpose**: Plume dispersion modeling

### ECMWF
- **URL**: https://www.ecmwf.int/
- **Resolution**: 0.1 degree
- **Update Frequency**: 12 hours
- **Purpose**: High-resolution weather for Europe

## Satellite Data

### NASA FIRMS
- **URL**: https://firms.modaps.eosdis.nasa.gov/
- **Coverage**: Global
- **Update Frequency**: Near real-time
- **Purpose**: Thermal anomaly detection (potential nuclear events)

## Implementation

Each source has a dedicated crawler in `crates/cherenkov-ingest/src/sources/`:

```
sources/
├── safecast.rs
├── uradmonitor.rs
├── epa_radnet.rs
├── eurdep.rs
├── iaea_pris.rs
├── usgs_seismic.rs
├── noaa_gfs.rs
└── nasa_firms.rs
```

## Data Normalization

All sources are normalized to the `NormalizedReading` schema:

```rust
pub struct NormalizedReading {
    pub sensor_id: String,
    pub timestamp: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub dose_rate: f64,        // Microsieverts per hour
    pub unit: String,
    pub quality: QualityFlag,
    pub source: DataSource,
}
