import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { AlertFilters } from './alert-filters';

const mockCounts = {
  all: 10,
  critical: 2,
  high: 3,
  medium: 4,
  low: 1,
};

describe('AlertFilters', () => {
  it('renders all filter options', () => {
    render(
      <AlertFilters
        activeFilter="all"
        onFilterChange={vi.fn()}
        counts={mockCounts}
      />
    );

    expect(screen.getByText('All')).toBeInTheDocument();
    expect(screen.getByText('Critical')).toBeInTheDocument();
    expect(screen.getByText('High')).toBeInTheDocument();
    expect(screen.getByText('Medium')).toBeInTheDocument();
    expect(screen.getByText('Low')).toBeInTheDocument();
  });

  it('calls onFilterChange when filter is clicked', () => {
    const onFilterChange = vi.fn();
    render(
      <AlertFilters
        activeFilter="all"
        onFilterChange={onFilterChange}
        counts={mockCounts}
      />
    );

    fireEvent.click(screen.getByText('Critical'));
    expect(onFilterChange).toHaveBeenCalledWith('critical');
  });

  it('shows active filter with different styling', () => {
    render(
      <AlertFilters
        activeFilter="critical"
        onFilterChange={vi.fn()}
        counts={mockCounts}
      />
    );

    const criticalButton = screen.getByText('Critical').closest('button');
    expect(criticalButton).toHaveClass('bg-[#1a1a25]');
  });

  it('displays count badges for filters with alerts', () => {
    render(
      <AlertFilters
        activeFilter="all"
        onFilterChange={vi.fn()}
        counts={mockCounts}
      />
    );

    expect(screen.getByText('10')).toBeInTheDocument();
    expect(screen.getByText('2')).toBeInTheDocument();
    expect(screen.getByText('3')).toBeInTheDocument();
  });
});
