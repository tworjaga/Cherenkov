(()=>{var e={};e.id=931,e.ids=[931],e.modules={47849:e=>{"use strict";e.exports=require("next/dist/client/components/action-async-storage.external")},55403:e=>{"use strict";e.exports=require("next/dist/client/components/request-async-storage.external")},94749:e=>{"use strict";e.exports=require("next/dist/client/components/static-generation-async-storage.external")},20399:e=>{"use strict";e.exports=require("next/dist/compiled/next-server/app-page.runtime.prod.js")},25528:e=>{"use strict";e.exports=require("next/dist\\client\\components\\action-async-storage.external.js")},91877:e=>{"use strict";e.exports=require("next/dist\\client\\components\\request-async-storage.external.js")},25319:e=>{"use strict";e.exports=require("next/dist\\client\\components\\static-generation-async-storage.external.js")},13685:e=>{"use strict";e.exports=require("http")},95687:e=>{"use strict";e.exports=require("https")},85477:e=>{"use strict";e.exports=require("punycode")},12781:e=>{"use strict";e.exports=require("stream")},57310:e=>{"use strict";e.exports=require("url")},71267:e=>{"use strict";e.exports=require("worker_threads")},59796:e=>{"use strict";e.exports=require("zlib")},95777:(e,t,s)=>{"use strict";s.r(t),s.d(t,{GlobalError:()=>a.a,__next_app__:()=>p,originalPathname:()=>c,pages:()=>u,routeModule:()=>m,tree:()=>d});var r=s(50482),o=s(69108),n=s(62563),a=s.n(n),i=s(68300),l={};for(let e in i)0>["default","tree","pages","GlobalError","originalPathname","__next_app__","routeModule"].indexOf(e)&&(l[e]=()=>i[e]);s.d(t,l);let d=["",{children:["__PAGE__",{},{page:[()=>Promise.resolve().then(s.bind(s,51136)),"C:\\Users\\User\\Downloads\\pythonprojects\\cherenkov\\web\\src\\app\\page.tsx"]}]},{layout:[()=>Promise.resolve().then(s.bind(s,5018)),"C:\\Users\\User\\Downloads\\pythonprojects\\cherenkov\\web\\src\\app\\layout.tsx"],error:[()=>Promise.resolve().then(s.bind(s,24117)),"C:\\Users\\User\\Downloads\\pythonprojects\\cherenkov\\web\\src\\app\\error.tsx"],loading:[()=>Promise.resolve().then(s.bind(s,93301)),"C:\\Users\\User\\Downloads\\pythonprojects\\cherenkov\\web\\src\\app\\loading.tsx"],"not-found":[()=>Promise.resolve().then(s.bind(s,31341)),"C:\\Users\\User\\Downloads\\pythonprojects\\cherenkov\\web\\src\\app\\not-found.tsx"]}],u=["C:\\Users\\User\\Downloads\\pythonprojects\\cherenkov\\web\\src\\app\\page.tsx"],c="/page",p={require:s,loadChunk:()=>Promise.resolve()},m=new r.AppPageRouteModule({definition:{kind:o.x.APP_PAGE,page:"/page",pathname:"/",bundlePath:"",filename:"",appPaths:[]},userland:{loaderTree:d}})},94632:(e,t,s)=>{Promise.resolve().then(s.bind(s,99546))},99546:(e,t,s)=>{"use strict";s.r(t),s.d(t,{default:()=>l});var r=s(95344),o=s(40356),n=s(95144);s(17868),s(3729),s(9111);var a=s(46575);s(48118),s(35891);var i=s(65706);function l(){let{sensors:e,anomalies:t}=(0,n.a2)();return(0,a.a)(),r.jsx("div",{className:"w-full h-full",children:r.jsx(o.TH,{sensors:e,anomalies:t})})}i.Ps`
  query Sensors {
    sensors {
      id
      name
      latitude
      longitude
      status
      lastReading
    }
  }
`,i.Ps`
  query Sensor($id: ID!) {
    sensor(id: $id) {
      id
      name
      latitude
      longitude
      status
      lastReading
    }
  }
`,i.Ps`
  query Readings($sensorIds: [ID!]!, $from: Timestamp!, $to: Timestamp!, $aggregation: String) {
    readings(sensorIds: $sensorIds, from: $from, to: $to, aggregation: $aggregation) {
      id
      sensorId
      timestamp
      doseRate
      unit
    }
  }
`,i.Ps`
  query Anomalies($severity: [String!], $since: Timestamp!, $limit: Int) {
    anomalies(severity: $severity, since: $since, limit: $limit) {
      id
      sensorId
      severity
      zScore
      detectedAt
    }
  }
`,i.Ps`
  query Facilities {
    facilities {
      id
      name
      facilityType
      latitude
      longitude
      status
    }
  }
`,i.Ps`
  query GlobalStatus {
    globalStatus {
      defconLevel
      status
      activeAnomalies
      lastUpdated
    }
  }
`,i.Ps`
  mutation AcknowledgeAlert($alertId: ID!) {
    acknowledgeAlert(alertId: $alertId) {
      id
      acknowledged
      acknowledgedAt
    }
  }
`,i.Ps`
  mutation CreateAlertRule($rule: AlertRuleInput!) {
    createAlertRule(rule: $rule) {
      id
      name
      enabled
      createdAt
    }
  }
`},51136:(e,t,s)=>{"use strict";s.r(t),s.d(t,{$$typeof:()=>n,__esModule:()=>o,default:()=>a});let r=(0,s(86843).createProxy)(String.raw`C:\Users\User\Downloads\pythonprojects\cherenkov\web\src\app\page.tsx`),{__esModule:o,$$typeof:n}=r,a=r.default}};var t=require("../webpack-runtime.js");t.C(e);var s=e=>t(t.s=e),r=t.X(0,[638,240,227,827,356],()=>s(95777));module.exports=r})();