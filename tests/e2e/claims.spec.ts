import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('@claim:demo-isolation demo requests stay in the demo namespace',async({page})=>{
  const outside:string[]=[];
  page.on('request',r=>{if(new URL(r.url()).origin!=='http://127.0.0.1:4173')outside.push(r.url());});
  await page.goto('/demo');
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  await page.getByRole('button',{name:'Add to request'}).first().click();
  await page.getByRole('button',{name:/Review request/}).click();
  await page.getByLabel('Your name').fill('Maya Patel');
  await page.getByLabel('Company').fill('Juniper Corner');
  await page.getByLabel('Email').fill('maya@example.test');
  await page.getByRole('button',{name:'Send quote request'}).click();
  await expect(page.getByRole('heading',{name:/Request RQ-DEMO-01 received/})).toBeVisible();
  const keys=await page.evaluate(()=>Object.keys(localStorage));
  expect(keys).toEqual(['demo:client-catalogue-request:requests']);
  expect(outside).toEqual([]);
});

test('@claim:poa-price blank prices are shown as POA',async({page})=>{
  await page.goto('/demo');
  await expect(page.getByRole('article').filter({hasText:'Custom paper tape'}).getByText('POA')).toBeVisible();
});

test('@claim:csv-export exports one row for every sample request line',async({page})=>{
  await page.goto('/demo/inbox');
  const downloadPromise=page.waitForEvent('download');
  await page.getByRole('button',{name:'Export CSV'}).click();
  const download=await downloadPromise;
  const stream=await download.createReadStream();
  const chunks:Buffer[]=[];for await(const chunk of stream)chunks.push(chunk);
  const csv=Buffer.concat(chunks).toString('utf8');
  expect(csv.split('\n')).toHaveLength(3);
  expect(csv).toContain('NW-101');
  expect(csv).toContain('PK-228');
});

test('@claim:structured-request keeps SKUs and quantities',async({page})=>{
  await page.goto('/demo/inbox');
  const request=page.getByRole('article').filter({hasText:'RQ-6C24A19E'});
  await expect(request.getByText('NW-101')).toBeVisible();
  await expect(request.getByRole('cell',{name:'24'})).toBeVisible();
  await expect(request.getByText('PO-1842')).toBeVisible();
});

test('@claim:no-card-data has no card fields or checkout in request form',async({page})=>{
  await page.goto('/demo');
  await page.getByRole('button',{name:'Add to request'}).first().click();
  await page.getByRole('button',{name:/Review request/}).click();
  await expect(page.getByText('It does not place or pay for an order.')).toBeVisible();
  await expect(page.locator('input[autocomplete="cc-number"]')).toHaveCount(0);
});

test('landing and demo have no serious accessibility findings',async({page},testInfo)=>{
  await page.goto(testInfo.project.name==='mobile'?'/demo':'/');
  await expect(page.locator('h1')).toHaveCount(1);
  const results=await new AxeBuilder({page:page as never}).withTags(['wcag2a','wcag2aa']).analyze();
  expect(results.violations.filter(v=>['serious','critical'].includes(v.impact||''))).toEqual([]);
});
