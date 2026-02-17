import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Progress } from './progress';

describe('Progress', () => {
  it('renders progress bar', () => {
    render(<Progress value={50} />);
    expect(screen.getByRole('progressbar')).toBeInTheDocument();
  });

  it('displays correct value', () => {
    render(<Progress value={75} />);
    const progress = screen.getByRole('progressbar');
    expect(progress).toHaveAttribute('aria-valuenow', '75');
  });

  it('displays indeterminate state', () => {
    render(<Progress value={0} indeterminate />);
    const progress = screen.getByRole('progressbar');
    expect(progress).toHaveAttribute('data-state', 'indeterminate');
  });

  it('applies custom className', () => {
    render(<Progress value={50} className="custom-class" />);
    const progress = screen.getByRole('progressbar');
    expect(progress).toHaveClass('custom-class');
  });

  it('has correct max value', () => {
    render(<Progress value={50} max={100} />);
    const progress = screen.getByRole('progressbar');
    expect(progress).toHaveAttribute('aria-valuemax', '100');
  });
});
