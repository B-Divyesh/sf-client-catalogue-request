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
  await expect(page.getByRole('heading',{name:/Request RQ-DEMO-[A-F0-9]{4} received/})).toBeVisible();
  const keys=await page.evaluate(()=>Object.keys(localStorage));
  expect(keys).toEqual(['demo:client-catalogue-request:requests']);
  expect(outside).toEqual([]);
});

test('resetting the demo creates a clean sample workspace',async({page})=>{
  await page.goto('/demo');
  const first=await page.evaluate(()=>localStorage.getItem('demo:client-catalogue-request:requests'));
  await page.getByRole('button',{name:'Reset demo'}).click();
  await expect.poll(()=>page.evaluate(()=>localStorage.getItem('demo:client-catalogue-request:requests'))).not.toBe(first);
  await page.getByRole('link',{name:'Seller sample'}).click();
  await expect(page.getByText('1 sample request')).toBeVisible();
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
  await expect(request.getByText('maya@example.test')).toBeVisible();
  await expect(request.getByText('Please quote delivery to Bristol.')).toBeVisible();
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

test('the request basket works from the keyboard',async({page})=>{
  await page.goto('/demo');
  const add=page.getByRole('button',{name:'Add to request'}).first();
  await add.focus();
  await page.keyboard.press('Enter');
  await expect(page.getByRole('button',{name:/Review request 1/})).toBeVisible();
  await page.getByRole('button',{name:/Review request 1/}).focus();
  await page.keyboard.press('Enter');
  await expect(page.getByRole('dialog')).toBeVisible();
  await page.keyboard.press('Escape');
  await expect(page.getByRole('dialog')).not.toBeVisible();
});

test('public routes keep the document skeleton and load without console errors',async({page},testInfo)=>{
  test.skip(testInfo.project.name==='mobile','The mobile project covers the interactive catalogue claims.');
  const errors:string[]=[];
  page.on('console',message=>{if(message.type()==='error')errors.push(message.text());});
  for(const path of ['/','/demo','/demo/inbox','/privacy','/terms','/missing-page']){
    await page.goto(path);
    await expect(page.locator('main')).toHaveCount(1);
    await expect(page.locator('h1')).toHaveCount(1);
    await expect(page).toHaveTitle(/Client Catalogue Request/);
    expect(await page.locator('html').getAttribute('lang')).toBe('en');
    expect(await page.locator('img:not([alt])').count()).toBe(0);
    const results=await new AxeBuilder({page:page as never}).withTags(['wcag2a','wcag2aa']).analyze();
    expect(results.violations.filter(v=>['serious','critical'].includes(v.impact||'')),path).toEqual([]);
  }
  expect(errors).toEqual([]);
});

test('@claim:print-request opens a print-ready request',async({page},testInfo)=>{
  test.skip(testInfo.project.name==='mobile','Print output is verified once; mobile catalogue behavior is covered separately.');
  await page.goto('/demo/inbox');
  await page.evaluate(()=>{(window as Window & {printed?:boolean}).printed=false;window.print=()=>{(window as Window & {printed?:boolean}).printed=true;};});
  await page.getByRole('button',{name:'Print request / save PDF'}).click();
  await expect(page.locator('.request-card.print-selected')).toHaveCount(1);
  expect(await page.evaluate(()=>(window as Window & {printed?:boolean}).printed)).toBe(true);
});

test('@claim:paid-license stores and verifies a returned license',async({page})=>{
  await page.route('https://api.sociobot.in/api/v1/products/client-catalogue-request/verify?license=test-license',route=>route.fulfill({json:{valid:true,reason:'ok',expires_at:null}}));
  await page.goto('/?license=test-license');
  await expect(page).toHaveURL('/');
  await expect(page.getByText('₹1,499')).toBeVisible();
  await expect(page.getByText('Use more than 12 catalogue rows and create more than one client link.')).toBeVisible();
  await expect(page.getByRole('link',{name:'Buy the full workspace'})).toHaveAttribute('href','https://api.sociobot.in/api/v1/products/client-catalogue-request/checkout');
  await expect.poll(()=>page.evaluate(()=>localStorage.getItem('sb_license:client-catalogue-request'))).toBe('test-license');
  await expect.poll(()=>page.evaluate(()=>JSON.parse(localStorage.getItem('sb_license_cache:client-catalogue-request')||'null')?.valid)).toBe(true);
});

test('@claim:privacy-runtime uses no third-party runtime assets',async({page})=>{
  const origins=new Set<string>();
  page.on('request',request=>origins.add(new URL(request.url()).origin));
  await page.goto('/');
  await page.goto('/demo');
  expect([...origins]).toEqual(['http://127.0.0.1:4173']);
});

test('@claim:csv-import seller import to client request works end to end',async({page},testInfo)=>{
  test.skip(testInfo.project.name==='mobile','The same responsive UI is covered by mobile claim tests.');
  await page.goto('/manage');
  const password='correct horse battery';
  if(await page.getByRole('heading',{name:'Set up your request desk'}).isVisible()){
    await page.getByLabel('Business name').fill('Northline Test Supply');
    await page.getByLabel('Password').fill(password);
    await page.getByRole('button',{name:'Create seller workspace'}).click();
  }else{
    await page.getByLabel('Password').fill(password);
    await page.getByRole('button',{name:'Open seller workspace'}).click();
  }
  await expect(page.getByRole('heading',{name:'Prepare links. Receive clean requests.'})).toBeVisible();
  await page.getByLabel('Choose a product CSV').setInputFiles({name:'products.csv',mimeType:'text/csv',buffer:Buffer.from('sku,name,description,category,price,stock_note\nT-1,Test tray,Oak tray,Service,14.50,In stock')});
  await page.getByRole('button',{name:'Save catalogue'}).click();
  await expect(page.locator('#catalogue').getByRole('cell',{name:'T-1',exact:true})).toBeVisible();
  if(await page.getByText('No client links yet').isVisible()){
    await page.getByLabel('Client or group name').fill('Test client');
    await page.getByRole('button',{name:'Create client link'}).click();
  }
  const clientHref=await page.getByRole('link',{name:/Open catalogue/}).first().getAttribute('href');
  await page.goto(clientHref!);
  await page.getByRole('button',{name:'Add to request'}).click();
  await page.getByRole('button',{name:/Review request/}).click();
  await page.getByLabel('Your name').fill('Alex Buyer');
  await page.getByLabel('Company').fill('Test client');
  await page.getByLabel('Email').fill('alex@example.test');
  await page.getByRole('button',{name:'Send quote request'}).click();
  await expect(page.getByRole('heading',{name:/Request RQ-/})).toBeVisible();
  await page.goto('/manage');
  await expect(page.getByText('alex@example.test').first()).toBeVisible();
});
