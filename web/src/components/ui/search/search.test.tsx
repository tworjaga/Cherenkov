import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Search } from './search';

describe('Search', () => {
  it('renders search input with placeholder', () => {
    render(<Search placeholder="Search..." />);
    expect(screen.getByPlaceholderText('Search...')).toBeInTheDocument();
  });

  it('calls onChange when input changes', () => {
    const handleChange = vi.fn();
    render(<Search onChange={handleChange} />);
    
    const input = screen.getByPlaceholderText('Search...');
    fireEvent.change(input, { target: { value: 'test query' } });
    
    expect(handleChange).toHaveBeenCalledWith('test query');
  });



  it('displays initial value', () => {
    render(<Search value="initial" />);
    expect(screen.getByDisplayValue('initial')).toBeInTheDocument();
  });

  it('clears input when clear button is clicked', () => {
    const handleChange = vi.fn();
    render(<Search value="query" onChange={handleChange} />);
    
    const clearButton = screen.getByLabelText('Clear search');
    fireEvent.click(clearButton);
    
    expect(handleChange).toHaveBeenCalledWith('');
  });


  it('applies custom className', () => {

    render(<Search className="custom-class" />);
    const container = document.querySelector('.custom-class');
    expect(container).toBeInTheDocument();
  });
});
