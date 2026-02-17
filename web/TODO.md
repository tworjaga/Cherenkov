# Cherenkov Frontend Implementation TODO

## Phase 1: Project Initialization - COMPLETE
- [x] Initialize Next.js 14 project with TypeScript
- [x] Install all dependencies (React 18, Tailwind, Framer Motion, Zustand, deck.gl, etc.)
- [x] Configure TypeScript strict mode
- [x] Set up Tailwind with custom design tokens
- [x] Configure ESLint and Prettier

## Phase 2: Design System Foundation - COMPLETE
- [x] Create color system constants (exact values from spec)
- [x] Create typography system (Inter, JetBrains Mono)
- [x] Create spacing system (4pt grid)
- [x] Create animation presets (Framer Motion)
- [x] Set up global CSS variables

## Phase 3: GraphQL Infrastructure - COMPLETE
- [x] Set up GraphQL client with graphql-request
- [x] Create schema file matching backend API
- [x] Create queries, mutations, subscriptions
- [x] Set up WebSocket client structure

## Phase 4: State Management (Zustand) - COMPLETE
- [x] Create AppStore (view state, UI state, time control, connection)
- [x] Create GlobeStore (viewport, layers, hovered features)
- [x] Create DataStore (alerts, readings, global status)

## Phase 5: Core Layout Components - COMPLETE
- [x] Header component (56px, DEFCON indicator, UTC clock, connection status)
- [x] Sidebar component (64px icon-only, navigation)
- [x] RightPanel component (320px, alert feed, sensor detail)
- [x] BottomPanel component (200px, charts, stats, events)

## Phase 6: 3D Globe Implementation - COMPLETE
- [x] Set up deck.gl with MapLibre base map
- [x] Create globe container component
- [x] Implement sensor layer (ScatterplotLayer)
- [x] Implement anomaly layer (markers)
- [x] Implement heatmap layer (HeatmapLayer)
- [x] Add layer toggle controls
- [x] Add zoom and viewport controls

## Phase 7: Dashboard Components - COMPLETE
- [x] DefconIndicator component (levels with animations)
- [x] AlertCard component (severity colors)
- [x] AlertFeed component (scrolling list)
- [x] SensorDetail component (current reading, metadata)
- [x] ReadingChart component (Recharts area chart)
- [x] RegionalStats component (horizontal bar chart)
- [x] Event feed component

## Phase 8: Real-time Features - COMPLETE
- [x] WebSocket hook structure
- [x] useWebSocket hook for subscriptions
- [x] Connection status management

## Phase 9: Interactions & Animations - COMPLETE
- [x] Keyboard shortcuts hook (1-5 views, G, T, Space, arrows)
- [x] Framer Motion animations structure
- [x] Hover states and transitions

## Phase 10: Testing - COMPLETE
- [x] Vitest configuration
- [x] Test setup with jsdom
- [x] Unit tests for formatters
- [x] Unit tests for calculations
- [x] Unit tests for app store

## P0 Implementation Summary

All Priority 0 components have been implemented:

1. **3D WebGL Globe** (deck.gl 8.9.0)
   - ScatterplotLayer for sensors
   - HeatmapLayer for aggregate data
   - Anomaly markers with severity colors
   - Layer toggle controls
   - Viewport controls

2. **Real-time Alert Feed**
   - Alert cards with severity-based colors
   - Acknowledge functionality
   - Auto-scrolling with pause on hover
   - Framer Motion animations

3. **Sensor Detail Panel**
   - Current dose rate display
   - Location metadata
   - Status indicators
   - Selection integration with globe

4. **Analytics Charts** (recharts 2.10.0)
   - AreaChart for global radiation time-series
   - BarChart for regional statistics
   - Tabbed interface (chart/regions/events)
   - Responsive container

5. **Layout System**
   - Header with DEFCON indicator
   - Sidebar navigation
   - Right panel (alerts + sensor detail)
   - Bottom panel (analytics)

6. **State Management**
   - Zustand stores (app, globe, data)
   - TypeScript strict mode compliance
   - Persistent state where needed

7. **Type Declarations**
   - deckgl.d.ts for WebGL components
   - Full TypeScript coverage

## Next Steps (P1/P2)
- [ ] GraphQL Code Generator setup
- [ ] WebSocket subscription implementation
- [ ] Plume simulation layer
- [ ] Facility markers layer
- [ ] Time slider component
- [ ] Search functionality
- [ ] Settings panels
- [ ] E2E tests with Playwright
- [ ] Storybook documentation
- [ ] Production Docker build
