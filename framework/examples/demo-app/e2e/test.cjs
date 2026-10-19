const puppeteer = require('puppeteer');
const url = process.env.URL || 'http://127.0.0.1:8099';
(async () => {
  const browser = await puppeteer.launch({ headless: 'new' });
  for (const pagePath of ['/', '/about', '/contact']) {
    const page = await browser.newPage();
    const response = await page.goto(url + pagePath, { waitUntil: 'networkidle0' });
    const raw = await response.text();
    if (/<link[^>]+assets\/tailwind\.css/.test(raw) || /<script[^>]+type=["']module["'][^>]+src=/.test(raw)) {
      console.error('CSS or JS was loaded eagerly in', pagePath);
      process.exit(1);
    }
    for (let i = 0; i < 5; i++) { await page.click('#inc'); await new Promise(r => setTimeout(r, 100)); }
    const countText = await page.$eval('#count', e => e.textContent);
    console.log(pagePath, 'count:', countText);
    if (countText !== '5') { console.error('expected 5, got ' + countText); process.exit(1); }
    const lazyState = await page.evaluate(() => ({
      flag: document.documentElement.dataset.lazyCss,
      css: [...document.styleSheets].some(s => s.href && s.href.endsWith('/assets/tailwind.css')),
      link: !!document.querySelector('link[href="/assets/tailwind.css"]'),
    }));
    if (lazyState.flag !== 'ready' || !lazyState.css || !lazyState.link) {
      console.error('lazy CSS did not load after DOMContentLoaded', lazyState);
      process.exit(1);
    }
    const before = await page.$eval('#lazy', e => getComputedStyle(e).backgroundColor);
    await page.click('#lazy');
    await page.waitForFunction(() => {
      const el = document.getElementById('lazy');
      return !!el && getComputedStyle(el).backgroundColor === 'rgb(239, 68, 68)';
    }, { timeout: 5000 });
    const after = await page.$eval('#lazy', e => ({
      bg: getComputedStyle(e).backgroundColor,
      color: getComputedStyle(e).color,
      cls: e.className,
    }));
    console.log(pagePath, 'lazy:', before, '->', after.bg, after.color);
    if (!after.cls.includes('bg-red-500') || after.bg !== 'rgb(239, 68, 68)') {
      console.error('lazy CSS did not apply', after);
      process.exit(1);
    }
    await page.close();
  }
  await browser.close();
  console.log('PASS');
})();
