const puppeteer = require('puppeteer-core');

(async () => {
  const browser = await puppeteer.launch();
  const page = await browser.newPage();
  await page.goto('http://localhost:8086');
  await page.screenshot({path: 'example.png'});
  await browser.close();
})();
