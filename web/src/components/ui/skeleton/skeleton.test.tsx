import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/react';
import { Skeleton } from './skeleton';


describe('Skeleton', () => {
  it('renders skeleton element', () => {
    render(<Skeleton />);
    const skeleton = document.querySelector('.animate-pulse');
    expect(skeleton).toBeInTheDocument();
  });

  it('applies custom className', () => {
    render(<Skeleton className="custom-skeleton" />);
    const skeleton = document.querySelector('.animate-pulse');
    expect(skeleton?.className).toContain('custom-skeleton');
  });

  it('renders with specified dimensions', () => {
    render(<Skeleton className="h-4 w-32" />);
    const skeleton = document.querySelector('.animate-pulse');
    expect(skeleton?.className).toContain('h-4');
    expect(skeleton?.className).toContain('w-32');
  });
});
