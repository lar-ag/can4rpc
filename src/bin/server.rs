#![feature(async_await)]

// use structopt::*;
use tide;
use wqm_can::{
  analog,
  digital,
  doppelmotor,
};

use runtime;
use failure::{Fallible};
use std::fmt::Write;
pub use tide::{error::ResultExt, response, App, Context, EndpointResult};
use http::status::StatusCode;
// use failure::{Fallible, ResultExt};







pub async fn can0_info(_cx: Context<()>) -> EndpointResult<String> {
    let mut info = String::new();
    write!(&mut info, "can0 info").unwrap(); // uses fmt::Write::write_fmt
    Ok(info)

}

#[runtime::main]
async fn main() {
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::{Appender, Config, Root};

    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
    // let repo = monitor::new_uv().await;
    let mut app = tide::App::new(());
    app.middleware(tide::middleware::RootLogger::new());
    app.at("/can0").nest(|api| {
        api.at("/").get(can0_info);
        api.at("/analog/:id").get(analog::info);
        api.at("/analog/:id/in01").get(analog::get_input01);
        api.at("/analog/:id/in02").get(analog::get_input02);
        api.at("/analog/:id/in03").get(analog::get_input03);
        api.at("/analog/:id/in04").get(analog::get_input04);
        api.at("/analog/:id/in05").get(analog::get_input05);
        api.at("/analog/:id/out").get(analog::get_out).post(analog::set_out);
        api.at("/analog/:id/temp01").get(analog::get_temp01);
        api.at("/analog/:id/temp02").get(analog::get_temp02);
        api.at("/analog/:id/temp03").get(analog::get_temp03);
        api.at("/analog/:id/uart01").get(analog::get_uart01).post(analog::set_uart01);
        api.at("/analog/:id/baut01").post(analog::setup_uart02);
        api.at("/analog/:id/uart02").get(analog::get_uart02).post(analog::set_uart02);
        api.at("/analog/:id/baut02").post(analog::setup_uart02);
        api.at("/digital/:id").get(digital::info);
        api.at("/digital/:id/input").get(digital::get_input);
        api.at("/digital/:id/output").get(digital::get_output).post(digital::set_output);
        api.at("/doppelmotor/:id/uart01").get(doppelmotor::get_uart01).post(doppelmotor::set_uart01);
        api.at("/doppelmotor/:id/baut01").post(doppelmotor::setup_uart02);
        api.at("/doppelmotor/:id/uart02").get(doppelmotor::get_uart02).post(doppelmotor::set_uart02);
        api.at("/doppelmotor/:id/baut02").post(doppelmotor::setup_uart02);
    //   api.at("/analog/:id/in01").get(analog::get_input01);
    });
    app.serve("127.0.0.1:8000").unwrap();
}

