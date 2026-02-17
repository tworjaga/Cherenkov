import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Switch } from './toggle';

describe('Switch', () => {
  it('renders switch in off state by default', () => {
    render(<Switch />);
    const switchElement = screen.getByRole('switch');
    expect(switchElement).toBeInTheDocument();
    expect(switchElement).toHaveAttribute('data-state', 'unchecked');
  });

  it('renders switch in on state when checked', () => {
    render(<Switch checked={true} onCheckedChange={vi.fn()} />);
    const switchElement = screen.getByRole('switch');
    expect(switchElement).toHaveAttribute('data-state', 'checked');
  });

  it('calls onCheckedChange when clicked', () => {
    const handleCheckedChange = vi.fn();
    render(<Switch onCheckedChange={handleCheckedChange} />);
    const switchElement = screen.getByRole('switch');
    fireEvent.click(switchElement);
    expect(handleCheckedChange).toHaveBeenCalledWith(true);
  });

  it('can be disabled', () => {
    render(<Switch disabled />);
    const switchElement = screen.getByRole('switch');
    expect(switchElement).toBeDisabled();
  });

  it('applies custom className', () => {
    render(<Switch className="custom-switch" />);
    const switchElement = screen.getByRole('switch');
    expect(switchElement.className).toContain('custom-switch');
  });
});
