import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { ThemeToggle } from './ThemeToggle';

const mockSetTheme = vi.fn();
const mockGetAttribute = vi.fn();
const mockSetAttribute = vi.fn();

vi.mock('../../stores/useAppStore', () => ({
  useAppStore: (selector: (state: { theme: string; setTheme: (t: string) => void }) => string | ((t: string) => void)) => {
    const state = {
      theme: 'DARK',
      setTheme: mockSetTheme,
    };
    return selector(state);
  },
}));

describe('ThemeToggle', () => {
  beforeEach(() => {
    mockSetTheme.mockClear();
    mockGetAttribute.mockReturnValue(null);
    mockSetAttribute.mockClear();
    
    Object.defineProperty(document, 'documentElement', {
      value: {
        getAttribute: mockGetAttribute,
        setAttribute: mockSetAttribute,
      },
      writable: true,
    });
  });

  it('renders theme toggle button', () => {
    render(<ThemeToggle />);
    const button = screen.getByRole('button');
    expect(button).toBeInTheDocument();
  });

  it('calls setTheme when clicked', () => {
    render(<ThemeToggle />);
    const button = screen.getByRole('button');
    fireEvent.click(button);
    expect(mockSetTheme).toHaveBeenCalledWith('LIGHT');
  });

  it('sets data-theme attribute on document when clicked', () => {
    render(<ThemeToggle />);
    const button = screen.getByRole('button');
    fireEvent.click(button);
    expect(mockSetAttribute).toHaveBeenCalledWith('data-theme', 'light');
  });

  it('has correct ARIA label', () => {
    render(<ThemeToggle />);
    const button = screen.getByRole('button');
    expect(button).toHaveAttribute('aria-label', 'Switch to light theme');
  });

  it('renders SVG icon', () => {
    render(<ThemeToggle />);
    const button = screen.getByRole('button');
    expect(button.querySelector('svg')).toBeInTheDocument();
  });
});
