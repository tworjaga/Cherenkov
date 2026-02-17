'use client';

export default function GeneralSettingsPage() {
  return (
    <div className="space-y-6">
      <section className="bg-bg-secondary rounded-lg p-6 border border-border-subtle">
        <h2 className="text-heading-xs text-text-primary mb-4">GENERAL SETTINGS</h2>
        <div className="space-y-4">
          <div>
            <label className="block text-body-sm text-text-secondary mb-2">Theme</label>
            <select className="w-full bg-bg-primary border border-border-default rounded-md px-3 py-2 text-body-sm text-text-primary">
              <option value="dark">Dark</option>
              <option value="light">Light</option>
              <option value="system">System</option>
            </select>
          </div>
          <div>
            <label className="block text-body-sm text-text-secondary mb-2">Language</label>
            <select className="w-full bg-bg-primary border border-border-default rounded-md px-3 py-2 text-body-sm text-text-primary">
              <option value="en">English</option>
              <option value="de">German</option>
              <option value="fr">French</option>
            </select>
          </div>
        </div>
      </section>
    </div>
  );
}
