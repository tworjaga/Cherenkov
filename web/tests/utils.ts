import { expect, type Page } from '@playwright/test';

export async function waitForHydration(page: Page): Promise<void> {
  await page.waitForFunction(() => document.readyState === 'complete');
}

export async function mockApiResponse<T>(
  page: Page,
  url: string,
  data: T,
  status = 200
): Promise<void> {
  await page.route(url, async (route) => {
    await route.fulfill({
      status,
      contentType: 'application/json',
      body: JSON.stringify(data),
    });
  });
}

export function createMockSensor(overrides = {}) {
  return {
    id: 'sensor-1',
    name: 'Test Sensor',
    location: { lat: 51.5074, lng: -0.1278 },
    status: 'active',
    lastReading: new Date().toISOString(),
    ...overrides,
  };
}

export function createMockAlert(overrides = {}) {
  return {
    id: 'alert-1',
    severity: 'warning',
    message: 'Test alert message',
    timestamp: new Date().toISOString(),
    acknowledged: false,
    ...overrides,
  };
}

export async function takeScreenshot(page: Page, name: string): Promise<void> {
  await page.screenshot({
    path: `./test-results/screenshots/${name}.png`,
    fullPage: true,
  });
}

export function expectToBeVisible(locator: ReturnType<Page['locator']>): Promise<void> {
  return expect(locator).toBeVisible();
}
