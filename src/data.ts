export type Product = { id:string; sku:string; name:string; description:string; category:string; price_cents:number|null; stock_note:string };
export type Settings = { business_name:string; price_label:string; tax_note:string; currency:string };
export type Catalogue = { settings:Settings; products:Product[]; links?:ClientLink[] };
export type ClientLink = { token:string; label:string; active:boolean };
export type RequestLine = { product_id:string; sku?:string; name?:string; quantity:number; price_cents?:number|null };
export type QuoteRequest = { id:string; client_name:string; company:string; email:string; po_number:string; note:string; status:string; created_at:string; lines:RequestLine[] };

export const sampleCatalogue: Catalogue = {
  settings: { business_name:'Northline Supply Co.', price_label:'Trade price', tax_note:'Prices exclude VAT', currency:'GBP' },
  products: [
    {id:'p1',sku:'NW-101',name:'Recycled counter notebook',description:'A5 dot-grid book with 160 recycled pages.',category:'Desk',price_cents:850,stock_note:'In stock'},
    {id:'p2',sku:'NW-114',name:'Brass desk ruler',description:'30 cm ruler with etched metric marks.',category:'Desk',price_cents:1250,stock_note:'Low stock — ask for lead time'},
    {id:'p3',sku:'PK-220',name:'Kraft dispatch box',description:'Self-locking box, pack of 25.',category:'Packing',price_cents:1890,stock_note:'In stock'},
    {id:'p4',sku:'PK-228',name:'Custom paper tape',description:'One-colour print on reinforced paper tape.',category:'Packing',price_cents:null,stock_note:'Made to order — allow 3 weeks'},
    {id:'p5',sku:'SV-410',name:'Shelf label set',description:'Water-resistant labels, set of 80.',category:'Shop floor',price_cents:2400,stock_note:'Preorder for October delivery'},
    {id:'p6',sku:'SV-421',name:'Oak display riser',description:'Oil-finished oak, 30 × 20 cm.',category:'Shop floor',price_cents:null,stock_note:'Ask about availability'}
  ],
  links: [{token:'demo',label:'Sample client',active:true}]
};

export const sampleRequests: QuoteRequest[] = [{
  id:'RQ-6C24A19E',client_name:'Maya Patel',company:'Juniper Corner',email:'maya@example.test',po_number:'PO-1842',note:'Please quote delivery to Bristol.',status:'New',created_at:'2026-08-28 09:14',
  lines:[{product_id:'p1',sku:'NW-101',name:'Recycled counter notebook',quantity:24,price_cents:850},{product_id:'p4',sku:'PK-228',name:'Custom paper tape',quantity:12,price_cents:null}]
}];
