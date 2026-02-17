'use client';

export default function DataSourcesSettingsPage() {
  return (
    <div className="space-y-6">
      <section className="bg-bg-secondary rounded-lg p-6 border border-border-subtle">
        <h2 className="text-heading-xs text-text-primary mb-4">DATA SOURCES</h2>
        <div className="space-y-4">
          <div className="flex items-center justify-between p-3 bg-bg-primary rounded-md">
            <div>
              <span className="text-body-sm text-text-primary">SAFECAST</span>
              <p className="text-body-xs text-text-tertiary">Crowdsourced radiation data</p>
            </div>
            <input type="checkbox" defaultChecked className="accent-accent-primary" />
          </div>
          <div className="flex items-center justify-between p-3 bg-bg-primary rounded-md">
            <div>
              <span className="text-body-sm text-text-primary">EURDEP</span>
              <p className="text-body-xs text-text-tertiary">European radiation data</p>
            </div>
            <input type="checkbox" defaultChecked className="accent-accent-primary" />
          </div>
          <div className="flex items-center justify-between p-3 bg-bg-primary rounded-md">
            <div>
              <span className="text-body-sm text-text-primary">NRC</span>
              <p className="text-body-xs text-text-tertiary">US Nuclear Regulatory Commission</p>
            </div>
            <input type="checkbox" defaultChecked className="accent-accent-primary" />
          </div>
        </div>
      </section>
    </div>
  );
}
