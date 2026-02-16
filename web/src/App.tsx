import { Route } from '@solidjs/router';

import Layout from './components/Layout';
import Dashboard from './pages/Dashboard';
import GlobeView from './pages/GlobeView';
import Sensors from './pages/Sensors';
import Anomalies from './pages/Anomalies';
import PlumeSimulator from './pages/PlumeSimulator';
import Settings from './pages/Settings';

function App() {
  return (
    <Route path="/" component={Layout}>
      <Route path="/" component={Dashboard} />
      <Route path="/globe" component={GlobeView} />
      <Route path="/sensors" component={Sensors} />
      <Route path="/anomalies" component={Anomalies} />
      <Route path="/plume" component={PlumeSimulator} />
      <Route path="/settings" component={Settings} />
    </Route>
  );
}


export default App;
