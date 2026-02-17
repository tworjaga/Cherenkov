import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { DatePicker } from './date-picker';

describe('DatePicker', () => {
  it('renders date picker with placeholder', () => {
    render(<DatePicker />);
    expect(screen.getByPlaceholderText('Pick a date')).toBeInTheDocument();
  });

  it('opens calendar when trigger is clicked', () => {
    render(<DatePicker />);
    const trigger = screen.getByRole('button');
    fireEvent.click(trigger);
    // Calendar should be visible
    expect(document.querySelector('[role="dialog"]')).toBeInTheDocument();
  });

  it('displays selected date', () => {
    const date = new Date(2024, 0, 15);
    render(<DatePicker date={date} />);
    expect(screen.getByDisplayValue('January 15, 2024')).toBeInTheDocument();
  });

  it('calls onSelect when date is selected', () => {
    const handleSelect = vi.fn();
    render(<DatePicker onSelect={handleSelect} />);
    
    const trigger = screen.getByRole('button');
    fireEvent.click(trigger);
    
    // Select a date
    const dateButton = screen.getByText('15');
    fireEvent.click(dateButton);
    
    expect(handleSelect).toHaveBeenCalled();
  });

  it('applies custom className', () => {

    render(<DatePicker className="custom-class" />);
    const trigger = screen.getByRole('button');
    expect(trigger).toHaveClass('custom-class');
  });
});
