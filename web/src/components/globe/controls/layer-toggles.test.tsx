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

    const checkboxes = screen.getAllByRole('checkbox');
    expect(checkboxes[0]).toBeChecked(); // sensors: true
    expect(checkboxes[1]).not.toBeChecked(); // facilities: false
    expect(checkboxes[2]).toBeChecked(); // anomalies: true
    expect(checkboxes[3]).not.toBeChecked(); // plumes: false
    expect(checkboxes[4]).not.toBeChecked(); // heatmap: false
  });
});
