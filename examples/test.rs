extern crate futures;
extern crate golem_gw_api;
extern crate hyper;
extern crate tokio_core;
extern crate url;

use futures::Future;
use tokio_core::reactor::Core;

use golem_gw_api::models::SubtaskResult;
use hyper::client::Client;

fn main() {
    let mut event_loop = Core::new().unwrap();
    let client = Client::new();

    let config = golem_gw_api::apis::configuration::Configuration::new(client);
    let client = golem_gw_api::apis::client::APIClient::new(config);
    let subscription =
        golem_gw_api::models::Subscription::new(1.0, 5, 1000666, 1000000).with_performance(12.0);

    // Subscribe
    let work = client
        .default_api()
        .subscribe("1", "Blender", subscription)
        .and_then(|res| {
            println!("Response: {:?}", res);
            Ok(())
        });

    // All subscriptions
    //    let work = client.default_api().all_subscriptions("1");

    // Subscription
    //  let work = client.default_api().subscription("1", "Blender");

    // Remove subscription
    //    let work = client.default_api().unsubscribe("1", "Blender");

    // Confirm subtask
    //    let work = client.default_api().confirm_subtask("1", "1");

    // Cancel subtask
    //    let work = client.default_api().cancel_subtask("1", "1");

    // Report result of subtask
    //    let work = client.default_api().subtask_result("1", "1", SubtaskResult::new("succeded".into(), "path".into()));

    // Subtask info
    //    let work = client.default_api().subtask_info("1", "1");

    // Task info
    //    let work = client.default_api().task_info("1", "1");

    // info
    //    let work = client.default_api().fetch_events("1", "Blender", 10);

    // Computation willingness
    //    let work = client.default_api().want_to_compute_task("1", "1");

    let user = event_loop.run(work);

    println!("\nEvent loop result:\n {:?}", user);
}
