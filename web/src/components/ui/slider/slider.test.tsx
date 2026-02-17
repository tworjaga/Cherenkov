import { describe, it, expect, vi } from 'vitest';
import { render } from '@testing-library/react';
import { Slider } from './slider';


describe('Slider', () => {
  it('renders slider', () => {
    render(<Slider value={[50]} />);
    expect(document.querySelector('[role="slider"]')).toBeInTheDocument();
  });

  it('displays correct value', () => {
    render(<Slider value={[75]} />);
    const slider = document.querySelector('[role="slider"]');
    expect(slider).toHaveAttribute('aria-valuenow', '75');
  });

  it('calls onValueChange when value changes', () => {
    const handleChange = vi.fn();
    render(<Slider value={[50]} onValueChange={handleChange} />);
    expect(document.querySelector('[role="slider"]')).toBeInTheDocument();
  });

  it('respects min and max values', () => {
    render(<Slider value={[50]} min={0} max={100} />);
    const slider = document.querySelector('[role="slider"]');
    expect(slider).toHaveAttribute('aria-valuemin', '0');
    expect(slider).toHaveAttribute('aria-valuemax', '100');
  });

  it('is disabled when disabled prop is true', () => {
    render(<Slider value={[50]} disabled />);
    const slider = document.querySelector('[role="slider"]');
    expect(slider).toHaveAttribute('data-disabled', '');
  });

  it('applies custom className', () => {
    render(<Slider value={[50]} className="custom-class" />);
    const root = document.querySelector('.custom-class');
    expect(root).toBeInTheDocument();
  });

  it('respects step value', () => {
    render(<Slider value={[50]} step={5} />);
    const slider = document.querySelector('[role="slider"]');
    expect(slider).toBeInTheDocument();
  });
});
