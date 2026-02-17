import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Toggle } from './toggle';

describe('Toggle', () => {
  it('renders with children', () => {
    render(<Toggle>Toggle Button</Toggle>);
    expect(screen.getByText('Toggle Button')).toBeInTheDocument();
  });

  it('toggles pressed state on click', () => {
    render(<Toggle>Toggle</Toggle>);
    const button = screen.getByRole('button');
    
    expect(button).toHaveAttribute('aria-pressed', 'false');
    fireEvent.click(button);
    expect(button).toHaveAttribute('aria-pressed', 'true');
  });

  it('calls onPressedChange when toggled', () => {
    const onPressedChange = vi.fn();
    render(<Toggle onPressedChange={onPressedChange}>Toggle</Toggle>);
    
    fireEvent.click(screen.getByRole('button'));
    expect(onPressedChange).toHaveBeenCalledWith(true);
    
    fireEvent.click(screen.getByRole('button'));
    expect(onPressedChange).toHaveBeenCalledWith(false);
  });

  it('respects controlled pressed prop', () => {
    const { rerender } = render(<Toggle pressed={true}>Toggle</Toggle>);
    expect(screen.getByRole('button')).toHaveAttribute('aria-pressed', 'true');
    
    rerender(<Toggle pressed={false}>Toggle</Toggle>);
    expect(screen.getByRole('button')).toHaveAttribute('aria-pressed', 'false');
  });

  it('does not toggle when disabled', () => {
    const onPressedChange = vi.fn();
    render(<Toggle disabled onPressedChange={onPressedChange}>Toggle</Toggle>);
    
    fireEvent.click(screen.getByRole('button'));
    expect(onPressedChange).not.toHaveBeenCalled();
  });

  it('applies data-state attribute', () => {
    render(<Toggle pressed>Toggle</Toggle>);
    expect(screen.getByRole('button')).toHaveAttribute('data-state', 'on');
  });
});
