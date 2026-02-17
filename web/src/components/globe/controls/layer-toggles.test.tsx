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
    expect(container.firstChild?.className).toContain('custom-class');
  });

  it('reflects correct toggle states from store', () => {
    render(<LayerToggles />);

    const toggles = screen.getAllByRole('switch');
    expect(toggles[0]).toHaveAttribute('data-state', 'checked'); // sensors
    expect(toggles[1]).toHaveAttribute('data-state', 'unchecked'); // facilities
    expect(toggles[2]).toHaveAttribute('data-state', 'checked'); // anomalies
  });
});
