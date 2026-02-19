import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Radio } from './radio';

describe('Radio', () => {
  it('renders radio input', () => {
    render(<Radio value="option1" />);
    expect(screen.getByRole('radio')).toBeInTheDocument();
  });

  it('renders with label', () => {
    render(<Radio value="option1" label="Option 1" />);
    expect(screen.getByText('Option 1')).toBeInTheDocument();
  });

  it('calls onChange when clicked', () => {
    const handleChange = vi.fn();
    render(<Radio value="option1" onChange={handleChange} />);
    
    const radio = screen.getByRole('radio');
    fireEvent.click(radio);
    
    expect(handleChange).toHaveBeenCalled();
  });

  it('displays checked state correctly', () => {
    render(<Radio value="option1" checked readOnly />);
    
    const radio = screen.getByRole('radio');
    expect(radio).toBeChecked();
  });

  it('is disabled when disabled prop is true', () => {
    render(<Radio value="option1" disabled />);
    expect(screen.getByRole('radio')).toBeDisabled();
  });

  it('applies custom className', () => {
    render(<Radio value="option1" className="custom-class" />);
    const radio = screen.getByRole('radio');
    expect(radio).toHaveClass('sr-only', 'custom-class');
  });

});
