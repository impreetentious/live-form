import { test, expect } from "@playwright/test";

test("home heading", async ({ page }) => {
  const response = await page.goto("/");
  expect(response?.ok()).toBeTruthy();
  await expect(page.getByRole("heading", { name: "Liveform" })).toBeVisible();
  await expect(page.getByText("Loading the Colony shell.")).toBeVisible();
});
