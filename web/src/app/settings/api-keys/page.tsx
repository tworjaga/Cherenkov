'use client';

export default function ApiKeysSettingsPage() {
  return (
    <div className="space-y-6">
      <section className="bg-bg-secondary rounded-lg p-6 border border-border-subtle">
        <h2 className="text-heading-xs text-text-primary mb-4">API KEYS</h2>
        <div className="space-y-4">
          <div className="p-3 bg-bg-primary rounded-md">
            <div className="flex items-center justify-between mb-2">
              <span className="text-body-sm text-text-primary">Production Key</span>
              <button className="text-body-xs text-accent-primary hover:text-accent-secondary">
                Regenerate
              </button>
            </div>
            <code className="block text-mono-xs text-text-tertiary bg-bg-secondary p-2 rounded">
              chrk_live_xxxxxxxxxxxxxxxxxxxxxxxx
            </code>
          </div>
          <div className="p-3 bg-bg-primary rounded-md">
            <div className="flex items-center justify-between mb-2">
              <span className="text-body-sm text-text-primary">Development Key</span>
              <button className="text-body-xs text-accent-primary hover:text-accent-secondary">
                Regenerate
              </button>
            </div>
            <code className="block text-mono-xs text-text-tertiary bg-bg-secondary p-2 rounded">
              chrk_dev_xxxxxxxxxxxxxxxxxxxxxxxx
            </code>
          </div>
        </div>
      </section>
    </div>
  );
}
