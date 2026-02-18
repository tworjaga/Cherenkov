import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Switch } from './switch';

describe('Switch', () => {
  it('renders switch input', () => {
    render(<Switch />);
    expect(screen.getByRole('switch')).toBeInTheDocument();
  });

  it('calls onCheckedChange when clicked', () => {
    const handleChange = vi.fn();
    render(<Switch onCheckedChange={handleChange} />);
    
    const switchEl = screen.getByRole('switch');
    fireEvent.click(switchEl);
    
    expect(handleChange).toHaveBeenCalled();
  });



  it('displays checked state correctly', () => {
    render(<Switch checked />);
    
    const switchInput = screen.getByRole('switch');
    const switchContainer = switchInput.parentElement;
    expect(switchContainer).toHaveAttribute('data-state', 'checked');
  });




  it('is disabled when disabled prop is true', () => {
    render(<Switch disabled />);
    expect(screen.getByRole('switch')).toBeDisabled();
  });

  it('applies custom className', () => {
    render(<Switch className="custom-class" />);
    const switchInput = screen.getByRole('switch');
    const switchLabel = switchInput.closest('label');
    expect(switchLabel).toHaveClass('custom-class');
  });




  it('renders with switchSize small', () => {
    render(<Switch switchSize="sm" />);
    const switchInput = screen.getByRole('switch');
    const switchContainer = switchInput.parentElement;
    const tracks = switchContainer?.querySelectorAll('.rounded-full');
    const track = tracks?.[0];
    expect(track).toHaveClass('w-8');
  });

  it('renders with switchSize large', () => {
    render(<Switch switchSize="lg" />);
    const switchInput = screen.getByRole('switch');
    const switchContainer = switchInput.parentElement;
    const tracks = switchContainer?.querySelectorAll('.rounded-full');
    const track = tracks?.[0];
    expect(track).toHaveClass('w-14');
  });



});
