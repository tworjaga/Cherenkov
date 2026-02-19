import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { Search } from './search';

describe('Search', () => {
  it('renders search input with placeholder', () => {
    render(<Search placeholder="Search items..." />);
    expect(screen.getByPlaceholderText('Search items...')).toBeInTheDocument();
  });

  it('calls onChange when typing', async () => {
    const handleChange = vi.fn();
    const user = userEvent.setup();
    render(<Search onChange={handleChange} />);
    
    const input = screen.getByPlaceholderText('Search...');
    await user.type(input, 'test query');
    
    expect(handleChange).toHaveBeenCalledWith('test query');
  });

  it('calls onSearch when pressing Enter', async () => {
    const handleSearch = vi.fn();
    const user = userEvent.setup();
    render(<Search onSearch={handleSearch} />);
    
    const input = screen.getByPlaceholderText('Search...');
    await user.type(input, 'search term{enter}');
    
    expect(handleSearch).toHaveBeenCalledWith('search term');
  });

  it('displays clear button when has value', async () => {
    render(<Search value="test" />);
    
    const clearButton = screen.getByRole('button', { name: /clear/i });
    expect(clearButton).toBeInTheDocument();
  });

  it('clears value when clear button clicked', async () => {
    const handleChange = vi.fn();
    const user = userEvent.setup();
    render(<Search value="test" onChange={handleChange} />);
    
    const clearButton = screen.getByRole('button', { name: /clear/i });
    await user.click(clearButton);
    
    expect(handleChange).toHaveBeenCalledWith('');
  });

  it('applies custom className', () => {
    render(<Search className="custom-search" />);
    expect(screen.getByPlaceholderText('Search...').parentElement).toHaveClass('custom-search');
  });
});
