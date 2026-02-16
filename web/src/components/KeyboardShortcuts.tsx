import React from 'react';

interface Shortcut {
  key: string;
  description: string;
  context?: string;
}

const shortcuts: Shortcut[] = [
  { key: '?', description: 'Show keyboard shortcuts', context: 'Global' },
  { key: 'G', description: 'Go to Globe view', context: 'Navigation' },
  { key: 'D', description: 'Go to Dashboard', context: 'Navigation' },
  { key: 'S', description: 'Go to Sensors', context: 'Navigation' },
  { key: 'A', description: 'Go to Anomalies', context: 'Navigation' },
  { key: 'P', description: 'Go to Plume Simulator', context: 'Navigation' },
  { key: 'Esc', description: 'Close panels / Cancel selection', context: 'Global' },
  { key: 'Space', description: 'Play/Pause time', context: 'Globe' },
  { key: 'R', description: 'Reset view', context: 'Globe' },
  { key: 'L', description: 'Toggle layer panel', context: 'Globe' },
  { key: 'T', description: 'Toggle theme', context: 'Global' },
  { key: 'F', description: 'Toggle fullscreen', context: 'Global' },
  { key: 'A', description: 'Acknowledge selected alert', context: 'Alerts' },
  { key: '↑/↓', description: 'Navigate alerts', context: 'Alerts' },
  { key: 'Enter', description: 'Open selected alert', context: 'Alerts' },
];

interface KeyboardShortcutsProps {
  isOpen: boolean;
  onClose: () => void;
}

export const KeyboardShortcuts: React.FC<KeyboardShortcutsProps> = ({ isOpen, onClose }) => {
  if (!isOpen) return null;

  const grouped = shortcuts.reduce((acc, shortcut) => {
    const ctx = shortcut.context || 'Other';
    if (!acc[ctx]) acc[ctx] = [];
    acc[ctx].push(shortcut);
    return acc;
  }, {} as Record<string, Shortcut[]>);

  return (
    <div
      className="fixed inset-0 z-[200] flex items-center justify-center bg-black/60 backdrop-blur-sm"
      onClick={onClose}
      role="dialog"
      aria-modal="true"
      aria-labelledby="shortcuts-title"
    >
      <div
        className="bg-bg-secondary border border-border-subtle rounded-lg shadow-2xl w-[600px] max-w-[90vw] max-h-[80vh] overflow-hidden"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between p-4 border-b border-border-subtle">
          <h2 id="shortcuts-title" className="text-text-primary font-semibold text-lg">
            Keyboard Shortcuts
          </h2>
          <button
            onClick={onClose}
            className="p-2 hover:bg-bg-hover rounded transition-colors"
            aria-label="Close"
          >
            <svg className="w-5 h-5 text-text-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div className="p-4 overflow-y-auto max-h-[60vh]">
          {Object.entries(grouped).map(([context, items]) => (
            <div key={context} className="mb-6 last:mb-0">
              <h3 className="text-accent-primary text-xs uppercase tracking-wider font-semibold mb-3">
                {context}
              </h3>
              <div className="space-y-2">
                {items.map((shortcut) => (
                  <div
                    key={shortcut.key}
                    className="flex items-center justify-between py-2 px-3 bg-bg-tertiary/50 rounded"
                  >
                    <span className="text-text-secondary text-sm">{shortcut.description}</span>
                    <kbd className="px-2 py-1 bg-bg-primary border border-border-subtle rounded text-text-primary text-xs font-mono min-w-[60px] text-center">
                      {shortcut.key}
                    </kbd>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>

        <div className="p-4 border-t border-border-subtle bg-bg-tertiary/30">
          <p className="text-text-tertiary text-xs text-center">
            Press <kbd className="px-1 py-0.5 bg-bg-primary border border-border-subtle rounded text-text-secondary">?</kbd> to show this help at any time
          </p>
        </div>
      </div>
    </div>
  );
};
