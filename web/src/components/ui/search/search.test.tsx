import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Search } from './search';

describe('Search', () => {
  it('renders search input with placeholder', () => {
    render(<Search placeholder="Search..." />);
    expect(screen.getByPlaceholderText('Search...')).toBeInTheDocument();
  });

  it('calls onSearch when input changes', () => {
    const handleSearch = vi.fn();
    render(<Search onSearch={handleSearch} />);
    
    const input = screen.getByRole('searchbox');
    fireEvent.change(input, { target: { value: 'test query' } });
    
    expect(handleSearch).toHaveBeenCalledWith('test query');
  });

  it('displays initial value', () => {
    render(<Search value="initial" />);
    expect(screen.getByDisplayValue('initial')).toBeInTheDocument();
  });

  it('clears input when clear button is clicked', () => {
    const handleSearch = vi.fn();
    render(<Search value="query" onSearch={handleSearch} />);
    
    const clearButton = screen.getByLabelText('Clear search');
    fireEvent.click(clearButton);
    
    expect(handleSearch).toHaveBeenCalledWith('');
  });

  it('applies custom className', () => {

    render(<Search className="custom-class" />);
    const container = document.querySelector('.custom-class');
    expect(container).toBeInTheDocument();
  });
});
