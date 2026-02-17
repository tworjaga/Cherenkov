declare module 'deck.gl' {
  export * from '@deck.gl/core';
  export * from '@deck.gl/layers';
  export * from '@deck.gl/aggregation-layers';
  export * from '@deck.gl/geo-layers';
  export * from '@deck.gl/react';
}

declare module '@deck.gl/react' {

  import * as React from 'react';
  
  export interface MapViewState {
    longitude: number;
    latitude: number;
    zoom: number;
    pitch?: number;
    bearing?: number;
    minZoom?: number;
    maxZoom?: number;
  }
  
  export interface DeckProps {
    viewState?: MapViewState;
    onViewStateChange?: (params: { viewState: MapViewState }) => void;
    controller?: boolean | Record<string, unknown>;
    layers?: unknown[];
    getTooltip?: (info: { object?: unknown; x?: number; y?: number }) => string | { text?: string; html?: string } | null;
    style?: React.CSSProperties;
    children?: React.ReactNode;
  }
  
  export const DeckGL: React.FC<DeckProps>;
}


declare module '@deck.gl/layers' {
  export interface LayerProps<D = unknown> {
    id?: string;
    data?: D[];
    visible?: boolean;
    pickable?: boolean;
    opacity?: number;
    onClick?: (info: { object?: D; x: number; y: number }) => void;
    onHover?: (info: { object?: D; x: number; y: number }) => void;
    getPosition?: (d: D) => [number, number, number?];
    getFillColor?: (d: D) => [number, number, number, number?] | [number, number, number] | (() => [number, number, number, number?] | [number, number, number]) | number[];
    getLineColor?: (d: D) => [number, number, number, number?] | [number, number, number] | (() => [number, number, number, number?] | [number, number, number]) | number[];

    getLineWidth?: number | ((d: D) => number);
    getRadius?: number | ((d: D) => number);

    radiusMinPixels?: number;
    radiusMaxPixels?: number;
    lineWidthMinPixels?: number;
    lineWidthMaxPixels?: number;
    stroked?: boolean;
    filled?: boolean;
    billboard?: boolean;
    sizeScale?: number;
    sizeUnits?: string;
    getSize?: number | ((d: D) => number);
    getIcon?: string | ((d: D) => string);
    getColor?: (d: D) => [number, number, number, number?] | [number, number, number];
    iconAtlas?: string;
    iconMapping?: Record<string, unknown>;
    updateTriggers?: Record<string, unknown[]>;
    transitions?: Record<string, number | { duration?: number; easing?: (t: number) => number }>;
  }

  
  export class ScatterplotLayer<D = unknown> {
    constructor(props: LayerProps<D>);
  }
  
  export class IconLayer<D = unknown> {
    constructor(props: LayerProps<D>);
  }
  
  export class LineLayer<D = unknown> {
    constructor(props: LayerProps<D>);
  }
  
  export class PolygonLayer<D = unknown> {
    constructor(props: LayerProps<D>);
  }
}

declare module '@deck.gl/aggregation-layers' {
  export interface HeatmapLayerProps<D = unknown> {
    id?: string;
    data?: D[];
    visible?: boolean;
    pickable?: boolean;
    opacity?: number;
    getPosition?: (d: D) => [number, number];
    getWeight?: number | ((d: D) => number);
    radiusPixels?: number;
    intensity?: number;
    threshold?: number;
    colorRange?: [number, number, number][];
    onClick?: (info: { object?: D; x: number; y: number }) => void;
    onHover?: (info: { object?: D; x: number; y: number }) => void;
  }
  
  export class HeatmapLayer<D = unknown> {
    constructor(props: HeatmapLayerProps<D>);
  }
}

declare module '@deck.gl/geo-layers' {
  export interface GeoJsonLayerProps<D = unknown> {
    id?: string;
    data?: string | D[] | Record<string, unknown>;
    visible?: boolean;
    pickable?: boolean;
    opacity?: number;
    filled?: boolean;
    stroked?: boolean;
    getFillColor?: (d: D) => [number, number, number, number?] | [number, number, number];
    getLineColor?: (d: D) => [number, number, number, number?] | [number, number, number];
    getLineWidth?: number | ((d: D) => number);
    lineWidthMinPixels?: number;
    lineWidthMaxPixels?: number;
    pointRadiusMinPixels?: number;
    pointRadiusMaxPixels?: number;
    getRadius?: number | ((d: D) => number);
    onClick?: (info: { object?: D; x: number; y: number }) => void;
    onHover?: (info: { object?: D; x: number; y: number }) => void;
  }
  
  export class GeoJsonLayer<D = unknown> {
    constructor(props: GeoJsonLayerProps<D>);
  }
}

declare module '@deck.gl/core' {
  export interface MapViewState {
    longitude: number;
    latitude: number;
    zoom: number;
    pitch?: number;
    bearing?: number;
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
    getTooltip?: (info: { object?: unknown; x?: number; y?: number }) => string | { text?: string; html?: string } | null;
    style?: React.CSSProperties;
    children?: React.ReactNode;
  }
  
  export class Layer<D = unknown> {
    constructor(props: Record<string, unknown>);
    id: string;
    props: Record<string, unknown>;
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

declare module '@apollo/client' {
  export interface DocumentNode {}
  export function gql(literals: TemplateStringsArray, ...placeholders: unknown[]): DocumentNode;
  export function useQuery<TData = unknown, TVariables = Record<string, unknown>>(query: DocumentNode, options?: { variables?: TVariables; skip?: boolean }): { data?: TData; loading: boolean; error?: Error };
  export function useMutation<TData = unknown, TVariables = Record<string, unknown>>(mutation: DocumentNode, options?: { variables?: TVariables }): [(options?: { variables?: TVariables }) => Promise<{ data?: TData }>, { loading: boolean; error?: Error }];
  export function useSubscription<TData = unknown, TVariables = Record<string, unknown>>(subscription: DocumentNode, options?: { variables?: TVariables }): { data?: TData; loading: boolean; error?: Error };
  export class ApolloClient<TCacheShape = unknown> {
    constructor(options: { uri: string; cache: unknown });
  }
  export function InMemoryCache(): unknown;
}
