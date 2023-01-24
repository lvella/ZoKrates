import puppeteer from "puppeteer";
import assert from "assert";
import path from "path";

describe("umd web tests", () => {
  it("verify", async () => {
    const browser = await puppeteer.launch({ headless: true });
    const page = await browser.newPage();

    let response = await page.goto(
      path.dirname(import.meta.url) + "/index.html"
    );
    assert(response.ok());

    let element = await page.waitForSelector("#result", {
      timeout: 5000,
      visible: true,
    });
    let value = await element.evaluate((el) => el.textContent, element);
    assert.equal(value, "true");

    await browser.close();
  });
});