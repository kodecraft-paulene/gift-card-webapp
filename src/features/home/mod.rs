use leptos::*;
use leptab::DataTable;
use leptab::model::{DownloadDataRequest, TableHeader};

/// Default Home Page
#[allow(non_snake_case)]
#[component]
pub fn Home() -> impl IntoView {

//     let selected_page = RwSignal::new("All".to_string());
//     let headers = RwSignal::new(vec![
//         TableHeader::new("time", "expiry_timestamp", "Expiry Date", false, "", false, "", "", "", true, None),
//         TableHeader::new("date_created", "date_created", "Date Created", false, "", false, "", "", "", true, None),
//         TableHeader::new("market", "venue_instrument_name", "Market", false, "", false, "", "", "", true, None),
//         TableHeader::new("party_a",  "party_a", "Party A", false, "", false, "", "", "", true, None),
//         TableHeader::new("party_b",  "party_b.ticker", "Party B", false, "", false, "", "", "", true, None),
//         TableHeader::new("side_status", "side", "Side", false, "", false, "", "buy", "sell", true, None),
//         TableHeader::new("trans_type", "instrument_kind", "Type", false, "", false, "- -", "", "", true, None),
//         TableHeader::new("trade_type", "trade_type", "Trade Type", false, "", false, "", "", "", true, None),
//         TableHeader::new("kind", "option_kind", "Kind", false, "", false, "", "", "", true, None),
//         TableHeader::new("size", "amount", "Size", false, "", true, "- -", "", "", false, None),
//         TableHeader::new("price", "px_in_quote_ccy", "Price", true, "premium_ccy", true, "- -", "", "", true, None),
//         TableHeader::new("index_price", "index_price", "Index Price", false, "", false, "- -", "", "", true, None),
//         TableHeader::new("realized_pnl", "pnl", "Realized PnL", true, "realized_pnl_ccy", true, "- -", "", "", true, None),
//         TableHeader::new("trade_status", "trade_status", "Status", false, "", false, "", "", "", true, None),
//     ]);
//     let sort = RwSignal::new(false);
//     let sort_by = RwSignal::new("date_created".to_string());
//     let offset = RwSignal::new(0u32);
//     let limit = RwSignal::new(25u32);
//     let search = RwSignal::new("".to_string());

//     let filter = move || {
//         match selected_page.get().as_str() {
//             "All" => "filter[counterparty_id][ticker][_eq]=JABRA&filter[party_a][_neq]=null&filter[party_b][_neq]=null".to_string(),
//             "Option" => "filter[counterparty_id][ticker][_eq]=JABRA&filter[party_a][_neq]=null&filter[party_b][_neq]=null&filter[instrument_kind][_eq]=Option".to_string(),
//             "Spot" => "filter[counterparty_id][ticker][_eq]=JABRA&filter[party_a][_neq]=null&filter[party_b][_neq]=null&filter[instrument_kind][_eq]=Spot".to_string(),
//             "Perpetual Futures" => "filter[counterparty_id][ticker][_eq]=JABRA&filter[party_a][_neq]=null&filter[party_b][_neq]=null&filter[instrument_kind][_eq]=Futures".to_string(),
//             _ => "filter[counterparty_id][ticker][_eq]=JABRA&filter[party_a][_neq]=null&filter[party_b][_neq]=null".to_string(),
//         }
//     };
//     let fields = RwSignal::new(Trade::get_query());
//     let data_count_resource = create_local_resource(move || (search.get()), move |a| get_collection_count("trade".to_string(), filter(), Some(a)));
//     let data_resource = create_local_resource(move || ("trade".to_string(),filter(), fields.get(), offset.get(), limit.get(), search.get(), sort.get(), sort_by.get()),
//         move |(a,b,c,d,e,f ,g, h)| get_collection::<TradeHistory>(a, b, c, Some(d), Some(e), Some(f), Some(g), Some(h)));
//     let download_data_request = Signal::derive(move || {
//         DownloadDataRequest {
//             table_name: "trade".to_string(),
//             filter: filter(),
//             fields: fields.get(),
//             search: search.get(),
//         }
//     });
//     let download_resource: Resource<DownloadDataRequest, Result<String, ServerFnError>> = create_local_resource(move || (download_data_request.get()), move |d| get_collection_file::<TradeHistory>(d));

//     let data_count = RwSignal::new(0u32);
//     let extracted_data = RwSignal::new(Vec::<ExtractedTrade>::default());
//     let json_data = Signal::derive(move || {
//         extracted_data
//             .get()
//             .into_iter()
//             .map(serde_json::to_value)
//             .collect::<Result<Vec<serde_json::Value>, _>>()
//             .expect("Failed to serialize to JSON")
//     });
//     let current_page = RwSignal::new(1u32);
//     let allow_download = RwSignal::new(true);
//     let download_filename = RwSignal::new("trade".to_string());

  view! {   
//     <p>"Home Page"</p>
//     <div class="p-4">
//     <div class="pb-5 ml-2 text-xl font-bold text-white">
//         <span>Trade History</span>
//     </div>
//     <Transition fallback=move || {
//         view! {
//             <div class="items-center mt-5">
//                 <div class="flex justify-center ">
//                     <Spinners size=ComponentSize::SMALL _type=ComponentType::SUCCESS />
//                 </div>
//             </div>
//         }
//     }>  
//         <div class = "flex flex-col">
//             <div class="flex justify-start gap-4 flex-0 ">
//                 {
//                     move || {
                    
//                     let page_keys = vec![
//                         String::from("All"),
//                         String::from("Option"),
//                         String::from("Spot"),
//                         String::from("Perpetual Futures"),
//                     ];
//                     page_keys
//                         .into_iter()
//                         .map(|k| {
//                             view! {
//                                 <MenuButtonAsFilter
//                                     selected_page=selected_page
//                                     page=k.clone()
//                                     name=k.clone()
//                                     limit=limit
//                                     offset=offset
//                                     current_page=current_page
//                                     default_limit=limit.get_untracked()
//                                 />
//                             }
//                         })
//                         .collect_view()
//                     }
//                 }
//             </div>
//             {
//                 move || {
//                     data_count_resource.and_then(|c| {
//                         data_count.set(*c);
//                         data_resource.and_then(|d| {
//                             extracted_data.set(d.extract());
//                             view! {
//                                 <DataTable 
//                                     headers = headers 
//                                     data = json_data 
//                                     offset = offset 
//                                     search = search 
//                                     sort = sort 
//                                     sort_by = sort_by 
//                                     limit = limit 
//                                     total = data_count 
//                                     current_page = current_page
//                                     allow_download = allow_download
//                                     download_filename = download_filename
//                                     download_resource = download_resource
//                                 />
//                             }
//                         })
//                     })
//                 }
//             }
//         </div>
//     </Transition>
// </div>
  }
}
