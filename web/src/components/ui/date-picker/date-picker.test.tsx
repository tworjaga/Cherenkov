import { describe, it, expect, vi } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { DatePicker } from './date-picker';

describe('DatePicker', () => {
  it('renders date picker with placeholder', () => {
    render(<DatePicker />);
    expect(screen.getByText('Pick a date')).toBeInTheDocument();
  });

  it('opens calendar when trigger is clicked', async () => {
    const user = userEvent.setup();
    render(<DatePicker />);
    const trigger = screen.getByRole('button');
    await user.click(trigger);
    // Calendar dialog should be visible
    await waitFor(() => {
      expect(screen.getByText('Select Date')).toBeInTheDocument();
    });
  });

  it('displays selected date', () => {
    const date = new Date(2024, 0, 15);
    render(<DatePicker date={date} />);
    // date-fns 'PPP' format includes ordinal suffix (15th)
    expect(screen.getByText('January 15th, 2024')).toBeInTheDocument();
  });

  it('calls onSelect when date is selected', async () => {
    const handleSelect = vi.fn();
    const user = userEvent.setup();
    render(<DatePicker onSelect={handleSelect} />);
    
    const trigger = screen.getByRole('button');
    await user.click(trigger);
    
    // Wait for modal to open and select a date (15th)
    await waitFor(() => {
      const dateButton = screen.getByText('15');
      expect(dateButton).toBeInTheDocument();
    });
    
    const dateButton = screen.getByText('15');
    await user.click(dateButton);
    
    expect(handleSelect).toHaveBeenCalled();
  });

  it('applies custom className', () => {
    render(<DatePicker className="custom-class" />);
    const trigger = screen.getByRole('button');
    expect(trigger).toHaveClass('custom-class');
  });
});
