import { describe, it, expect, vi } from 'vitest';
import { render } from '@testing-library/react';
import { FacilityLayer } from './facility-layer';

// Mock the DataStore
vi.mock('@/stores', () => ({
  useDataStore: () => ({
    facilities: [
      {
        id: 'facility-1',
        name: 'Test Reactor',
        type: 'nuclear',
        location: { lat: 35.6762, lon: 139.6503 },
        status: 'operating',
        reactorType: 'PWR',
        capacity: 1000,
      },
    ],
  }),
}));

describe('FacilityLayer', () => {
  const defaultProps = {
    onFacilityClick: vi.fn(),
    selectedFacilityId: null,
  };

  it('renders without crashing', () => {
    const { container } = render(<FacilityLayer {...defaultProps} />);
    expect(container).toBeInTheDocument();
  });

  it('renders with selected facility', () => {
    const { container } = render(
      <FacilityLayer {...defaultProps} selectedFacilityId="facility-1" />
    );
    expect(container).toBeInTheDocument();
  });

  it('renders without click handler', () => {
    const { container } = render(<FacilityLayer />);
    expect(container).toBeInTheDocument();
  });
});
