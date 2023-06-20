use std::collections::BTreeMap;
use k8s_openapi::{
    api::{
        apps::v1::{DaemonSet, DaemonSetSpec},
        core::v1::{
            Container, HostPathVolumeSource, PodSpec, PodTemplateSpec,
            SecurityContext, Volume, VolumeMount,
        },
    },
    apimachinery::pkg::apis::meta::v1::LabelSelector,
};
use kube::{core::ObjectMeta, CustomResource, Resource, ResourceExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "aaaaaaah.dev",
    version = "v1alpha1",
    kind = "NodeScript",
    namespaced
)]
pub struct NodeScriptSpec {
    pub script: String,
}

impl Into<DaemonSet> for &NodeScript {
    fn into(self) -> DaemonSet {
        let owner_references = self.controller_owner_ref(&()).unwrap();
        let name = self.metadata.name.clone().unwrap();
        let annotations = self.annotations();
        let labels = self.labels();
        let script = self.spec.script.clone();

        let match_labels = Some(BTreeMap::from([
            ("name".to_string(), name.clone()),
            ("nodescript".to_string(), "true".to_string()),
        ]));
        DaemonSet {
            metadata: ObjectMeta {
                name: Some(name),
                annotations: Some(annotations.clone()),
                labels: Some(labels.clone()),
                owner_references: Some(vec![owner_references.clone()]),
                ..Default::default()
            },
            spec: Some(DaemonSetSpec {
                selector: LabelSelector {
                    match_labels: match_labels.clone(),
                    ..Default::default()
                },
                template: PodTemplateSpec {
                    metadata: Some(ObjectMeta {
                        annotations: Some(annotations.clone()),
                        labels: match_labels.clone(),
                        ..Default::default()
                    }),
                    spec: Some(PodSpec {
                        containers: vec![Container {
                            command: Some(vec![
                                "/bin/ash".to_string(),
                                "-c".to_string(),
                                r#"chroot /host sh -c "$1" && sleep infinity;"#
                                    .to_string(),
                                "--".to_string(),
                                script,
                            ]),
                            security_context: Some(SecurityContext {
                                privileged: Some(true),
                                ..Default::default()
                            }),
                            image: Some("alpine:3.18".to_string()),
                            name: "nodescript".to_string(),
                            volume_mounts: Some(vec![VolumeMount {
                                mount_path: "/host".to_string(),
                                name: "host".to_string(),
                                ..Default::default()
                            }]),
                            ..Default::default()
                        }],
                        volumes: Some(vec![Volume {
                            host_path: Some(HostPathVolumeSource {
                                path: "/".to_string(),
                                type_: Some("Directory".to_string()),
                            }),
                            name: "host".to_string(),
                            ..Default::default()
                        }]),
                        host_ipc: Some(true),
                        host_network: Some(true),
                        host_pid: Some(true),
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
