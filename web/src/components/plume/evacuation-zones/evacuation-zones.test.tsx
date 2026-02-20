import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { EvacuationZones } from './evacuation-zones';

describe('EvacuationZones', () => {
  it('renders default zones when no particles provided', () => {
    render(<EvacuationZones />);
    
    expect(screen.getByText('Emergency Response Zones')).toBeInTheDocument();
    expect(screen.getByText('Immediate Evacuation Zone')).toBeInTheDocument();
    expect(screen.getByText('Shelter in Place Zone')).toBeInTheDocument();
    expect(screen.getByText('Monitoring Zone')).toBeInTheDocument();
  });

  it('calculates zones from particle data', () => {
    const particles = [
      { x: 0.1, y: 0.1, z: 0, concentration: 10000000 }, // High concentration
      { x: 0.5, y: 0.5, z: 0, concentration: 1000000 },  // Medium concentration
      { x: 1.0, y: 1.0, z: 0, concentration: 100000 },    // Low concentration
    ];
    
    render(
      <EvacuationZones
        particles={particles}
        releaseLat={0}
        releaseLng={0}
        isotope="Cs-137"
      />
    );
    
    expect(screen.getByText('Calculated from 3 particle measurements')).toBeInTheDocument();
  });

  it('displays dose rate for each zone', () => {
    render(<EvacuationZones />);
    
    const doseRates = screen.getAllByText(/μSv\/h/);
    expect(doseRates.length).toBeGreaterThan(0);
  });

  it('shows zone severity badges', () => {
    render(<EvacuationZones />);
    
    expect(screen.getByText('Critical')).toBeInTheDocument();
    expect(screen.getByText('High')).toBeInTheDocument();
    expect(screen.getByText('Medium')).toBeInTheDocument();
  });

  it('displays evacuation instructions', () => {
    render(<EvacuationZones />);
    
    expect(screen.getByText(/Evacuate immediately/)).toBeInTheDocument();
    expect(screen.getByText(/Close all windows/)).toBeInTheDocument();
    expect(screen.getByText(/Stay alert/)).toBeInTheDocument();
  });

  it('shows population estimates', () => {
    render(<EvacuationZones />);
    
    const populations = screen.getAllByText(/Population:/);
    expect(populations.length).toBeGreaterThan(0);
  });

  it('displays radius information', () => {
    render(<EvacuationZones />);
    
    const radiuses = screen.getAllByText(/km radius/);
    expect(radiuses.length).toBeGreaterThan(0);
  });


  it('uses custom zones when provided', () => {
    const customZones = [
      {
        id: 'custom-1',
        name: 'Custom Zone',
        radius: 3,
        severity: 'low' as const,
        population: 1000,
        instructions: 'Custom instructions',
        doseRate: 5,
      },
    ];
    
    render(<EvacuationZones zones={customZones} />);
    
    expect(screen.getByText('Custom Zone')).toBeInTheDocument();
    expect(screen.getByText('Custom instructions')).toBeInTheDocument();
  });
});
