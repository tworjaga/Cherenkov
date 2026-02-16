# Cherenkov User Manual

## Table of Contents
1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Interface Overview](#interface-overview)
4. [Globe View](#globe-view)
5. [Alert Management](#alert-management)
6. [Time Controls](#time-controls)
7. [Sensor Data](#sensor-data)
8. [Settings](#settings)
9. [Keyboard Shortcuts](#keyboard-shortcuts)
10. [Troubleshooting](#troubleshooting)

## Introduction

Cherenkov is a production-grade radiological intelligence interface for monitoring global radiation levels, nuclear facilities, and environmental anomalies. This manual covers all features and functionality of the web interface.

## Getting Started

### System Requirements
- Modern web browser (Chrome 90+, Firefox 88+, Safari 14+, Edge 90+)
- WebGL 2.0 support for 3D globe visualization
- Minimum 4GB RAM
- Stable internet connection (1 Mbps minimum)

### Accessing the Application
1. Navigate to the Cherenkov URL provided by your administrator
2. Log in with your credentials
3. The dashboard will load with the default globe view

## Interface Overview

The Cherenkov interface consists of five main areas:

```
┌─────────────────────────────────────────┐
│              HEADER (56px)              │
│  Logo | DEFCON Status | Connection | UTC │
├──────────┬──────────────────────┬────────┤
│          │                      │        │
│ SIDEBAR  │    MAIN VIEWPORT    │ RIGHT  │
│ (64px)   │                     │ PANEL  │
│          │   3D WEBGL GLOBE    │(320px) │
│ [D]      │                     │        │
│ [G]      │  ┌───────────────┐  │Alerts  │
│ [S]      │  │ Overlay       │  │Sensor  │
│ [A]      │  │ Controls      │  │Details │
│ [P]      │  └───────────────┘  │        │
│ [C]      │                     │        │
├──────────┴──────────────────────┴────────┤
│         BOTTOM PANEL (200px)              │
│  [Global Chart] [Stats] [Alert Feed]      │
└────────────────────────────────────────────┘
```

### Header
- **Logo**: Click to return to dashboard
- **DEFCON Status**: Current global threat level (1-5)
- **Connection**: WebSocket connection status
- **UTC Time**: Current time in UTC
- **Notifications**: Alert notifications bell
- **User Menu**: Profile and logout options

### Sidebar Navigation
- **D** - Dashboard: Overview with key metrics
- **G** - Globe: 3D visualization of global radiation
- **S** - Sensors: List view of all sensors
- **A** - Anomalies: Detected radiation anomalies
- **P** - Plume: Dispersion simulation view
- **C** - Settings: User preferences and configuration

## Globe View

The 3D globe is the primary visualization interface for radiation data.

### Navigation Controls
| Action | Control |
|--------|---------|
| Rotate | Left-click + drag |
| Pan | Right-click + drag |
| Zoom | Scroll wheel |
| Reset View | Click reset button (bottom-left) |

### Layer Controls
Toggle data layers using buttons in the top-right overlay:

- **Sensor Heatmap**: Hexagon aggregation of sensor readings
- **Sensor Points**: Individual sensor locations
- **Facilities**: Nuclear power plants and research facilities
- **Plume Simulation**: Dispersion model output
- **Anomalies**: Detected radiation events
- **Seismic**: Earthquake data correlation

### Interacting with Data
- **Hover**: Shows tooltip with sensor ID and current reading
- **Click**: Selects sensor/facility and opens details in right panel
- **Double-click**: Zooms to location

### Color Legend
| Color | Radiation Level |
|-------|----------------|
| Green (#00ff88) | Normal (< 0.1 μSv/h) |
| Blue (#00d4ff) | Low (0.1 - 0.5 μSv/h) |
| Yellow (#ffb800) | Medium (0.5 - 1.0 μSv/h) |
| Orange (#ff6b35) | High (1.0 - 5.0 μSv/h) |
| Red (#ff3366) | Critical (> 5.0 μSv/h) |

## Alert Management

### Alert Feed (Bottom Panel)
The alert feed displays real-time notifications of radiation events.

#### Alert Severity Levels
- **CRITICAL** (Red): Immediate threat, requires action
- **HIGH** (Orange): Elevated readings, investigation needed
- **MEDIUM** (Yellow): Unusual patterns, monitoring required
- **LOW** (Blue): Informational, within normal variance

#### Managing Alerts
1. **View Alert**: Click to center globe on location
2. **Acknowledge**: Press 'A' key or click checkmark
3. **Filter**: Use tabs to show All, Unread, Critical, or High alerts
4. **Dismiss**: Click X to remove from feed

### Alert Details
Each alert card shows:
- Severity indicator (colored border)
- Title and description
- Location coordinates
- Timestamp
- Associated sensor/facility ID

## Time Controls

The time slider controls temporal data visualization.

### Modes
- **LIVE**: Real-time data streaming (default)
- **PAUSED**: Static view of current moment
- **REPLAY**: Historical data playback

### Playback Controls
- **Play/Pause**: Toggle time advancement
- **Speed**: 1x, 2x, 5x, 10x, 50x real-time
- **Scrubber**: Drag to specific time point
- **Window**: Adjust visible time range

### Using Time Controls
1. Click mode button to switch between LIVE/PAUSED/REPLAY
2. In REPLAY mode, use play/pause to control animation
3. Adjust speed for faster/slower playback
4. Drag timeline to jump to specific moments

## Sensor Data

### Sensor Information Panel (Right Panel)
When a sensor is selected, the right panel displays:

- **Sensor ID**: Unique identifier
- **Location**: Latitude and longitude
- **Current Reading**: μSv/h value with unit
- **Status**: Active, inactive, or error
- **Trend**: 24-hour sparkline chart
- **History**: Link to detailed historical data

### Reading Interpretation
Normal background radiation varies by location:
- **Typical background**: 0.05 - 0.20 μSv/h
- **Elevated**: 0.20 - 1.00 μSv/h
- **High**: 1.00 - 10.00 μSv/h
- **Dangerous**: > 10.00 μSv/h

## Settings

Access settings via the sidebar 'C' button or user menu.

### Display Settings
- **Theme**: Dark (default) or Light mode
- **Units**: μSv/h, mSv/h, or nSv/h
- **Time Format**: 12h or 24h
- **Date Format**: Locale-based options

### Notification Settings
- **Browser Notifications**: Enable/disable
- **Alert Sounds**: Enable/disable
- **Minimum Severity**: Filter alert notifications

### Globe Settings
- **Default View**: Initial camera position
- **Auto-rotate**: Enable automatic rotation
- **Performance Mode**: Reduce visual quality for better FPS

### Data Settings
- **Auto-refresh**: Data update interval
- **History Window**: Default time range
- **Sensor Density**: Clustering threshold

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `1-6` | Switch views (Dashboard, Globe, Sensors, Anomalies, Plume, Settings) |
| `G` | Focus globe view |
| `A` | Acknowledge selected alert |
| `Esc` | Close panels / Deselect |
| `Space` | Play/Pause time |
| `+` / `-` | Zoom in/out on globe |
| `R` | Reset globe view |
| `F` | Toggle fullscreen |
| `?` | Show keyboard shortcuts help |

## Troubleshooting

### Common Issues

#### Globe Not Loading
1. Check WebGL 2.0 support: Visit https://get.webgl.org/webgl2/
2. Update graphics drivers
3. Disable browser extensions that block WebGL
4. Try a different browser

#### No Data Displayed
1. Check connection status in header
2. Verify WebSocket connection (green dot)
3. Refresh the page
4. Check if sensors are selected in layer controls

#### Slow Performance
1. Enable Performance Mode in settings
2. Reduce sensor density
3. Disable unnecessary layers
4. Close other browser tabs
5. Check system resources (RAM/CPU)

#### Alerts Not Appearing
1. Check notification settings
2. Verify minimum severity threshold
3. Check browser notification permissions
4. Ensure WebSocket is connected

### Error Messages

| Error | Solution |
|-------|----------|
| "WebSocket Disconnected" | Check network connection, refresh page |
| "WASM Load Failed" | Clear browser cache, reload |
| "No Sensors Available" | Wait for data ingestion, check filters |
| "Render Error" | Reduce quality settings, update drivers |

### Getting Help
- **Documentation**: https://github.com/tworjaga/Cherenkov/docs
- **Issues**: https://github.com/tworjaga/Cherenkov/issues
- **Contact**: Telegram @al7exy

---

*Document Version: 1.0*
*Last Updated: 2025*
