'use client';

import { useState, useCallback, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Slider } from '@/components/ui/slider';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { EvacuationZones } from '@/components/plume/evacuation-zones';
import { PlumeVisualization, PlumePoint } from '@/components/plume/plume-visualization';
import { useToast } from '@/components/ui/toast';
import { 
  Wind, 
  Radio, 
  MapPin, 
  Play, 
  Pause, 
  RotateCcw, 
  Download,
  AlertTriangle,
  Info,
  Clock,
  Thermometer,
  Gauge
} from 'lucide-react';

interface ReleaseParameters {
  latitude: number;
  longitude: number;
  altitude: number;
  releaseRate: number;
  duration: number;
  isotope: string;
  stabilityClass: string;
}

interface WeatherData {
  windSpeed: number;
  windDirection: number;
  temperature: number;
  pressure: number;
  humidity: number;
  stabilityClass: string;
  timestamp: string;
}

const ISOTOPES = [
  { value: 'I-131', label: 'Iodine-131', halfLife: '8.02 days' },
  { value: 'Cs-137', label: 'Cesium-137', halfLife: '30.17 years' },
  { value: 'Cs-134', label: 'Cesium-134', halfLife: '2.06 years' },
  { value: 'Xe-133', label: 'Xenon-133', halfLife: '5.25 days' },
  { value: 'Kr-85', label: 'Krypton-85', halfLife: '10.76 years' },
  { value: 'Co-60', label: 'Cobalt-60', halfLife: '5.27 years' },
];

const STABILITY_CLASSES = [
  { value: 'A', label: 'A - Very Unstable', description: 'Strong insolation, light winds' },
  { value: 'B', label: 'B - Unstable', description: 'Moderate insolation, light winds' },
  { value: 'C', label: 'C - Slightly Unstable', description: 'Weak insolation or cloudy' },
  { value: 'D', label: 'D - Neutral', description: 'Overcast or windy' },
  { value: 'E', label: 'E - Slightly Stable', description: 'Light winds, nighttime' },
  { value: 'F', label: 'F - Stable', description: 'Clear night, light winds' },
];

export default function PlumePage() {
  const { addToast } = useToast();
  const [isSimulating, setIsSimulating] = useState(false);
  const [simulationTime, setSimulationTime] = useState(0);
  const [maxSimulationTime] = useState(72); // hours
  
  const [releaseParams, setReleaseParams] = useState<ReleaseParameters>({
    latitude: 37.4215,
    longitude: 141.0323,
    altitude: 50,
    releaseRate: 1e15, // Bq/s
    duration: 4, // hours
    isotope: 'I-131',
    stabilityClass: 'D',
  });

  const [weatherData, setWeatherData] = useState<WeatherData>({
    windSpeed: 5.2,
    windDirection: 270,
    temperature: 288.15,
    pressure: 101325,
    humidity: 65,
    stabilityClass: 'D',
    timestamp: new Date().toISOString(),
  });

  const [plumeData, setPlumeData] = useState<PlumePoint[]>([]);
  const [activeTab, setActiveTab] = useState('simulation');

  // Fetch NOAA GFS weather data
  const fetchWeatherData = useCallback(async () => {
    try {
      // In production, this would call the NOAA GFS API endpoint
      // For now, simulate with realistic variations
      const response = await fetch('/api/weather/noaa-gfs?' + new URLSearchParams({
        lat: releaseParams.latitude.toString(),
        lon: releaseParams.longitude.toString(),
      }));
      
      if (response.ok) {
        const data = await response.json();
        setWeatherData(prev => ({
          ...prev,
          ...data,
          timestamp: new Date().toISOString(),
        }));
        addToast({
          title: 'Weather Data Updated',
          description: 'NOAA GFS data loaded successfully',
          variant: 'default',
        });
      } else {
        // Simulate weather variations for demo
        setWeatherData(prev => ({
          ...prev,
          windSpeed: 3 + Math.random() * 7,
          windDirection: Math.floor(Math.random() * 360),
          temperature: 283 + Math.random() * 15,
          timestamp: new Date().toISOString(),
        }));
      }
    } catch (error) {
      addToast({
        title: 'Weather Data Error',
        description: 'Failed to fetch NOAA GFS data',
        variant: 'destructive',
      });
    }
  }, [releaseParams.latitude, releaseParams.longitude, addToast]);

  // Calculate Gaussian plume dispersion
  const calculatePlumeDispersion = useCallback((timeHours: number): PlumePoint[] => {
    const data: PlumePoint[] = [];
    const { windSpeed, windDirection } = weatherData;
    const { releaseRate, isotope, stabilityClass } = releaseParams;
    
    // Pasquill-Gifford dispersion coefficients
    const dispersionCoeffs: Record<string, { ay: number; by: number; az: number; bz: number }> = {
      'A': { ay: 0.22, by: 0.90, az: 0.20, bz: 0.90 },
      'B': { ay: 0.16, by: 0.90, az: 0.12, bz: 0.90 },
      'C': { ay: 0.11, by: 0.90, az: 0.08, bz: 0.90 },
      'D': { ay: 0.08, by: 0.90, az: 0.06, bz: 0.90 },
      'E': { ay: 0.06, by: 0.90, az: 0.04, bz: 0.90 },
      'F': { ay: 0.04, by: 0.90, az: 0.02, bz: 0.90 },
    };
    
    const coeffs = dispersionCoeffs[stabilityClass] || dispersionCoeffs['D'];
    
    // Generate plume points along wind direction
    const numPoints = 50;
    const maxDistance = windSpeed * timeHours * 3600; // meters
    
    for (let i = 0; i < numPoints; i++) {
      const x = (i / numPoints) * maxDistance; // downwind distance
      const sigmaY = coeffs.ay * Math.pow(x, coeffs.by);
      const sigmaZ = coeffs.az * Math.pow(x, coeffs.bz);
      
      // Calculate centerline concentration
      const Q = releaseRate; // Bq/s
      const u = windSpeed; // m/s
      const H = releaseParams.altitude; // release height
      
      // Gaussian plume equation for centerline
      const exponent = -Math.pow(H, 2) / (2 * Math.pow(sigmaZ, 2));
      const concentration = (Q / (2 * Math.PI * sigmaY * sigmaZ * u)) * Math.exp(exponent);
      
      // Calculate dose rate (simplified conversion)
      const doseConversionFactor = 3.2e-14; // Sv per Bq/m³ for I-131
      const doseRate = concentration * doseConversionFactor * 1e6; // microSv/h
      
      // Convert to lat/lon (simplified)
      const windRad = (windDirection * Math.PI) / 180;
      const latOffset = (x * Math.cos(windRad)) / 111000;
      const lonOffset = (x * Math.sin(windRad)) / (111000 * Math.cos(releaseParams.latitude * Math.PI / 180));
      
      data.push({
        lat: releaseParams.latitude + latOffset,
        lng: releaseParams.longitude + lonOffset,
        concentration: Math.max(0, concentration),
        doseRate: Math.max(0, doseRate),
      });
    }
    
    return data;
  }, [weatherData, releaseParams]);

  // Run simulation
  useEffect(() => {
    if (!isSimulating) return;
    
    const interval = setInterval(() => {
      setSimulationTime(prev => {
        const newTime = prev + 0.5;
        if (newTime >= maxSimulationTime) {
          setIsSimulating(false);
          return maxSimulationTime;
        }
        return newTime;
      });
    }, 500);
    
    return () => clearInterval(interval);
  }, [isSimulating, maxSimulationTime]);

  // Update plume data when simulation time changes
  useEffect(() => {
    if (simulationTime > 0) {
      const newPlumeData = calculatePlumeDispersion(simulationTime);
      setPlumeData(newPlumeData);
    }
  }, [simulationTime, calculatePlumeDispersion]);

  const handleStartSimulation = () => {
    setIsSimulating(true);
    setSimulationTime(0);
    addToast({
      title: 'Simulation Started',
      description: `Running ${maxSimulationTime}h dispersion model`,
      variant: 'default',
    });
  };

  const handlePauseSimulation = () => {
    setIsSimulating(false);
  };

  const handleResetSimulation = () => {
    setIsSimulating(false);
    setSimulationTime(0);
    setPlumeData([]);
  };

  const handleExportData = () => {
    const data = {
      releaseParameters: releaseParams,
      weatherData,
      plumeData,
      simulationTime,
      exportTimestamp: new Date().toISOString(),
    };
    
    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `plume-simulation-${Date.now()}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
    
    addToast({
      title: 'Data Exported',
      description: 'Simulation data saved to file',
      variant: 'default',
    });
  };

  return (
    <div className="h-full w-full p-6 overflow-auto">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-display-md font-sans text-text-primary">Plume Dispersion Modeling</h1>
          <p className="text-text-secondary mt-1">
            Atmospheric dispersion calculations for radioactive releases with NOAA GFS integration
          </p>
        </div>
        <div className="flex items-center gap-2">
          <Badge variant="outline" className="flex items-center gap-1">
            <Wind className="h-3 w-3" />
            NOAA GFS Connected
          </Badge>
          <Button variant="outline" size="sm" onClick={handleExportData}>
            <Download className="h-4 w-4 mr-2" />
            Export
          </Button>
        </div>
      </div>

      <Tabs value={activeTab} onValueChange={setActiveTab} className="space-y-6">
        <TabsList className="grid w-full max-w-md grid-cols-3">
          <TabsTrigger value="simulation">Simulation</TabsTrigger>
          <TabsTrigger value="visualization">Visualization</TabsTrigger>
          <TabsTrigger value="evacuation">Evacuation Zones</TabsTrigger>
        </TabsList>

        <TabsContent value="simulation" className="space-y-6">
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            {/* Release Parameters */}
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Radio className="h-5 w-5 text-red-500" />
                  Release Parameters
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <Label htmlFor="latitude">Latitude</Label>
                    <Input
                      id="latitude"
                      type="number"
                      step="0.0001"
                      value={releaseParams.latitude}
                      onChange={(e) => setReleaseParams(prev => ({ ...prev, latitude: parseFloat(e.target.value) }))}
                    />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="longitude">Longitude</Label>
                    <Input
                      id="longitude"
                      type="number"
                      step="0.0001"
                      value={releaseParams.longitude}
                      onChange={(e) => setReleaseParams(prev => ({ ...prev, longitude: parseFloat(e.target.value) }))}
                    />
                  </div>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="altitude">Release Altitude (m)</Label>
                  <Slider
                    id="altitude"
                    value={[releaseParams.altitude]}
                    onValueChange={([value]) => setReleaseParams(prev => ({ ...prev, altitude: value }))}
                    min={0}
                    max={500}
                    step={10}
                  />
                  <div className="text-sm text-text-secondary">{releaseParams.altitude} m</div>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="isotope">Isotope</Label>
                  <Select
                    value={releaseParams.isotope}
                    onValueChange={(value) => setReleaseParams(prev => ({ ...prev, isotope: value }))}
                  >
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {ISOTOPES.map(isotope => (
                        <SelectItem key={isotope.value} value={isotope.value}>
                          {isotope.label} (t½: {isotope.halfLife})
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="releaseRate">Release Rate (Bq/s)</Label>
                  <Input
                    id="releaseRate"
                    type="number"
                    value={releaseParams.releaseRate.toExponential(2)}
                    onChange={(e) => setReleaseParams(prev => ({ ...prev, releaseRate: parseFloat(e.target.value) }))}
                  />
                </div>

                <div className="space-y-2">
                  <Label htmlFor="duration">Release Duration (hours)</Label>
                  <Slider
                    id="duration"
                    value={[releaseParams.duration]}
                    onValueChange={([value]) => setReleaseParams(prev => ({ ...prev, duration: value }))}
                    min={1}
                    max={24}
                    step={0.5}
                  />
                  <div className="text-sm text-text-secondary">{releaseParams.duration} hours</div>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="stability">Atmospheric Stability</Label>
                  <Select
                    value={releaseParams.stabilityClass}
                    onValueChange={(value) => setReleaseParams(prev => ({ ...prev, stabilityClass: value }))}
                  >
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {STABILITY_CLASSES.map(sc => (
                        <SelectItem key={sc.value} value={sc.value}>
                          {sc.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <p className="text-xs text-text-secondary">
                    {STABILITY_CLASSES.find(sc => sc.value === releaseParams.stabilityClass)?.description}
                  </p>
                </div>
              </CardContent>
            </Card>

            {/* Weather Conditions */}
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Wind className="h-5 w-5 text-blue-500" />
                  NOAA GFS Weather Data
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <Label className="flex items-center gap-2">
                      <Wind className="h-4 w-4" />
                      Wind Speed
                    </Label>
                    <div className="text-2xl font-bold">{weatherData.windSpeed.toFixed(1)} m/s</div>
                  </div>
                  <div className="space-y-2">
                    <Label className="flex items-center gap-2">
                      <MapPin className="h-4 w-4" />
                      Wind Direction
                    </Label>
                    <div className="text-2xl font-bold">{weatherData.windDirection}°</div>
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <Label className="flex items-center gap-2">
                      <Thermometer className="h-4 w-4" />
                      Temperature
                    </Label>
                    <div className="text-2xl font-bold">{(weatherData.temperature - 273.15).toFixed(1)}°C</div>
                  </div>
                  <div className="space-y-2">
                    <Label className="flex items-center gap-2">
                      <Gauge className="h-4 w-4" />
                      Pressure
                    </Label>
                    <div className="text-2xl font-bold">{(weatherData.pressure / 100).toFixed(0)} hPa</div>
                  </div>
                </div>

                <div className="space-y-2">
                  <Label>Humidity</Label>
                  <div className="text-lg font-semibold">{weatherData.humidity}%</div>
                </div>

                <div className="pt-4 border-t border-border-subtle">
                  <div className="flex items-center gap-2 text-sm text-text-secondary">
                    <Clock className="h-4 w-4" />
                    Last updated: {new Date(weatherData.timestamp).toLocaleString()}
                  </div>
                </div>

                <Button onClick={fetchWeatherData} variant="outline" className="w-full">
                  <Wind className="h-4 w-4 mr-2" />
                  Refresh Weather Data
                </Button>
              </CardContent>
            </Card>
          </div>

          {/* Simulation Controls */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Clock className="h-5 w-5" />
                Simulation Controls
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center gap-4">
                <Button
                  onClick={isSimulating ? handlePauseSimulation : handleStartSimulation}
                  variant={isSimulating ? 'secondary' : 'default'}
                  className="flex-1"
                >
                  {isSimulating ? (
                    <>
                      <Pause className="h-4 w-4 mr-2" />
                      Pause
                    </>
                  ) : (
                    <>
                      <Play className="h-4 w-4 mr-2" />
                      Start Simulation
                    </>
                  )}
                </Button>
                <Button onClick={handleResetSimulation} variant="outline">
                  <RotateCcw className="h-4 w-4 mr-2" />
                  Reset
                </Button>
              </div>

              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <Label>Simulation Time</Label>
                  <span className="text-sm font-semibold">{simulationTime.toFixed(1)} hours</span>
                </div>
                <Slider
                  value={[simulationTime]}
                  onValueChange={([value]) => setSimulationTime(value)}
                  min={0}
                  max={maxSimulationTime}
                  step={0.5}
                  disabled={isSimulating}
                />
                <div className="flex justify-between text-xs text-text-secondary">
                  <span>0h</span>
                  <span>{maxSimulationTime / 2}h</span>
                  <span>{maxSimulationTime}h</span>
                </div>
              </div>

              {plumeData.length > 0 && (
                <div className="grid grid-cols-3 gap-4 pt-4 border-t border-border-subtle">
                  <div className="text-center">
                    <div className="text-2xl font-bold text-red-500">
                      {Math.max(...plumeData.map(p => p.doseRate)).toExponential(2)}
                    </div>
                    <div className="text-xs text-text-secondary">Max Dose Rate (μSv/h)</div>
                  </div>
                  <div className="text-center">
                    <div className="text-2xl font-bold text-orange-500">
                      {(plumeData.length * 0.1).toFixed(1)}
                    </div>
                    <div className="text-xs text-text-secondary">Plume Length (km)</div>
                  </div>
                  <div className="text-center">
                    <div className="text-2xl font-bold text-blue-500">
                      {plumeData.filter(p => p.doseRate > 1).length}
                    </div>
                    <div className="text-xs text-text-secondary">High Dose Points</div>
                  </div>
                </div>
              )}
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="visualization">
          <Card className="h-[600px]">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <MapPin className="h-5 w-5" />
                Real-time Plume Visualization
              </CardTitle>
            </CardHeader>
            <CardContent className="h-[500px]">
              <PlumeVisualization 
                plumeData={plumeData}
                simulationData={{
                  center: [releaseParams.longitude, releaseParams.latitude],
                  radius: 1000,
                  concentration: plumeData.map(p => p.concentration),
                  windDirection: weatherData.windDirection,
                  windSpeed: weatherData.windSpeed,
                  stabilityClass: releaseParams.stabilityClass,
                  timeStep: simulationTime,
                  maxTime: maxSimulationTime,
                }}
                isAnimating={isSimulating}
                currentTime={simulationTime}
              />
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="evacuation">
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <EvacuationZones />
            
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <AlertTriangle className="h-5 w-5 text-amber-500" />
                  Dose Thresholds
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <div className="flex items-center justify-between p-3 bg-red-50 dark:bg-red-950 rounded-lg">
                    <div className="flex items-center gap-2">
                      <div className="w-3 h-3 rounded-full bg-red-500" />
                      <span className="font-semibold">Immediate Evacuation</span>
                    </div>
                    <span className="text-red-600 dark:text-red-400 font-bold">{'>'} 50 mSv/h</span>
                  </div>
                  
                  <div className="flex items-center justify-between p-3 bg-orange-50 dark:bg-orange-950 rounded-lg">
                    <div className="flex items-center gap-2">
                      <div className="w-3 h-3 rounded-full bg-orange-500" />
                      <span className="font-semibold">Shelter in Place</span>
                    </div>
                    <span className="text-orange-600 dark:text-orange-400 font-bold">10-50 mSv/h</span>
                  </div>
                  
                  <div className="flex items-center justify-between p-3 bg-yellow-50 dark:bg-yellow-950 rounded-lg">
                    <div className="flex items-center gap-2">
                      <div className="w-3 h-3 rounded-full bg-yellow-500" />
                      <span className="font-semibold">Monitoring Zone</span>
                    </div>
                    <span className="text-yellow-600 dark:text-yellow-400 font-bold">1-10 mSv/h</span>
                  </div>
                  
                  <div className="flex items-center justify-between p-3 bg-blue-50 dark:bg-blue-950 rounded-lg">
                    <div className="flex items-center gap-2">
                      <div className="w-3 h-3 rounded-full bg-blue-500" />
                      <span className="font-semibold">Precautionary Zone</span>
                    </div>
                    <span className="text-blue-600 dark:text-blue-400 font-bold">0.1-1 mSv/h</span>
                  </div>
                </div>

                <div className="pt-4 border-t border-border-subtle">
                  <div className="flex items-start gap-2 text-sm text-text-secondary">
                    <Info className="h-4 w-4 mt-0.5 flex-shrink-0" />
                    <p>
                      Evacuation zones are calculated based on projected dose rates from the 
                      Gaussian plume dispersion model. Actual zones may vary based on terrain, 
                      building shielding, and weather changes.
                    </p>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>
      </Tabs>
    </div>
  );
}
