import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('@claim:demo-isolation demo requests stay in the demo namespace',async({page})=>{
  const outside:string[]=[];
  const productOrigin=new URL(process.env.PLAYWRIGHT_BASE_URL||'http://127.0.0.1:4173').origin;
  page.on('request',r=>{if(new URL(r.url()).origin!==productOrigin)outside.push(r.url());});
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
  expect(keys).toEqual(['demo:client-catalogue-request:requests','demo:client-catalogue-request:submitted']);
  await page.goto('/demo/inbox');
  await expect(page.getByText('2 sample requests')).toBeVisible();
  expect(outside).toEqual([]);
});

test('@claim:demo-reset resetting the demo removes browser sample requests',async({page})=>{
  await page.goto('/demo');
  const first=await page.evaluate(()=>localStorage.getItem('demo:client-catalogue-request:requests'));
  await page.getByRole('button',{name:'Reset demo'}).click();
  await expect.poll(()=>page.evaluate(()=>localStorage.getItem('demo:client-catalogue-request:requests'))).not.toBe(first);
  await expect.poll(()=>page.evaluate(()=>localStorage.getItem('demo:client-catalogue-request:submitted'))).toBeNull();
  await page.getByRole('link',{name:'Seller sample'}).click();
  await expect(page.getByText('1 sample request')).toBeVisible();
});

test('@claim:demo-entry opens an isolated sample in one click and by query URL',async({page})=>{
  await page.goto('/');
  await page.getByRole('link',{name:'Try it with sample data'}).click();
  await expect(page).toHaveURL(/\/?\?demo=1$/);
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  await expect(page.getByRole('heading',{level:1,name:'Northline Supply Co.'})).toBeVisible();
  expect(await page.evaluate(()=>Object.keys(localStorage))).toEqual(['demo:client-catalogue-request:requests']);

  await page.evaluate(()=>localStorage.clear());
  await page.goto('/?demo=1');
  await expect(page.getByRole('heading',{level:1,name:'Northline Supply Co.'})).toBeVisible();
  await expect(page.getByRole('button',{name:'Reset demo'})).toBeVisible();
});

test('@claim:demo-sample-content provides six useful products and a seeded seller inbox',async({page})=>{
  await page.goto('/?demo=1');
  await expect(page.locator('.product-card')).toHaveCount(6);
  await expect(page.locator('.product-card').getByText('POA',{exact:true})).toHaveCount(2);
  await expect(page.getByText('Low stock — ask for lead time')).toBeVisible();
  await expect(page.getByText('Made to order — allow 3 weeks')).toBeVisible();
  await expect(page.getByRole('button',{name:/Review request/})).toBeVisible();
  await page.getByRole('link',{name:'Seller sample'}).click();
  await expect(page.getByText('1 sample request')).toBeVisible();
  await expect(page.getByText('Juniper Corner')).toBeVisible();
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

test('@claim:free-export exports requests without a license',async({page})=>{
  await page.goto('/demo/inbox?demo=1');
  expect(await page.evaluate(()=>localStorage.getItem('sb_license:client-catalogue-request'))).toBeNull();
  const downloadPromise=page.waitForEvent('download');
  await page.getByRole('button',{name:'Export CSV'}).click();
  const download=await downloadPromise;
  expect(download.suggestedFilename()).toBe('sample-quote-requests.csv');
  const stream=await download.createReadStream();
  const chunks:Buffer[]=[];for await(const chunk of stream)chunks.push(chunk);
  expect(Buffer.concat(chunks).toString('utf8')).toContain('RQ-6C24A19E');
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

test('@claim:service-boundaries labels requests as unconfirmed and keeps fulfilment outside the service',async({page})=>{
  await page.goto('/?demo=1');
  await page.getByRole('button',{name:'Add to request'}).first().click();
  await page.getByRole('button',{name:/Review request/}).click();
  await expect(page.getByText('This sends a quote request. It does not place or pay for an order.')).toBeVisible();
  await page.goto('/terms');
  await expect(page.getByText('A request is not an accepted order, stock promise, shipping quote, or tax invoice.')).toBeVisible();
});

test('@claim:billing-handoff uses Sociobot for checkout and locks a refunded license',async({page})=>{
  await page.goto('/');
  await expect(page.getByRole('link',{name:'Buy the full workspace'})).toHaveAttribute('href','https://api.sociobot.in/api/v1/products/client-catalogue-request/checkout');
  await page.goto('/terms');
  await expect(page.getByText(/Sociobot, the merchant of record/)).toBeVisible();
  await expect(page.getByText(/A refund revokes the related license/)).toBeVisible();
  await page.route('https://api.sociobot.in/api/v1/products/client-catalogue-request/verify?license=refunded-license',route=>route.fulfill({json:{valid:false,reason:'revoked',expires_at:null}}));
  await page.goto('/?license=refunded-license');
  await expect(page.getByRole('status')).toContainText('License no longer active');
});

test('@claim:browser-storage keeps sign-in, license, and demo data in their documented namespaces',async({page})=>{
  await page.addInitScript(()=>sessionStorage.setItem('ccr:session','test-seller:storage-claim'));
  await page.route('https://api.sociobot.in/api/v1/products/client-catalogue-request/verify?license=storage-license',route=>route.fulfill({json:{valid:true,reason:'ok',expires_at:null}}));
  await page.goto('/?license=storage-license');
  await expect(page).toHaveURL('/');
  expect(await page.evaluate(()=>Object.keys(sessionStorage))).toEqual(['ccr:session']);
  await expect.poll(()=>page.evaluate(()=>Object.keys(localStorage).sort())).toEqual(['sb_license:client-catalogue-request','sb_license_cache:client-catalogue-request']);
  await page.goto('/?demo=1');
  await expect.poll(()=>page.evaluate(()=>Object.keys(localStorage).filter(key=>key.startsWith('demo:')))).toEqual(['demo:client-catalogue-request:requests']);
  await page.getByRole('link',{name:'Start for real'}).click();
  await expect.poll(()=>page.evaluate(()=>Object.keys(localStorage).filter(key=>key.startsWith('demo:')))).toEqual([]);
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

test('an out-of-range quantity is announced and never submitted',async({page})=>{
  await page.goto('/demo');
  await page.getByRole('button',{name:'Add to request'}).first().click();
  await page.getByRole('button',{name:/Review request/}).click();
  const quantity=page.getByLabel('Quantity');
  await page.getByLabel('Your name').fill('Maya Patel');
  await page.getByLabel('Company').fill('Juniper Corner');
  await page.getByLabel('Email').fill('maya@example.test');
  for(const invalid of ['0','10000']){
    await quantity.fill(invalid);
    await page.getByRole('button',{name:'Send quote request'}).click();
    await expect(page.getByRole('alert')).toContainText('Enter a whole number from 1 to 9,999.');
    await expect(quantity).toHaveValue(invalid);
    expect(await page.evaluate(()=>localStorage.getItem('demo:client-catalogue-request:submitted'))).toBeNull();
  }

  await quantity.fill('25');
  await page.getByRole('button',{name:'Send quote request'}).click();
  await expect(page.getByRole('heading',{name:/Request RQ-DEMO-/})).toBeVisible();
  expect(await page.evaluate(()=>JSON.parse(localStorage.getItem('demo:client-catalogue-request:submitted')||'[]')[0].lines[0].quantity)).toBe(25);
});

test('the first keyboard stop is the skip link',async({page})=>{
  await page.goto('/');
  await page.keyboard.press('Tab');
  await expect(page.locator('.skip')).toBeFocused();
});

test('client-side navigation moves focus to the new page heading',async({page})=>{
  await page.goto('/');
  await page.locator('.site-header nav').getByRole('link',{name:'Demo'}).click();
  await expect(page.getByRole('heading',{level:1,name:'Northline Supply Co.'})).toBeFocused();
  await page.goBack();
  await expect(page.getByRole('heading',{level:1,name:'Turn repeat orders into clear requests'})).toBeFocused();
});

test('production policy allows Entra discovery and requests the seller API scope',async({page},testInfo)=>{
  test.skip(testInfo.project.name==='mobile','The same shared sign-in configuration is verified once.');
  const consoleErrors:string[]=[];
  page.on('console',message=>{if(message.type()==='error')consoleErrors.push(message.text());});
  const response=await page.goto(process.env.PLAYWRIGHT_BASE_URL?'/manage':'http://127.0.0.1:8080/manage');
  expect(response?.headers()['content-security-policy']).toContain('connect-src \'self\' https://api.sociobot.in https://sociobotcustomers.ciamlogin.com');
  const authorize=page.waitForResponse(response=>response.url().includes('sociobotcustomers.ciamlogin.com')&&response.url().includes('/oauth2/v2.0/authorize'));
  await page.getByRole('button',{name:'Sign in with Sociobot'}).click();
  const responseFromEntra=await authorize;
  const request=responseFromEntra.request();
  expect(responseFromEntra.status()).toBe(200);
  expect(new URL(request.url()).searchParams.get('scope')).toContain('api://25c704f4-465a-47af-80ab-2c489466b697/access_as_user');
  await expect(page).toHaveURL(/^https:\/\/sociobotcustomers\.ciamlogin\.com\//);
  await expect(page).toHaveTitle(/Sign in to your account/);
  expect(await page.locator('body').innerText()).not.toContain('AADSTS');
  expect(consoleErrors.filter(error=>error.includes('Content Security Policy'))).toEqual([]);
});

test('mobile controls meet touch and text-reflow boundaries',async({page},testInfo)=>{
  test.skip(testInfo.project.name!=='mobile','This regression is specific to the 390 px mobile layout.');
  await page.goto('/demo');
  for(const selector of ['.wordmark','.site-header nav a','.chip','footer nav a']){
    const boxes=await page.locator(selector).evaluateAll(nodes=>nodes.map(node=>{const box=(node as HTMLElement).getBoundingClientRect();return {width:box.width,height:box.height};}));
    for(const box of boxes.filter(box=>box.width>0&&box.height>0)){expect(box.width,selector).toBeGreaterThanOrEqual(44);expect(box.height,selector).toBeGreaterThanOrEqual(44);}
  }
  await page.goto('/');
  await expect.poll(()=>page.evaluate(()=>document.documentElement.scrollWidth)).toBeLessThanOrEqual(390);
  await expect(page.getByRole('link',{name:'Try it with sample data'})).toBeInViewport();
  const privacyLink=await page.getByRole('link',{name:'Read how request data is handled'}).boundingBox();
  expect(privacyLink?.height).toBeGreaterThanOrEqual(44);
  await page.goto('/demo/inbox');
  const emailLink=await page.getByRole('link',{name:'maya@example.test'}).boundingBox();
  expect(emailLink?.height).toBeGreaterThanOrEqual(44);
  const inboxGeometry=async()=>page.evaluate(()=>{
    const card=document.querySelector<HTMLElement>('.request-card')!;
    const tableWrap=card.querySelector<HTMLElement>('.table-wrap')!;
    const cardBox=card.getBoundingClientRect();
    const wrapBox=tableWrap.getBoundingClientRect();
    return {
      clientWidth:document.documentElement.clientWidth,
      scrollWidth:document.documentElement.scrollWidth,
      cardLeft:cardBox.left,
      cardRight:cardBox.right,
      wrapLeft:wrapBox.left,
      wrapRight:wrapBox.right,
      tableClientWidth:tableWrap.clientWidth,
      tableScrollWidth:tableWrap.scrollWidth,
    };
  });
  const expectInboxToReflow=(geometry:Awaited<ReturnType<typeof inboxGeometry>>)=>{
    expect(geometry.scrollWidth).toBe(geometry.clientWidth);
    expect(geometry.cardLeft).toBeGreaterThanOrEqual(0);
    expect(geometry.cardRight).toBeLessThanOrEqual(geometry.clientWidth);
    expect(geometry.wrapLeft).toBeGreaterThanOrEqual(geometry.cardLeft);
    expect(geometry.wrapRight).toBeLessThanOrEqual(geometry.cardRight);
    expect(geometry.tableScrollWidth).toBeGreaterThan(geometry.tableClientWidth);
  };
  expectInboxToReflow(await inboxGeometry());
  await page.goto('/demo');
  for(const selector of ['.demo-banner button','.demo-banner a']){
    const boxes=await page.locator(selector).evaluateAll(nodes=>nodes.map(node=>{const box=(node as HTMLElement).getBoundingClientRect();return {width:box.width,height:box.height};}));
    for(const box of boxes){expect(box.width,selector).toBeGreaterThanOrEqual(44);expect(box.height,selector).toBeGreaterThanOrEqual(44);}
  }
  await page.setViewportSize({width:320,height:844});
  await page.goto('/demo/inbox');
  await page.evaluate(()=>document.documentElement.style.fontSize='200%');
  expectInboxToReflow(await inboxGeometry());
  for(const locator of [
    page.getByRole('link',{name:'Start for real'}),
    page.getByRole('link',{name:'Return to catalogue'}),
    page.getByRole('button',{name:'Export CSV'}),
    page.getByRole('link',{name:'maya@example.test'}),
    page.getByText('Please quote delivery to Bristol.'),
    page.getByRole('button',{name:'Print request / save PDF'}),
  ]){
    await locator.scrollIntoViewIfNeeded();
    const box=await locator.boundingBox();
    expect(box?.x).toBeGreaterThanOrEqual(0);
    expect((box?.x||0)+(box?.width||0)).toBeLessThanOrEqual(320);
  }
  const tableWrap=page.locator('.request-card .table-wrap').first();
  await tableWrap.focus();
  await expect(tableWrap).toBeFocused();
  const initialTableScroll=await tableWrap.evaluate(node=>node.scrollLeft);
  await page.keyboard.press('ArrowRight');
  await expect.poll(()=>tableWrap.evaluate(node=>node.scrollLeft)).toBeGreaterThan(initialTableScroll);
  await tableWrap.evaluate(node=>{node.scrollLeft=node.scrollWidth;});
  const wrapBox=await tableWrap.boundingBox();
  const priceHeaderBox=await page.getByRole('columnheader',{name:'Unit price'}).boundingBox();
  expect(priceHeaderBox?.x).toBeGreaterThanOrEqual(wrapBox?.x||0);
  expect((priceHeaderBox?.x||0)+(priceHeaderBox?.width||0)).toBeLessThanOrEqual((wrapBox?.x||0)+(wrapBox?.width||0)+1);

  // A real request carries the backend's long RFC3339 timestamp, unlike the short seeded sample.
  await page.goto('/demo');
  await page.getByRole('button',{name:'Add to request'}).first().click();
  await page.getByRole('button',{name:/Review request/}).click();
  await page.getByLabel('Your name').fill('Maya Patel');
  await page.getByLabel('Company').fill('Juniper Corner');
  await page.getByLabel('Email').fill('maya@example.test');
  await page.getByRole('button',{name:'Send quote request'}).click();
  await expect(page.getByRole('heading',{name:/Request RQ-DEMO-/})).toBeVisible();
  await page.goto('/demo/inbox');
  await page.evaluate(()=>document.documentElement.style.fontSize='200%');
  const timestamp=page.locator('.request-card time').first();
  await expect(timestamp).toHaveText(/^\d{4}-\d{2}-\d{2}T.+\+00:00$/);
  expectInboxToReflow(await inboxGeometry());
  const timestampGeometry=await timestamp.evaluate(node=>{
    const element=node as HTMLElement;
    const box=element.getBoundingClientRect();
    return {left:box.left,right:box.right,clientWidth:element.clientWidth,scrollWidth:element.scrollWidth,overflowWrap:getComputedStyle(element).overflowWrap};
  });
  expect(timestampGeometry.overflowWrap).toBe('anywhere');
  expect(timestampGeometry.left).toBeGreaterThanOrEqual(0);
  expect(timestampGeometry.right).toBeLessThanOrEqual(320);
  expect(timestampGeometry.scrollWidth).toBeLessThanOrEqual(timestampGeometry.clientWidth);
  await page.evaluate(()=>window.scrollTo({left:9999}));
  expect(await page.evaluate(()=>window.scrollX)).toBe(0);
});

test('public routes keep the document skeleton and load without console errors',async({page})=>{
  const errors:string[]=[];
  page.on('console',message=>{if(message.type()==='error')errors.push(message.text());});
  const routes=[
    {path:'/',title:'Client Catalogue Request — collect quote requests',canonical:'/'},
    {path:'/?demo=1',title:'Demo — Client Catalogue Request',canonical:'/demo'},
    {path:'/demo',title:'Demo — Client Catalogue Request',canonical:'/demo'},
    {path:'/demo/inbox',title:'Demo seller inbox — Client Catalogue Request',canonical:'/demo/inbox'},
    {path:'/privacy',title:'Privacy — Client Catalogue Request',canonical:'/privacy'},
    {path:'/terms',title:'Terms — Client Catalogue Request',canonical:'/terms'},
    {path:'/manage',title:'Seller workspace — Client Catalogue Request',canonical:'/manage'},
    {path:'/missing-page',title:'Page not found — Client Catalogue Request',canonical:'/missing-page'},
  ];
  for(const route of routes){
    const path=route.path;
    await page.goto(path);
    await expect(page.locator('main')).toHaveCount(1);
    await expect(page.locator('h1')).toHaveCount(1);
    await expect(page).toHaveTitle(route.title);
    await expect(page.locator('meta[name="description"]')).toHaveAttribute('content',/.{20,155}/);
    await expect(page.locator('link[rel="canonical"]')).toHaveAttribute('href',`https://client-catalogue-request.sociobot.in${route.canonical}`);
    await expect(page.locator('meta[property="og:title"]')).toHaveAttribute('content',route.title);
    await expect(page.locator('meta[name="twitter:title"]')).toHaveAttribute('content',route.title);
    expect(await page.locator('html').getAttribute('lang')).toBe('en');
    expect(await page.locator('img:not([alt])').count()).toBe(0);
    await expect(page.locator('footer').getByRole('link',{name:'Privacy'})).toBeVisible();
    await expect(page.locator('footer').getByRole('link',{name:'Terms'})).toBeVisible();
    const results=await new AxeBuilder({page:page as never}).withTags(['wcag2a','wcag2aa']).analyze();
    expect(results.violations.filter(v=>['serious','critical'].includes(v.impact||'')),path).toEqual([]);
  }
  const missingUrl=process.env.PLAYWRIGHT_BASE_URL?'/missing-page':'http://127.0.0.1:8080/missing-page';
  expect((await page.request.get(missingUrl)).status()).toBe(404);
  await page.goto('/missing-page');
  await page.getByRole('link',{name:'Return to the start'}).click();
  await expect(page.getByRole('heading',{name:'Turn repeat orders into clear requests'})).toBeFocused();
  expect(errors.filter(error=>!/^Failed to load resource: the server responded with a status of 404/.test(error))).toEqual([]);
});

test('@claim:print-request opens a print-ready request',async({page},testInfo)=>{
  test.skip(testInfo.project.name==='mobile','Print output is verified once; mobile catalogue behavior is covered separately.');
  await page.goto('/demo/inbox');
  await page.evaluate(()=>{(window as Window & {printed?:boolean}).printed=false;window.print=()=>{(window as Window & {printed?:boolean}).printed=true;};});
  await page.getByRole('button',{name:'Print request / save PDF'}).click();
  await expect(page.locator('.request-card.print-selected')).toHaveCount(1);
  expect(await page.evaluate(()=>(window as Window & {printed?:boolean}).printed)).toBe(true);
});

test('@claim:paid-license a verified license raises backend row and link limits',async({page},testInfo)=>{
  test.skip(testInfo.project.name==='mobile','The entitlement boundary is exercised once against the shared backend.');
  const seller=`paid-claim-${Date.now()}-${Math.random()}`;
  await page.addInitScript(({seller})=>{
    sessionStorage.setItem('ccr:session',`test-seller:${seller}`);
  },{seller});
  await page.route('https://api.sociobot.in/api/v1/products/client-catalogue-request/verify?license=test-license',route=>route.fulfill({json:{valid:true,reason:'ok',expires_at:null}}));
  await page.goto('/?license=test-license');
  await expect(page).toHaveURL('/');
  await expect(page.getByText('₹1,499')).toBeVisible();
  await expect(page.getByText('Use more than 12 catalogue rows and create more than one client link.')).toBeVisible();
  await expect(page.getByRole('link',{name:'Buy the full workspace'})).toHaveAttribute('href','https://api.sociobot.in/api/v1/products/client-catalogue-request/checkout');
  await expect.poll(()=>page.evaluate(()=>localStorage.getItem('sb_license:client-catalogue-request'))).toBe('test-license');
  await expect.poll(()=>page.evaluate(()=>JSON.parse(localStorage.getItem('sb_license_cache:client-catalogue-request')||'null')?.valid)).toBe(true);
  await page.goto('/manage');
  await expect(page.getByText('Full workspace active')).toBeVisible();
  const rows=['sku,name,description,category,price,stock_note'];
  for(let n=1;n<=13;n++)rows.push(`PAID-${n},Paid product ${n},Recorded fixture,Products,10.00,Ask`);
  await page.getByLabel('Choose a product CSV').setInputFiles({name:'paid-products.csv',mimeType:'text/csv',buffer:Buffer.from(rows.join('\n'))});
  await page.getByRole('button',{name:'Save catalogue'}).click();
  await expect(page.getByText('13 products')).toBeVisible();
  for(const [index,label] of ['Buyer one','Buyer two'].entries()){
    await page.getByLabel('Client or group name').fill(label);
    await page.getByRole('button',{name:'Create client link'}).click();
    await expect(page.getByRole('link',{name:/Open catalogue/})).toHaveCount(index+1);
  }
});

test('@claim:paid-license-invalid shows an accessible inactive-license notice',async({page})=>{
  await page.route('https://api.sociobot.in/api/v1/products/client-catalogue-request/verify?license=invalid-license',route=>route.fulfill({json:{valid:false,reason:'revoked',expires_at:null}}));
  await page.goto('/?license=invalid-license');
  await expect(page.getByRole('status')).toContainText('License no longer active');
  await expect(page.getByLabel('Have a license? Paste it')).toBeVisible();
});

test('@claim:privacy-runtime uses no third-party runtime assets',async({page})=>{
  const origins=new Set<string>();
  page.on('request',request=>origins.add(new URL(request.url()).origin));
  await page.goto('/');
  await page.goto('/demo');
  expect([...origins]).toEqual([new URL(page.url()).origin]);
});

test('@claim:csv-import @claim:client-data-control seller import to client request works end to end',async({page},testInfo)=>{
  test.skip(testInfo.project.name==='mobile','The same responsive UI is covered by mobile claim tests.');
  await page.addInitScript(()=>sessionStorage.setItem('ccr:session','test-seller:browser-e2e'));
  await page.goto('/manage');
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
  page.once('dialog',dialog=>dialog.accept());
  await page.getByRole('button',{name:'Delete request'}).click();
  await expect(page.getByText('alex@example.test')).toHaveCount(0);
});
