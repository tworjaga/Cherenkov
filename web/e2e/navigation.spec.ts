import { test, expect } from '@playwright/test';

test.describe('Navigation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display header with logo', async ({ page }) => {
    await expect(page.locator('[data-testid="header"]')).toBeVisible();
    await expect(page.locator('[data-testid="logo"]')).toContainText('CHERENKOV');
  });

  test('should navigate to different views via sidebar', async ({ page }) => {
    // Click on Dashboard
    await page.click('[data-testid="nav-dashboard"]');
    await expect(page).toHaveURL(/.*dashboard/);
    
    // Click on Globe
    await page.click('[data-testid="nav-globe"]');
    await expect(page).toHaveURL(/.*globe/);
    
    // Click on Sensors
    await page.click('[data-testid="nav-sensors"]');
    await expect(page).toHaveURL(/.*sensors/);
    
    // Click on Anomalies
    await page.click('[data-testid="nav-anomalies"]');
    await expect(page).toHaveURL(/.*anomalies/);
  });

  test('should toggle sidebar collapse', async ({ page }) => {
    const sidebar = page.locator('[data-testid="sidebar"]');
    
    // Initially expanded
    await expect(sidebar).not.toHaveClass(/collapsed/);
    
    // Click collapse button
    await page.click('[data-testid="toggle-sidebar"]');
    await expect(sidebar).toHaveClass(/collapsed/);
    
    // Click again to expand
    await page.click('[data-testid="toggle-sidebar"]');
    await expect(sidebar).not.toHaveClass(/collapsed/);
  });

  test('should toggle right panel', async ({ page }) => {
    const rightPanel = page.locator('[data-testid="right-panel"]');
    
    // Initially open
    await expect(rightPanel).toBeVisible();
    
    // Click toggle
    await page.click('[data-testid="toggle-right-panel"]');
    await expect(rightPanel).not.toBeVisible();
  });

  test('should toggle bottom panel', async ({ page }) => {
    const bottomPanel = page.locator('[data-testid="bottom-panel"]');
    
    // Initially open
    await expect(bottomPanel).toBeVisible();
    
    // Click toggle
    await page.click('[data-testid="toggle-bottom-panel"]');
    await expect(bottomPanel).not.toBeVisible();
  });
});

test.describe('Responsive Layout', () => {
  test('should adapt to mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');
    
    // Sidebar should be hidden or converted to bottom nav
    const sidebar = page.locator('[data-testid="sidebar"]');
    await expect(sidebar).not.toBeVisible();
  });

  test('should adapt to tablet viewport', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto('/');
    
    // Sidebar should be collapsed or icons only
    const sidebar = page.locator('[data-testid="sidebar"]');
    await expect(sidebar).toBeVisible();
  });
});
