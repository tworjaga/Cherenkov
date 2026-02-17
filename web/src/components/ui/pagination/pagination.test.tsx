import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Pagination } from './pagination';

describe('Pagination', () => {
  it('renders pagination with page numbers', () => {
    render(
      <Pagination
        currentPage={1}
        totalPages={5}
        onPageChange={vi.fn()}
      />
    );
    expect(screen.getByText('1')).toBeInTheDocument();
    expect(screen.getByText('5')).toBeInTheDocument();
  });

  it('calls onPageChange when page is clicked', () => {
    const handlePageChange = vi.fn();
    render(
      <Pagination
        currentPage={1}
        totalPages={5}
        onPageChange={handlePageChange}
      />
    );
    
    const page2 = screen.getByText('2');
    fireEvent.click(page2);
    
    expect(handlePageChange).toHaveBeenCalledWith(2);
  });

  it('calls onPageChange with previous page when prev button clicked', () => {
    const handlePageChange = vi.fn();
    render(
      <Pagination
        currentPage={3}
        totalPages={5}
        onPageChange={handlePageChange}
      />
    );
    
    const prevButton = screen.getAllByRole('button')[0];
    fireEvent.click(prevButton);
    
    expect(handlePageChange).toHaveBeenCalledWith(2);
  });

  it('calls onPageChange with next page when next button clicked', () => {
    const handlePageChange = vi.fn();
    render(
      <Pagination
        currentPage={3}
        totalPages={5}
        onPageChange={handlePageChange}
      />
    );
    
    const buttons = screen.getAllByRole('button');
    const nextButton = buttons[buttons.length - 1];
    fireEvent.click(nextButton);
    
    expect(handlePageChange).toHaveBeenCalledWith(4);
  });

  it('disables previous button on first page', () => {
    render(
      <Pagination
        currentPage={1}
        totalPages={5}
        onPageChange={vi.fn()}
      />
    );
    
    const prevButton = screen.getAllByRole('button')[0];
    expect(prevButton).toBeDisabled();
  });

  it('disables next button on last page', () => {
    render(
      <Pagination
        currentPage={5}
        totalPages={5}
        onPageChange={vi.fn()}
      />
    );
    
    const buttons = screen.getAllByRole('button');
    const nextButton = buttons[buttons.length - 1];
    expect(nextButton).toBeDisabled();
  });

  it('marks current page as active', () => {
    render(
      <Pagination
        currentPage={3}
        totalPages={5}
        onPageChange={vi.fn()}
      />
    );
    
    const activePage = screen.getByText('3');
    expect(activePage).toHaveClass('bg-accent-primary');
  });

  it('renders ellipsis for many pages', () => {
    render(
      <Pagination
        currentPage={5}
        totalPages={10}
        onPageChange={vi.fn()}
      />
    );
    
    // Should show ellipsis
    expect(document.querySelectorAll('svg').length).toBeGreaterThan(0);
  });

  it('applies custom className', () => {
    render(
      <Pagination
        currentPage={1}
        totalPages={3}
        onPageChange={vi.fn()}
        className="custom-class"
      />
    );
    
    const container = document.querySelector('.custom-class');
    expect(container).toBeInTheDocument();
  });
});
