import { test, expect } from '@playwright/test';

test.describe('Globe Visualization', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/globe');
  });

  test('should render globe canvas', async ({ page }) => {
    const canvas = page.locator('canvas');
    await expect(canvas).toBeVisible();
  });

  test('should display layer toggle controls', async ({ page }) => {
    await expect(page.getByLabel(/sensor layer/i)).toBeVisible();
    await expect(page.getByLabel(/facility layer/i)).toBeVisible();
    await expect(page.getByLabel(/anomaly layer/i)).toBeVisible();
  });

  test('should toggle sensor layer', async ({ page }) => {
    const toggle = page.getByLabel(/sensor layer/i);
    await toggle.click();
    await expect(toggle).not.toBeChecked();
  });

  test('should display time slider', async ({ page }) => {
    await expect(page.getByRole('slider', { name: /time/i })).toBeVisible();
  });

  test('should zoom in and out', async ({ page }) => {
    const zoomIn = page.getByRole('button', { name: /zoom in/i });
    const zoomOut = page.getByRole('button', { name: /zoom out/i });
    
    await expect(zoomIn).toBeVisible();
    await expect(zoomOut).toBeVisible();
    
    await zoomIn.click();
    await zoomOut.click();
  });
});
