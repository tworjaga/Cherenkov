import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { LayerToggles } from './layer-toggles';

// Mock the globe store
vi.mock('@/stores', () => ({
  useGlobeStore: () => ({
    layers: {
      sensors: true,
      facilities: false,
      anomalies: true,
      plumes: false,
      heatmap: false,
    },
    toggleLayer: vi.fn(),
  }),
}));

describe('LayerToggles', () => {
  it('renders all layer toggles', () => {
    render(<LayerToggles />);

    expect(screen.getByText('Sensors')).toBeInTheDocument();
    expect(screen.getByText('Facilities')).toBeInTheDocument();
    expect(screen.getByText('Anomalies')).toBeInTheDocument();
    expect(screen.getByText('Plumes')).toBeInTheDocument();
    expect(screen.getByText('Heatmap')).toBeInTheDocument();
  });

  it('renders with custom className', () => {
    const { container } = render(<LayerToggles className="custom-class" />);
    const firstElement = container.firstChild as Element | null;
    expect(firstElement?.className).toContain('custom-class');
  });


  it('reflects correct toggle states from store', () => {
    render(<LayerToggles />);

    const switches = screen.getAllByRole('switch');
    expect(switches[0]).toHaveAttribute('aria-checked', 'true'); // sensors: true
    expect(switches[1]).toHaveAttribute('aria-checked', 'false'); // facilities: false
    expect(switches[2]).toHaveAttribute('aria-checked', 'true'); // anomalies: true
    expect(switches[3]).toHaveAttribute('aria-checked', 'false'); // plumes: false
    expect(switches[4]).toHaveAttribute('aria-checked', 'false'); // heatmap: false
  });





});
