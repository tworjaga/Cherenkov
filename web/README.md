# Cherenkov Web Frontend

Professional-grade radiological intelligence dashboard built with Next.js 14, TypeScript, and Tailwind CSS.

## Architecture

| Layer | Technology |
|-------|-----------|
| Framework | Next.js 14 (App Router) |
| Language | TypeScript 5.3 (Strict Mode) |
| Styling | Tailwind CSS 3.4 |
| State | Zustand 4.4 |
| Animation | Framer Motion 10.16 |
| 3D Globe | deck.gl 8.9 + MapLibre |
| Charts | Recharts 2.10 |
| API | GraphQL + WebSocket |

## Project Structure

```
src/
├── app/              # Next.js app router
├── components/       # React components
│   └── layout/       # Header, Sidebar, Panels
├── stores/           # Zustand state management
├── hooks/            # Custom React hooks
├── lib/              # Utilities and API clients
│   ├── graphql/      # GraphQL schema and queries
│   └── utils/        # Formatters and calculations
├── styles/           # Theme and design tokens
└── types/            # TypeScript definitions
```

## Design System

### Colors
- Background: `#050508` (primary), `#0a0a10` (secondary)
- Accent: `#00d4ff` (Cherenkov blue)
- Alert levels: normal, low, medium, high, critical

### Typography
- Sans: Inter
- Mono: JetBrains Mono

## Development

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Run tests
npm test

# Type check
npm run type-check

# Build for production
npm run build
```

## Environment Variables

```env
NEXT_PUBLIC_API_URL=http://localhost:8080/graphql
NEXT_PUBLIC_WS_URL=ws://localhost:8080/ws
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| 1-5 | Switch views |
| G | Center on selected sensor |
| T | Toggle right panel |
| Space | Live mode |
| Arrows | Step time |
| F | Fullscreen |
| Esc | Deselect |

## Features Implemented

- [x] Header with DEFCON indicator, UTC clock, connection status
- [x] Sidebar with 6 view navigation
- [x] Zustand stores (app, globe, data)
- [x] GraphQL client with WebSocket subscriptions
- [x] Keyboard shortcuts
- [x] Design system (colors, typography, animations)
- [x] Unit tests for utilities and stores

## License

MIT
