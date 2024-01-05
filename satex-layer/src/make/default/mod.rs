use satex_core::export_make;

mod make;
mod registry;
mod set_discovery;
mod set_http_client;
mod trace;

export_make!(MakeDefaultRouteServiceLayer);
