declare module '@deck.gl/react' {
  import * as React from 'react';
  import { DeckProps } from '@deck.gl/core';
  
  export const DeckGL: React.FC<DeckProps>;
}

declare module '@deck.gl/layers' {
  export class ScatterplotLayer<D = unknown> {
    constructor(props: Record<string, unknown>);
  }
  
  export class IconLayer<D = unknown> {
    constructor(props: Record<string, unknown>);
  }
}

declare module '@deck.gl/aggregation-layers' {
  export class HeatmapLayer<D = unknown> {
    constructor(props: Record<string, unknown>);
  }
}

declare module '@deck.gl/core' {
  export interface MapViewState {
    longitude: number;
    latitude: number;
    zoom: number;
    pitch: number;
    bearing: number;
    minZoom?: number;
    maxZoom?: number;
    minPitch?: number;
    maxPitch?: number;
    transitionDuration?: number;
    transitionEasing?: (t: number) => number;
    transitionInterpolator?: unknown;
    transitionInterruption?: number;
  }
  
  export interface DeckProps {
    viewState?: MapViewState;
    onViewStateChange?: (params: { viewState: MapViewState }) => void;
    controller?: boolean | Record<string, unknown>;
    layers?: unknown[];
    getTooltip?: (info: { object?: unknown }) => { text?: string } | null;
    style?: React.CSSProperties;
  }
}

declare module 'recharts' {
  import * as React from 'react';
  
  export interface AreaChartProps {
    data?: unknown[];
    children?: React.ReactNode;
  }
  
  export interface BarChartProps {
    data?: unknown[];
    layout?: 'horizontal' | 'vertical';
    children?: React.ReactNode;
    margin?: { top?: number; right?: number; left?: number; bottom?: number };
  }
  
  export interface AreaProps {
    type?: string;
    dataKey?: string;
    stroke?: string;
    strokeWidth?: number;
    fillOpacity?: number;
    fill?: string;
  }
  
  export interface BarProps {
    dataKey?: string;
    radius?: number | [number, number, number, number];
    children?: React.ReactNode;
  }
  
  export interface XAxisProps {
    dataKey?: string;
    type?: 'number' | 'category';
    tickFormatter?: (value: number) => string;
    stroke?: string;
    tick?: { fill?: string; fontSize?: number };
    hide?: boolean;
    tickLine?: boolean;
    axisLine?: boolean;
  }
  
  export interface YAxisProps {
    dataKey?: string;
    type?: 'number' | 'category';
    tickFormatter?: (value: number) => string;
    stroke?: string;
    tick?: { fill?: string; fontSize?: number };
    width?: number;
    hide?: boolean;
    tickLine?: boolean;
    axisLine?: boolean;
  }
  
  export interface CartesianGridProps {
    strokeDasharray?: string;
    stroke?: string;
    horizontal?: boolean;
    vertical?: boolean;
  }
  
  export interface TooltipProps {
    contentStyle?: React.CSSProperties;
    labelStyle?: React.CSSProperties;
    itemStyle?: React.CSSProperties;
    formatter?: (value: number, name: string) => [string, string];
    labelFormatter?: (label: number) => string;
    cursor?: { fill?: string };
  }
  
  export interface ResponsiveContainerProps {
    width?: string | number;
    height?: string | number;
    children?: React.ReactNode;
  }
  
  export interface CellProps {
    fill?: string;
  }
  
  export const AreaChart: React.FC<AreaChartProps>;
  export const BarChart: React.FC<BarChartProps>;
  export const Area: React.FC<AreaProps>;
  export const Bar: React.FC<BarProps>;
  export const XAxis: React.FC<XAxisProps>;
  export const YAxis: React.FC<YAxisProps>;
  export const CartesianGrid: React.FC<CartesianGridProps>;
  export const Tooltip: React.FC<TooltipProps>;
  export const ResponsiveContainer: React.FC<ResponsiveContainerProps>;
  export const Cell: React.FC<CellProps>;
}
