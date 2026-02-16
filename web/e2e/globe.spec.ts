import { test, expect } from '@playwright/test';

test.describe('Globe View', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/globe');
  });

  test('should display 3D globe canvas', async ({ page }) => {
    const globe = page.locator('[data-testid="globe-canvas"]');
    await expect(globe).toBeVisible();
  });

  test('should display layer toggles', async ({ page }) => {
    const layerToggles = page.locator('[data-testid="layer-toggles"]');
    await expect(layerToggles).toBeVisible();
  });

  test('should toggle sensor heatmap layer', async ({ page }) => {
    const heatmapToggle = page.locator('[data-testid="layer-sensor-heatmap"]');
    await expect(heatmapToggle).toBeVisible();
    
    // Click to toggle off
    await heatmapToggle.click();
    await expect(heatmapToggle).not.toHaveClass(/active/);
    
    // Click to toggle on
    await heatmapToggle.click();
    await expect(heatmapToggle).toHaveClass(/active/);
  });

  test('should toggle facilities layer', async ({ page }) => {
    const facilitiesToggle = page.locator('[data-testid="layer-facilities"]');
    await expect(facilitiesToggle).toBeVisible();
    
    await facilitiesToggle.click();
    await expect(facilitiesToggle).not.toHaveClass(/active/);
  });

  test('should display time slider', async ({ page }) => {
    const timeSlider = page.locator('[data-testid="time-slider"]');
    await expect(timeSlider).toBeVisible();
  });

  test('should play/pause time animation', async ({ page }) => {
    const playButton = page.locator('[data-testid="play-pause-button"]');
    await expect(playButton).toBeVisible();
    
    // Click play
    await playButton.click();
    await expect(playButton).toHaveAttribute('aria-label', /pause/i);
    
    // Click pause
    await playButton.click();
    await expect(playButton).toHaveAttribute('aria-label', /play/i);
  });

  test('should display zoom controls', async ({ page }) => {
    const zoomIn = page.locator('[data-testid="zoom-in"]');
    const zoomOut = page.locator('[data-testid="zoom-out"]');
    
    await expect(zoomIn).toBeVisible();
    await expect(zoomOut).toBeVisible();
  });

  test('should display coordinates', async ({ page }) => {
    const coordinates = page.locator('[data-testid="coordinates"]');
    await expect(coordinates).toBeVisible();
    await expect(coordinates).toContainText(/Lat:/);
    await expect(coordinates).toContainText(/Lon:/);
  });

  test('should fly to location on sensor selection', async ({ page }) => {
    // Click on a sensor marker
    const sensorMarker = page.locator('[data-testid="sensor-marker"]').first();
    await sensorMarker.click();
    
    // Should show sensor details
    const sensorDetails = page.locator('[data-testid="sensor-details"]');
    await expect(sensorDetails).toBeVisible();
  });

  test('should handle mouse interactions', async ({ page }) => {
    const globe = page.locator('[data-testid="globe-canvas"]');
    
    // Drag to rotate
    await globe.dragTo(globe, {
      sourcePosition: { x: 100, y: 100 },
      targetPosition: { x: 200, y: 100 },
    });
    
    // Coordinates should update
    const coordinates = page.locator('[data-testid="coordinates"]');
    await expect(coordinates).not.toContainText('Lat: 0.0000');
  });

  test('should handle scroll to zoom', async ({ page }) => {
    const globe = page.locator('[data-testid="globe-canvas"]');
    
    // Get initial zoom level
    const initialZoom = await page.evaluate(() => {
      return (window as unknown as { globeZoom: number }).globeZoom;
    });
    
    // Simulate mouse wheel scroll
    await globe.dispatchEvent('wheel', {
      deltaY: -100,
      bubbles: true,
    });
    
    // Zoom level should change
    const newZoom = await page.evaluate(() => {
      return (window as unknown as { globeZoom: number }).globeZoom;
    });
    
    expect(newZoom).not.toBe(initialZoom);
  });

});

test.describe('Globe View - Mobile', () => {
  test('should support touch gestures', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/globe');
    
    const globe = page.locator('[data-testid="globe-canvas"]');
    
    // Simulate pinch zoom
    await globe.evaluate((el) => {
      const touch1 = new Touch({
        identifier: 1,
        target: el,
        clientX: 100,
        clientY: 100,
      });
      const touch2 = new Touch({
        identifier: 2,
        target: el,
        clientX: 200,
        clientY: 200,
      });
      
      el.dispatchEvent(new TouchEvent('touchstart', {
        touches: [touch1, touch2],
        bubbles: true,
      }));
    });
    
    await expect(globe).toBeVisible();
  });
});
