import { test, expect } from '@playwright/test';

test.describe('Dashboard', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display header with DEFCON indicator', async ({ page }) => {
    await expect(page.locator('header')).toBeVisible();
    await expect(page.locator('[data-testid="defcon-indicator"]')).toBeVisible();
    await expect(page.locator('[data-testid="utc-clock"]')).toBeVisible();
  });

  test('should display sidebar navigation', async ({ page }) => {
    await expect(page.locator('[data-testid="sidebar"]')).toBeVisible();
    await expect(page.locator('[data-testid="nav-dashboard"]')).toBeVisible();
    await expect(page.locator('[data-testid="nav-globe"]')).toBeVisible();
  });

  test('should display globe viewport', async ({ page }) => {
    await expect(page.locator('[data-testid="globe-container"]')).toBeVisible();
  });

  test('should display right panel with alerts', async ({ page }) => {
    await expect(page.locator('[data-testid="right-panel"]')).toBeVisible();
    await expect(page.locator('[data-testid="alert-feed"]')).toBeVisible();
  });

  test('should display bottom panel with charts', async ({ page }) => {
    await expect(page.locator('[data-testid="bottom-panel"]')).toBeVisible();
  });

  test('keyboard shortcut "t" should toggle bottom panel', async ({ page }) => {
    await page.click('body');
    await page.waitForTimeout(100);
    await page.keyboard.press('t');
    await expect(page.locator('text=Show Analytics')).toBeVisible();
    await page.keyboard.press('t');
    await expect(page.locator('[data-testid="bottom-panel"]')).toBeVisible();
  });

  test('keyboard shortcut "1-5" should switch views', async ({ page }) => {
    await page.click('body');
    await page.waitForTimeout(100);
    await page.keyboard.press('1');
    await expect(page.locator('[data-testid="nav-dashboard"]')).toHaveClass(/border-l-2/);
    await page.keyboard.press('2');
    await expect(page.locator('[data-testid="nav-globe"]')).toHaveClass(/border-l-2/);
  });


});
