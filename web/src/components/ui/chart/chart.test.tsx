import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { ChartContainer, ChartTooltip } from './index';


describe('Chart', () => {
  it('renders chart container with children', () => {
    render(
      <ChartContainer>
        <div data-testid="chart-content">Chart Content</div>
      </ChartContainer>
    );
    expect(screen.getByTestId('chart-content')).toBeInTheDocument();
  });

  it('applies custom className to chart container', () => {
    render(
      <ChartContainer className="custom-chart">
        <div>Chart</div>
      </ChartContainer>
    );
    const container = document.querySelector('.custom-chart');
    expect(container).toBeInTheDocument();
  });

  it('renders chart tooltip with content', () => {
    render(
      <ChartTooltip
        active={true}
        payload={[{ value: 100, name: 'Test', color: '#00ff88' }]}

        label="Test Label"
      />
    );
    expect(screen.getByText('Test Label')).toBeInTheDocument();
  });

  it('does not render tooltip when not active', () => {
    const { container } = render(
      <ChartTooltip
        active={false}
        payload={[]}
        label="Test"
      />
    );
    expect(container.firstChild).toBeNull();
  });
});
