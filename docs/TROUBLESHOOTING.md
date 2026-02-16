# Cherenkov Troubleshooting Guide

This guide provides solutions for common issues encountered when using the Cherenkov radiological intelligence interface.

## Quick Diagnostics

Before troubleshooting, check these basic items:

1. **Browser Console**: Open DevTools (F12) and check for errors
2. **Network Tab**: Verify WebSocket connection status
3. **System Resources**: Check CPU/RAM usage
4. **Browser Version**: Ensure you're using a supported browser

## WebGL and Globe Issues

### Globe Not Rendering (Black Screen)

**Symptoms**: 3D globe area is black or empty

**Solutions**:

1. **Verify WebGL 2.0 Support**
   ```javascript
   // Run in browser console
   const canvas = document.createElement('canvas');
   const gl = canvas.getContext('webgl2');
   console.log('WebGL 2.0:', gl ? 'Supported' : 'Not supported');
   ```

2. **Update Graphics Drivers**
   - NVIDIA: https://www.nvidia.com/drivers
   - AMD: https://www.amd.com/support
   - Intel: https://www.intel.com/content/www/us/en/download-center

3. **Enable Hardware Acceleration**
   - Chrome: Settings > System > Use hardware acceleration when available
   - Firefox: about:config > webgl.disable-false
   - Edge: Settings > System > Use hardware acceleration

4. **Disable Conflicting Extensions**
   - Ad blockers
   - Privacy extensions
   - WebGL blockers

### Poor Globe Performance (Low FPS)

**Symptoms**: Stuttering, laggy rotation, delayed response

**Solutions**:

1. **Enable Performance Mode**
   - Settings > Globe > Performance Mode: ON
   - Reduces particle count and shader complexity

2. **Reduce Layer Complexity**
   - Disable "Sensor Heatmap" (computationally expensive)
   - Disable "Plume Simulation" when not needed
   - Reduce "Sensor Density" in settings

3. **Browser Optimizations**
   ```css
   /* Add to browser custom CSS if supported */
   .globe-canvas {
     will-change: transform;
     contain: strict;
   }
   ```

4. **System-Level Fixes**
   - Close unnecessary browser tabs
   - Disable browser animations
   - Ensure GPU is being used (not integrated graphics)

## WebSocket Connection Issues

### Connection Dropped / Reconnecting Loop

**Symptoms**: Red connection indicator, "DISCONNECTED" status, frequent reconnects

**Solutions**:

1. **Check Network Stability**
   ```bash
   # Test WebSocket endpoint
   curl -i -N \
     -H "Connection: Upgrade" \
     -H "Upgrade: websocket" \
     -H "Host: your-cherenkov-instance" \
     -H "Origin: https://your-cherenkov-instance" \
     https://your-cherenkov-instance/ws
   ```

2. **Firewall / Proxy Issues**
   - Ensure port 443 (WSS) is open
   - Check corporate proxy settings
   - Verify WebSocket upgrade headers aren't stripped

3. **Browser-Specific Fixes**
   
   **Chrome**:
   - chrome://flags/#enable-websocket-multiplexing
   - Disable "Experimental Web Platform features" if enabled
   
   **Firefox**:
   - about:config > network.websocket.max-connections
   - Increase to 200 if low

4. **Server-Side Check**
   ```bash
   # Check WebSocket server logs
   kubectl logs -f deployment/cherenkov-api -n cherenkov
   
   # Verify service is running
   kubectl get svc cherenkov-api -n cherenkov
   ```

### No Real-Time Updates

**Symptoms**: Data appears static, no new alerts, timestamp not updating

**Solutions**:

1. **Verify Subscription**
   ```javascript
   // In browser console
   const ws = new WebSocket('wss://your-instance/ws');
   ws.onmessage = (e) => console.log('Message:', e.data);
   ```

2. **Check Time Mode**
   - Ensure not in "PAUSED" or "REPLAY" mode
   - Click "LIVE" button in time controls

3. **Clear Cache**
   - Hard refresh: Ctrl+Shift+R (Windows) or Cmd+Shift+R (Mac)
   - Clear site data in DevTools > Application > Clear storage

## Data Display Issues

### Sensors Not Appearing on Globe

**Symptoms**: Empty globe, no sensor markers visible

**Solutions**:

1. **Check Layer Visibility**
   - Verify "Sensor Points" or "Sensor Heatmap" layer is enabled
   - Toggle layer off/on to refresh

2. **Verify Data Ingestion**
   ```graphql
   # Run in GraphQL playground
   query {
     sensors(limit: 10) {
       id
       location {
         lat
         lon
       }
       lastReading {
         value
         timestamp
       }
     }
   }
   ```

3. **Time Window Check**
   - Ensure time slider includes current period
   - Check if viewing historical data (sensors may not exist in past)

4. **Geographic Filter**
   - Reset globe view to global (press 'R')
   - Check if zoomed into empty region

### Incorrect Radiation Readings

**Symptoms**: Impossible values (negative, extremely high), unit confusion

**Solutions**:

1. **Unit Conversion**
   - Check settings for display units (μSv/h vs mSv/h)
   - Conversion: 1 mSv/h = 1000 μSv/h

2. **Sensor Calibration**
   - Some sensors report raw counts, not dose
   - Check sensor metadata for calibration factor

3. **Data Validation**
   ```javascript
   // Check sensor status in store
   const sensor = useAppStore.getState().sensors['sensor-id'];
   console.log('Status:', sensor.status);
   console.log('Last update:', sensor.lastReading);
   ```

## UI and Interface Issues

### Layout Broken / Overlapping Elements

**Symptoms**: Panels overlapping, text cut off, misaligned buttons

**Solutions**:

1. **Viewport Check**
   - Minimum recommended: 1280x720
   - Optimal: 1920x1080 or higher

2. **Zoom Level**
   - Reset browser zoom: Ctrl+0 (Windows) or Cmd+0 (Mac)
   - Check OS display scaling (100% recommended)

3. **CSS Reset**
   ```javascript
   // Force layout recalculation
   document.body.style.display = 'none';
   document.body.offsetHeight; // Trigger reflow
   document.body.style.display = '';
   ```

### Sidebar / Panels Not Responding

**Symptoms**: Clicking navigation does nothing, panels won't open/close

**Solutions**:

1. **State Reset**
   ```javascript
   // Clear Zustand store
   localStorage.removeItem('cherenkov-storage');
   location.reload();
   ```

2. **Event Listener Check**
   ```javascript
   // Verify click handlers
   document.querySelector('nav').addEventListener('click', (e) => {
     console.log('Clicked:', e.target);
   });
   ```

3. **Component Remount**
   - Toggle "React Developer Tools" component highlighting
   - Force re-render by switching views twice

## Alert System Issues

### Alerts Not Appearing

**Symptoms**: No notifications, empty alert feed, missing critical alerts

**Solutions**:

1. **Notification Permissions**
   - Browser settings > Site settings > Notifications
   - Allow notifications for Cherenkov domain

2. **Severity Filter**
   - Check minimum severity threshold in settings
   - Switch to "All" tab in alert feed

3. **WebSocket Verification**
   - Alerts require active WebSocket connection
   - See "WebSocket Connection Issues" section above

4. **Alert Acknowledgment Check**
   - Unacknowledged alerts show in bold
   - Acknowledged alerts may be filtered out

### False Positive Alerts

**Symptoms**: Too many alerts, normal readings flagged as anomalies

**Solutions**:

1. **Adjust Sensitivity**
   - Settings > Alerts > Detection Sensitivity
   - Increase threshold (less sensitive)

2. **ML Model Retraining**
   - Contact admin to retrain anomaly detection model
   - Provide examples of false positives

3. **Sensor-Specific Rules**
   - Some sensors naturally have higher variance
   - Add to whitelist or adjust individual thresholds

## Performance and Resource Issues

### High Memory Usage

**Symptoms**: Browser slowing down, "Out of memory" errors, tab crashes

**Solutions**:

1. **Limit Data History**
   - Settings > Data > History Window: Reduce to 6 hours
   - Clear old data: DevTools > Application > Local Storage > Clear

2. **Reduce Sensor Count**
   - Settings > Globe > Sensor Density: Low
   - Disable "Show Inactive Sensors"

3. **Browser Profile**
   ```javascript
   // Check memory usage
   performance.memory.usedJSHeapSize / 1048576 + ' MB';
   ```

### High CPU Usage

**Symptoms**: Fan spinning, hot laptop, battery draining fast

**Solutions**:

1. **Limit Frame Rate**
   ```javascript
   // Add to console to throttle rendering
   const canvas = document.querySelector('canvas');
   canvas.getContext('webgl2', { desynchronized: true });
   ```

2. **Disable Animations**
   - Settings > Display > Reduce Motion: ON
   - Disable "Auto-rotate" in globe settings

3. **Background Throttling**
   - Chrome: chrome://flags/#enable-throttle-display-none-and-visibility-hidden-cross-origin-iframes
   - Enable "Throttle JavaScript timers in background"

## Authentication and Access Issues

### Login Failures

**Symptoms**: "Authentication failed", redirect loops, session expired

**Solutions**:

1. **Token Refresh**
   ```javascript
   // Force token refresh
   localStorage.removeItem('cherenkov-token');
   location.href = '/login';
   ```

2. **Cookie Settings**
   - Enable third-party cookies for auth domain
   - Check "Block third-party cookies" is disabled

3. **CORS Issues**
   - Verify API and web origins match configuration
   - Check `Access-Control-Allow-Origin` headers

### Permission Denied Errors

**Symptoms**: 403 errors, "Unauthorized" messages, missing data

**Solutions**:

1. **Role Verification**
   - Check user role in admin panel
   - Verify ABAC policies allow access to requested resources

2. **Token Claims**
   ```javascript
   // Decode JWT to check claims
   const token = localStorage.getItem('cherenkov-token');
   const payload = JSON.parse(atob(token.split('.')[1]));
   console.log('Roles:', payload.roles);
   console.log('Permissions:', payload.permissions);
   ```

## WASM-Specific Issues

### WASM Module Load Failure

**Symptoms**: "Failed to load WASM", globe fallback to 2D mode

**Solutions**:

1. **MIME Type Check**
   - Server must serve `.wasm` files with `application/wasm`
   - Check nginx/Apache configuration

2. **CORS Headers**
   ```
   Access-Control-Allow-Origin: *
   Access-Control-Allow-Methods: GET, POST, OPTIONS
   ```

3. **Integrity Check**
   ```bash
   # Verify WASM file integrity
   sha256sum cherenkov_web.wasm
   # Compare with build manifest
   ```

4. **Browser Support**
   - Safari < 15: Limited WASM support
   - Enable "Experimental Features" in Safari settings

### WASM Runtime Errors

**Symptoms**: "RuntimeError: memory access out of bounds", crashes

**Solutions**:

1. **Memory Limits**
   ```javascript
   // Increase memory allocation
   const memory = new WebAssembly.Memory({
     initial: 256,
     maximum: 512,
     shared: true
   });
   ```

2. **Browser Console Check**
   - Look for "unreachable" or "undefined" errors
   - Check for stack overflow messages

3. **Rebuild WASM**
   ```bash
   cd web/wasm
   wasm-pack build --target web --release
   ```

## Mobile and Touch Issues

### Touch Controls Not Working

**Symptoms**: Cannot pan/zoom on mobile, buttons unresponsive

**Solutions**:

1. **Touch Event Polyfills**
   ```javascript
   // Enable touch events
   document.addEventListener('touchstart', {}, {passive: true});
   ```

2. **Viewport Meta Tag**
   ```html
   <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
   ```

3. **Gesture Conflicts**
   - Disable browser swipe navigation
   - Use two-finger zoom instead of pinch

### Mobile Performance

**Symptoms**: Slow on phones/tablets, battery drain, overheating

**Solutions**:

1. **Mobile Mode**
   - Automatically enabled on screens < 768px
   - Uses 2D map instead of 3D globe

2. **Reduce Quality**
   - Settings > Performance > Mobile Optimization: ON
   - Disables particle effects and complex shaders

3. **Battery Saver**
   - Enable "Low Power Mode" detection
   - Reduces update frequency automatically

## Error Codes Reference

| Code | Meaning | Solution |
|------|---------|----------|
| E1001 | WebGL Context Lost | Refresh page, check GPU drivers |
| E1002 | WebSocket Timeout | Check network, increase timeout |
| E1003 | WASM Load Failed | Check server config, clear cache |
| E1004 | GraphQL Error | Check query syntax, auth token |
| E1005 | Sensor Data Stale | Check ingestion pipeline |
| E1006 | ML Inference Error | Retrain model, check input data |
| E1007 | Plume Calculation Failed | Check weather data source |
| E1008 | Database Timeout | Check ScyllaDB cluster health |

## Getting Additional Help

If issues persist after trying these solutions:

1. **Collect Diagnostics**
   ```javascript
   // Run in console and save output
   console.log('User Agent:', navigator.userAgent);
   console.log('WebGL:', !!document.createElement('canvas').getContext('webgl2'));
   console.log('Screen:', window.screen.width + 'x' + window.screen.height);
   console.log('Memory:', performance.memory?.usedJSHeapSize);
   ```

2. **Check Logs**
   - Browser console (F12)
   - Server logs (if self-hosted)
   - Network tab for failed requests

3. **Report Issue**
   - GitHub: https://github.com/tworjaga/Cherenkov/issues
   - Include: Browser version, OS, error messages, diagnostics
   - Telegram: @al7exy

---

*Document Version: 1.0*
*Last Updated: 2025*
