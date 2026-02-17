'use client';

export default function NotificationsSettingsPage() {
  return (
    <div className="space-y-6">
      <section className="bg-bg-secondary rounded-lg p-6 border border-border-subtle">
        <h2 className="text-heading-xs text-text-primary mb-4">NOTIFICATION SETTINGS</h2>
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <span className="text-body-sm text-text-secondary">Email Alerts</span>
            <input type="checkbox" defaultChecked className="accent-accent-primary" />
          </div>
          <div className="flex items-center justify-between">
            <span className="text-body-sm text-text-secondary">Push Notifications</span>
            <input type="checkbox" defaultChecked className="accent-accent-primary" />
          </div>
          <div className="flex items-center justify-between">
            <span className="text-body-sm text-text-secondary">Critical Alerts Only</span>
            <input type="checkbox" className="accent-accent-primary" />
          </div>
        </div>
      </section>
    </div>
  );
}
