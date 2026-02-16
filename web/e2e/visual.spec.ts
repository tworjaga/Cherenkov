import { test, expect } from '@playwright/test';

test.describe('Visual Regression', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard');
    // Wait for app to be fully loaded
    await page.waitForSelector('[data-testid="app-loaded"]', { timeout: 10000 });
  });

  test('dashboard layout matches snapshot', async ({ page }) => {
    await expect(page).toHaveScreenshot('dashboard.png', {
      fullPage: true,
      threshold: 0.2,
    });
  });

  test('sidebar navigation matches snapshot', async ({ page }) => {
    const sidebar = page.locator('[data-testid="sidebar"]');
    await expect(sidebar).toHaveScreenshot('sidebar.png');
  });

  test('alert panel matches snapshot', async ({ page }) => {
    const alertPanel = page.locator('[data-testid="right-panel"]');
    await expect(alertPanel).toHaveScreenshot('alert-panel.png');
  });

  test('global chart matches snapshot', async ({ page }) => {
    const chart = page.locator('[data-testid="global-chart"]');
    await expect(chart).toHaveScreenshot('global-chart.png');
  });

  test('dark theme matches snapshot', async ({ page }) => {
    // Ensure dark theme is active
    await page.evaluate(() => {
      document.documentElement.classList.add('dark');
    });
    
    await expect(page).toHaveScreenshot('dashboard-dark.png', {
      fullPage: true,
    });
  });

  test('mobile layout matches snapshot', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page).toHaveScreenshot('dashboard-mobile.png', {
      fullPage: true,
    });
  });

  test('tablet layout matches snapshot', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(page).toHaveScreenshot('dashboard-tablet.png', {
      fullPage: true,
    });
  });
});

test.describe('Globe Visual Regression', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/globe');
    await page.waitForSelector('[data-testid="globe-canvas"]', { timeout: 10000 });
  });

  test('globe view matches snapshot', async ({ page }) => {
    // Wait for WebGL to initialize
    await page.waitForTimeout(2000);
    
    await expect(page).toHaveScreenshot('globe-view.png', {
      fullPage: true,
      threshold: 0.3, // Higher threshold for WebGL content
    });
  });

  test('globe with sensors layer matches snapshot', async ({ page }) => {
    await page.click('[data-testid="layer-sensor-points"]');
    await page.waitForTimeout(1000);
    
    await expect(page).toHaveScreenshot('globe-sensors.png', {
      fullPage: true,
      threshold: 0.3,
    });
  });

  test('globe with facilities layer matches snapshot', async ({ page }) => {
    await page.click('[data-testid="layer-facilities"]');
    await page.waitForTimeout(1000);
    
    await expect(page).toHaveScreenshot('globe-facilities.png', {
      fullPage: true,
      threshold: 0.3,
    });
  });
});

test.describe('Component Visual Regression', () => {
  test('Toast notifications match snapshot', async ({ page }) => {
    await page.goto('/dashboard');
    
    // Trigger different severity toasts
    await page.evaluate(() => {
      window.dispatchEvent(new CustomEvent('toast:show', {
        detail: { severity: 'critical', title: 'Critical Alert', message: 'Test' },
      }));
    });
    
    const toast = page.locator('[role="alert"]').first();
    await expect(toast).toHaveScreenshot('toast-critical.png');
  });

  test('Modal dialogs match snapshot', async ({ page }) => {
    await page.goto('/dashboard');
    
    // Open a modal
    await page.click('[data-testid="open-settings"]');
    
    const modal = page.locator('[role="dialog"]');
    await expect(modal).toHaveScreenshot('settings-modal.png');
  });

  test('Loading states match snapshot', async ({ page }) => {
    await page.goto('/dashboard');
    
    // Show loading state
    await page.evaluate(() => {
      window.dispatchEvent(new CustomEvent('app:loading', { detail: true }));
    });
    
    const spinner = page.locator('[data-testid="loading-spinner"]');
    await expect(spinner).toHaveScreenshot('loading-spinner.png');
  });
});
