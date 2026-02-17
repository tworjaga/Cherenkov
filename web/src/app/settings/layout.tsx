export default function SettingsLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="h-full w-full p-6 overflow-auto">
      <h1 className="text-display-md font-sans text-text-primary mb-6">Settings</h1>
      <div className="flex gap-6">
        <nav className="w-48 flex flex-col gap-2">
          <a 
            href="/settings/general" 
            className="px-4 py-2 rounded-md text-body-sm text-text-secondary hover:bg-bg-hover transition-colors"
          >
            General
          </a>
          <a 
            href="/settings/notifications" 
            className="px-4 py-2 rounded-md text-body-sm text-text-secondary hover:bg-bg-hover transition-colors"
          >
            Notifications
          </a>
          <a 
            href="/settings/data-sources" 
            className="px-4 py-2 rounded-md text-body-sm text-text-secondary hover:bg-bg-hover transition-colors"
          >
            Data Sources
          </a>
          <a 
            href="/settings/api-keys" 
            className="px-4 py-2 rounded-md text-body-sm text-text-secondary hover:bg-bg-hover transition-colors"
          >
            API Keys
          </a>
        </nav>
        <div className="flex-1">{children}</div>
      </div>
    </div>
  );
}
