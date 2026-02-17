import { describe, it, expect, vi, beforeEach } from 'vitest';

import { render, screen, fireEvent } from '@testing-library/react';
import { ZoomControls } from './zoom-controls';

// Mock the GlobeStore
const mockSetViewport = vi.fn();

vi.mock('@/stores', () => ({
  useGlobeStore: () => ({
    viewport: {
      latitude: 20,
      longitude: 0,
      zoom: 2,
      pitch: 0,
      bearing: 0,
    },
    setViewport: mockSetViewport,
  }),
}));

describe('ZoomControls', () => {
  beforeEach(() => {
    mockSetViewport.mockClear();
  });

  it('renders zoom in button', () => {
    render(<ZoomControls />);
    expect(screen.getByRole('button', { name: /zoom in/i })).toBeInTheDocument();
  });

  it('renders zoom out button', () => {
    render(<ZoomControls />);
    expect(screen.getByRole('button', { name: /zoom out/i })).toBeInTheDocument();
  });

  it('renders reset button', () => {
    render(<ZoomControls />);
    expect(screen.getByRole('button', { name: /reset view/i })).toBeInTheDocument();
  });

  it('calls setViewport with increased zoom when zoom in clicked', () => {
    render(<ZoomControls />);
    fireEvent.click(screen.getByRole('button', { name: /zoom in/i }));
    expect(mockSetViewport).toHaveBeenCalledWith({ zoom: 3 });
  });

  it('calls setViewport with decreased zoom when zoom out clicked', () => {
    render(<ZoomControls />);
    fireEvent.click(screen.getByRole('button', { name: /zoom out/i }));
    expect(mockSetViewport).toHaveBeenCalledWith({ zoom: 1 });
  });

  it('calls setViewport with reset values when reset clicked', () => {
    render(<ZoomControls />);
    fireEvent.click(screen.getByRole('button', { name: /reset view/i }));
    expect(mockSetViewport).toHaveBeenCalledWith({
      latitude: 20,
      longitude: 0,
      zoom: 2,
      pitch: 0,
      bearing: 0,
    });
  });
});
