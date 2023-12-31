pub mod crd;

use std::{sync::Arc, time::Duration};

use crd::NodeScript;
use futures::StreamExt;
use k8s_openapi::api::apps::v1::DaemonSet;
use kube::{
    api::{Patch, PatchParams},
    runtime::{controller::Action, watcher, Controller},
    Api, Client, CustomResourceExt, Resource, ResourceExt, Error,
};
use log::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();

    // Generate CRD
    if let Some("crd") = std::env::args().nth(1).as_deref() {
        let crd = NodeScript::crd();
        print!("{}", serde_yaml::to_string(&crd).unwrap());
        return Ok(());
    }

    let client = Client::try_default().await?;
    let nodescript_api = Api::<NodeScript>::all(client.clone());
    let daemonset_api = Api::<DaemonSet>::all(client.clone());

    let context = Arc::new(client);

    info!("Starting Controller...");
    Controller::new(nodescript_api, watcher::Config::default())
        .owns(daemonset_api, watcher::Config::default())
        .shutdown_on_signal()
        .run(reconcile, error_policy, context)
        .for_each(|res| async move {
            match res {
                Ok(o) => info!("reconciled {:?}", o),
                Err(e) => warn!("reconcile failed: {:?}", e),
            }
        })
        .await;

    Ok(())
}

fn error_policy(
    _obj: Arc<NodeScript>,
    _error: &Error,
    _ctx: Arc<Client>,
) -> Action {
    Action::requeue(Duration::from_secs(1))
}

async fn reconcile(
    obj: Arc<NodeScript>,
    client: Arc<Client>,
) -> Result<Action, Error> {
    let client = client.as_ref();
    info!("reconciling {:?}", obj);

    if let Some(_) = obj.meta().deletion_timestamp {
        info!("deleting {:?}", obj);
        // Kubernetes will delete the DaemonSet for us as we defined async
        // owner reference. We're done!
        return Ok(Action::await_change());
    }

    let ns = obj.namespace().unwrap();
    let daemonset_api = Api::<DaemonSet>::namespaced(client.clone(), &ns);
    let daemonset: DaemonSet = obj.as_ref().into();
    let name = daemonset.meta().name.clone().unwrap();
    daemonset_api
        .patch(
            &name,
            &PatchParams::apply("nodescript-operator.aaaaaaaah.dev"),
            &Patch::Apply(&daemonset),
        )
        .await?;

    Ok(Action::await_change())
}
