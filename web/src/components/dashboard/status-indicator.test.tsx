import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { StatusIndicator } from './status-indicator';

describe('StatusIndicator', () => {
  it('renders DEFCON 5 (normal) status', () => {
    render(<StatusIndicator defcon={5} level="NORMAL" activeAlerts={0} />);
    expect(screen.getByText('DEFCON 5')).toBeInTheDocument();
    expect(screen.getByText('NORMAL')).toBeInTheDocument();
  });

  it('renders DEFCON 1 (critical) status', () => {
    render(<StatusIndicator defcon={1} level="CRITICAL" activeAlerts={5} />);
    expect(screen.getByText('DEFCON 1')).toBeInTheDocument();
    expect(screen.getByText('CRITICAL')).toBeInTheDocument();
  });

  it('displays active alerts count', () => {
    render(<StatusIndicator defcon={3} level="ELEVATED" activeAlerts={12} />);
    expect(screen.getByText('12')).toBeInTheDocument();
  });

  it('renders with pulse animation for critical levels', () => {
    const { container: c3 } = render(<StatusIndicator defcon={3} level="ELEVATED" activeAlerts={3} />);
    expect(c3.querySelector('[data-testid="pulse"]')).toBeInTheDocument();

    const { container: c5 } = render(<StatusIndicator defcon={5} level="NORMAL" activeAlerts={0} />);
    expect(c5.querySelector('[data-testid="pulse"]')).not.toBeInTheDocument();
  });
});
