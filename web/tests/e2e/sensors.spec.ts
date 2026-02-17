import { test, expect } from '@playwright/test';

test.describe('Sensors Management', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/sensors');
  });

  test('should display sensors table', async ({ page }) => {
    await expect(page.getByRole('table')).toBeVisible();
    await expect(page.getByText(/sensor name/i)).toBeVisible();
    await expect(page.getByText(/status/i)).toBeVisible();
    await expect(page.getByText(/location/i)).toBeVisible();
  });

  test('should filter sensors by status', async ({ page }) => {
    const filter = page.getByLabel(/filter by status/i);
    await filter.click();
    await page.getByRole('option', { name: /active/i }).click();
    
    const rows = page.getByRole('row');
    await expect(rows.first()).toContainText(/active/i);
  });

  test('should search sensors by name', async ({ page }) => {
    const search = page.getByPlaceholder(/search sensors/i);
    await search.fill('gamma');
    
    await expect(page.getByText(/gamma/i)).toBeVisible();
  });

  test('should navigate to sensor detail', async ({ page }) => {
    const sensorRow = page.getByRole('row').nth(1);
    await sensorRow.click();
    
    await expect(page).toHaveURL(/.*sensor/);
  });
});
