import { test, expect } from '@playwright/test';

test.describe('Dashboard', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard');
  });

  test('should display global status indicator', async ({ page }) => {
    const statusIndicator = page.locator('[data-testid="global-status"]');
    await expect(statusIndicator).toBeVisible();
    await expect(statusIndicator).toContainText(/DEFCON/);
  });

  test('should display alert feed', async ({ page }) => {
    const alertFeed = page.locator('[data-testid="alert-feed"]');
    await expect(alertFeed).toBeVisible();
  });

  test('should display global radiation chart', async ({ page }) => {
    const chart = page.locator('[data-testid="global-chart"]');
    await expect(chart).toBeVisible();
  });

  test('should acknowledge alerts', async ({ page }) => {
    // Wait for alerts to load
    await page.waitForSelector('[data-testid="alert-item"]', { timeout: 5000 });
    
    const firstAlert = page.locator('[data-testid="alert-item"]').first();
    await expect(firstAlert).toBeVisible();
    
    // Click acknowledge button
    await firstAlert.locator('[data-testid="acknowledge-alert"]').click();
    
    // Alert should be marked as acknowledged
    await expect(firstAlert).toHaveClass(/acknowledged/);
  });

  test('should filter alerts by severity', async ({ page }) => {
    // Click critical filter
    await page.click('[data-testid="filter-critical"]');
    
    // Only critical alerts should be visible
    const alerts = page.locator('[data-testid="alert-item"]');
    const count = await alerts.count();
    
    for (let i = 0; i < count; i++) {
      await expect(alerts.nth(i)).toHaveClass(/critical/);
    }
  });

  test('should display connection status', async ({ page }) => {
    const connectionStatus = page.locator('[data-testid="connection-status"]');
    await expect(connectionStatus).toBeVisible();
    await expect(connectionStatus).toContainText(/Connected|Disconnected/);
  });

  test('should show last update timestamp', async ({ page }) => {
    const timestamp = page.locator('[data-testid="last-update"]');
    await expect(timestamp).toBeVisible();
  });
});

test.describe('Dashboard - Real-time Updates', () => {
  test('should receive WebSocket updates', async ({ page }) => {
    await page.goto('/dashboard');
    
    // Mock WebSocket connection
    await page.evaluate(() => {
      window.dispatchEvent(new CustomEvent('ws:connected'));
    });
    
    // Simulate incoming alert
    await page.evaluate(() => {
      window.dispatchEvent(new CustomEvent('ws:alert', {
        detail: {
          id: 'test-alert',
          severity: 'HIGH',
          title: 'Test Alert',
          description: 'Test description',
        },
      }));
    });
    
    // New alert should appear
    const alert = page.locator('[data-testid="alert-item"]').filter({ hasText: 'Test Alert' });
    await expect(alert).toBeVisible();
  });
});
