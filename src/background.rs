use async_std::task;
use diesel::prelude::*;
use log::{error, info};
use std::time::Duration;

use crate::models::odoo_project::Pool;
use crate::modules::hubspot::hubspot_module;
use crate::modules::odoo::odoo_module;

pub async fn start_background(pool: Pool) {
	info!("Starting background processing.");

	let modules = std::env::var("MODULES").expect("MODULES must be set");
	let m: Vec<&str> = modules.split(",").collect();

	info!("Active modules: {}", &modules);

	let sleep = std::env::var("SLEEP").expect("SLEEP must be set");
	let seconds: u64 = sleep.parse().unwrap_or(60);

	info!("Sleep delay: {}", &seconds);

	loop {
		info!("Processing.");

		let conn: &PgConnection = &pool.get().unwrap();

		let a = m.iter();

		for s in a {
			let mut result: Result<(), String> = Ok(());

			match *s {
				"odoo" => {
					result = odoo_module::do_process(conn).await;
				}
				"hubspot" => {
					result = hubspot_module::do_process(conn).await;
				}
				_ => error!("Unknown module defined in MODULES variable!"),
			};

			match result {
				Ok(v) => v,
				Err(e) => {
					error!("Failure! {}", e);
				}
			}
		}

		task::sleep(Duration::from_secs(seconds)).await;
	}
}
