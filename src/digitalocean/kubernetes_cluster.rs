#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct KubernetesClusterData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_upgrade: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_subnet: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destroy_all_associated_resources: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ha: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    isolated_workers: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubeconfig_expire_seconds: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_integration: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_subnet: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    surge_upgrade: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_subnet_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amd_gpu_device_metrics_exporter_plugin: std
    ::option
    ::Option<std::vec::Vec<KubernetesCluster_AmdGpuDeviceMetricsExporterPlugin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amd_gpu_device_plugin: std::option::Option<std::vec::Vec<KubernetesCluster_AmdGpuDevicePlugin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amd_gpu_dra_driver: std::option::Option<std::vec::Vec<KubernetesCluster_AmdGpuDraDriver>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_autoscaler_configuration: std
    ::option
    ::Option<std::vec::Vec<KubernetesCluster_ClusterAutoscalerConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_firewall: std::option::Option<std::vec::Vec<KubernetesCluster_ControlPlaneFirewall>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coredns_autoscaler: std::option::Option<std::vec::Vec<KubernetesCluster_CorednsAutoscaler>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_policy: std::option::Option<std::vec::Vec<KubernetesCluster_MaintenancePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs_csi_plugin: std::option::Option<std::vec::Vec<KubernetesCluster_NfsCsiPlugin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool: std::option::Option<std::vec::Vec<KubernetesCluster_NodePool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nvidia_gpu_device_plugin: std::option::Option<std::vec::Vec<KubernetesCluster_NvidiaGpuDevicePlugin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nvidia_gpu_dra_driver: std::option::Option<std::vec::Vec<KubernetesCluster_NvidiaGpuDraDriver>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    p2p_oci_registry_plugin: std::option::Option<std::vec::Vec<KubernetesCluster_P2pOciRegistryPlugin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdma_shared_device_plugin: std::option::Option<std::vec::Vec<KubernetesCluster_RdmaSharedDevicePlugin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_agent: std::option::Option<std::vec::Vec<KubernetesCluster_RoutingAgent>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sso: std::option::Option<std::vec::Vec<KubernetesCluster_Sso>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<KubernetesCluster_Timeouts>,
    dynamic: KubernetesClusterDynamic,
}

struct KubernetesCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KubernetesClusterData_>,
}

#[derive(Clone)]
pub struct KubernetesCluster(std::rc::Rc<KubernetesCluster_>);

impl KubernetesCluster {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderDigitalocean) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc = "Set the field `auto_upgrade`.\n"]
    pub fn set_auto_upgrade(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_upgrade = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_subnet`.\n"]
    pub fn set_cluster_subnet(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cluster_subnet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destroy_all_associated_resources`.\n"]
    pub fn set_destroy_all_associated_resources(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().destroy_all_associated_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ha`.\n"]
    pub fn set_ha(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ha = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `isolated_workers`.\n"]
    pub fn set_isolated_workers(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().isolated_workers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kubeconfig_expire_seconds`.\n"]
    pub fn set_kubeconfig_expire_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().kubeconfig_expire_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry_integration`.\n"]
    pub fn set_registry_integration(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().registry_integration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_subnet`.\n"]
    pub fn set_service_subnet(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_subnet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `surge_upgrade`.\n"]
    pub fn set_surge_upgrade(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().surge_upgrade = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_uuid`.\n"]
    pub fn set_vpc_uuid(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker_subnet_uuid`.\n"]
    pub fn set_worker_subnet_uuid(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().worker_subnet_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `amd_gpu_device_metrics_exporter_plugin`.\n"]
    pub fn set_amd_gpu_device_metrics_exporter_plugin(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_AmdGpuDeviceMetricsExporterPlugin>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().amd_gpu_device_metrics_exporter_plugin = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.amd_gpu_device_metrics_exporter_plugin =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `amd_gpu_device_plugin`.\n"]
    pub fn set_amd_gpu_device_plugin(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_AmdGpuDevicePlugin>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().amd_gpu_device_plugin = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.amd_gpu_device_plugin = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `amd_gpu_dra_driver`.\n"]
    pub fn set_amd_gpu_dra_driver(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_AmdGpuDraDriver>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().amd_gpu_dra_driver = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.amd_gpu_dra_driver = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cluster_autoscaler_configuration`.\n"]
    pub fn set_cluster_autoscaler_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_ClusterAutoscalerConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cluster_autoscaler_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cluster_autoscaler_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `control_plane_firewall`.\n"]
    pub fn set_control_plane_firewall(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_ControlPlaneFirewall>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().control_plane_firewall = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.control_plane_firewall = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `coredns_autoscaler`.\n"]
    pub fn set_coredns_autoscaler(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_CorednsAutoscaler>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().coredns_autoscaler = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.coredns_autoscaler = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_MaintenancePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `nfs_csi_plugin`.\n"]
    pub fn set_nfs_csi_plugin(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_NfsCsiPlugin>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().nfs_csi_plugin = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.nfs_csi_plugin = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_pool`.\n"]
    pub fn set_node_pool(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_NodePool>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_pool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_pool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `nvidia_gpu_device_plugin`.\n"]
    pub fn set_nvidia_gpu_device_plugin(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_NvidiaGpuDevicePlugin>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().nvidia_gpu_device_plugin = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.nvidia_gpu_device_plugin = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `nvidia_gpu_dra_driver`.\n"]
    pub fn set_nvidia_gpu_dra_driver(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_NvidiaGpuDraDriver>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().nvidia_gpu_dra_driver = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.nvidia_gpu_dra_driver = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `p2p_oci_registry_plugin`.\n"]
    pub fn set_p2p_oci_registry_plugin(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_P2pOciRegistryPlugin>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().p2p_oci_registry_plugin = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.p2p_oci_registry_plugin = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rdma_shared_device_plugin`.\n"]
    pub fn set_rdma_shared_device_plugin(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_RdmaSharedDevicePlugin>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rdma_shared_device_plugin = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rdma_shared_device_plugin = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `routing_agent`.\n"]
    pub fn set_routing_agent(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_RoutingAgent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().routing_agent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.routing_agent = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sso`.\n"]
    pub fn set_sso(self, v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_Sso>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sso = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sso = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<KubernetesCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `auto_upgrade` after provisioning.\n"]
    pub fn auto_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_subnet` after provisioning.\n"]
    pub fn cluster_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_subnet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destroy_all_associated_resources` after provisioning.\n"]
    pub fn destroy_all_associated_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.destroy_all_associated_resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ha` after provisioning.\n"]
    pub fn ha(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ha", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv4_address` after provisioning.\n"]
    pub fn ipv4_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `isolated_workers` after provisioning.\n"]
    pub fn isolated_workers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.isolated_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kube_config` after provisioning.\n"]
    pub fn kube_config(&self) -> terrars::ListRef<KubernetesCluster_KubeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubeconfig_expire_seconds` after provisioning.\n"]
    pub fn kubeconfig_expire_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kubeconfig_expire_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_integration` after provisioning.\n"]
    pub fn registry_integration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_integration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_subnet` after provisioning.\n"]
    pub fn service_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_subnet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `surge_upgrade` after provisioning.\n"]
    pub fn surge_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.surge_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `urn` after provisioning.\n"]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_uuid` after provisioning.\n"]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_subnet_uuid` after provisioning.\n"]
    pub fn worker_subnet_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_subnet_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amd_gpu_device_metrics_exporter_plugin` after provisioning.\n"]
    pub fn amd_gpu_device_metrics_exporter_plugin(
        &self,
    ) -> terrars::ListRef<KubernetesCluster_AmdGpuDeviceMetricsExporterPluginRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.amd_gpu_device_metrics_exporter_plugin", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `amd_gpu_device_plugin` after provisioning.\n"]
    pub fn amd_gpu_device_plugin(&self) -> terrars::ListRef<KubernetesCluster_AmdGpuDevicePluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amd_gpu_device_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amd_gpu_dra_driver` after provisioning.\n"]
    pub fn amd_gpu_dra_driver(&self) -> terrars::ListRef<KubernetesCluster_AmdGpuDraDriverRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amd_gpu_dra_driver", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_autoscaler_configuration` after provisioning.\n"]
    pub fn cluster_autoscaler_configuration(
        &self,
    ) -> terrars::ListRef<KubernetesCluster_ClusterAutoscalerConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_autoscaler_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `control_plane_firewall` after provisioning.\n"]
    pub fn control_plane_firewall(&self) -> terrars::ListRef<KubernetesCluster_ControlPlaneFirewallRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_firewall", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `coredns_autoscaler` after provisioning.\n"]
    pub fn coredns_autoscaler(&self) -> terrars::ListRef<KubernetesCluster_CorednsAutoscalerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.coredns_autoscaler", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<KubernetesCluster_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nfs_csi_plugin` after provisioning.\n"]
    pub fn nfs_csi_plugin(&self) -> terrars::ListRef<KubernetesCluster_NfsCsiPluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_csi_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool` after provisioning.\n"]
    pub fn node_pool(&self) -> terrars::ListRef<KubernetesCluster_NodePoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nvidia_gpu_device_plugin` after provisioning.\n"]
    pub fn nvidia_gpu_device_plugin(&self) -> terrars::ListRef<KubernetesCluster_NvidiaGpuDevicePluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nvidia_gpu_device_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nvidia_gpu_dra_driver` after provisioning.\n"]
    pub fn nvidia_gpu_dra_driver(&self) -> terrars::ListRef<KubernetesCluster_NvidiaGpuDraDriverRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nvidia_gpu_dra_driver", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `p2p_oci_registry_plugin` after provisioning.\n"]
    pub fn p2p_oci_registry_plugin(&self) -> terrars::ListRef<KubernetesCluster_P2pOciRegistryPluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.p2p_oci_registry_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rdma_shared_device_plugin` after provisioning.\n"]
    pub fn rdma_shared_device_plugin(&self) -> terrars::ListRef<KubernetesCluster_RdmaSharedDevicePluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rdma_shared_device_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_agent` after provisioning.\n"]
    pub fn routing_agent(&self) -> terrars::ListRef<KubernetesCluster_RoutingAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sso` after provisioning.\n"]
    pub fn sso(&self) -> terrars::ListRef<KubernetesCluster_SsoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sso", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KubernetesCluster_TimeoutsRef {
        KubernetesCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for KubernetesCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for KubernetesCluster { }

impl terrars::ToListMappable for KubernetesCluster {
    type O = terrars::ListRef<KubernetesClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for KubernetesCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_kubernetes_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKubernetesCluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildKubernetesCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> KubernetesCluster {
        let out = KubernetesCluster(std::rc::Rc::new(KubernetesCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(KubernetesClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                auto_upgrade: core::default::Default::default(),
                cluster_subnet: core::default::Default::default(),
                destroy_all_associated_resources: core::default::Default::default(),
                ha: core::default::Default::default(),
                id: core::default::Default::default(),
                isolated_workers: core::default::Default::default(),
                kubeconfig_expire_seconds: core::default::Default::default(),
                name: self.name,
                region: self.region,
                registry_integration: core::default::Default::default(),
                service_subnet: core::default::Default::default(),
                surge_upgrade: core::default::Default::default(),
                tags: core::default::Default::default(),
                version: self.version,
                vpc_uuid: core::default::Default::default(),
                worker_subnet_uuid: core::default::Default::default(),
                amd_gpu_device_metrics_exporter_plugin: core::default::Default::default(),
                amd_gpu_device_plugin: core::default::Default::default(),
                amd_gpu_dra_driver: core::default::Default::default(),
                cluster_autoscaler_configuration: core::default::Default::default(),
                control_plane_firewall: core::default::Default::default(),
                coredns_autoscaler: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                nfs_csi_plugin: core::default::Default::default(),
                node_pool: core::default::Default::default(),
                nvidia_gpu_device_plugin: core::default::Default::default(),
                nvidia_gpu_dra_driver: core::default::Default::default(),
                p2p_oci_registry_plugin: core::default::Default::default(),
                rdma_shared_device_plugin: core::default::Default::default(),
                routing_agent: core::default::Default::default(),
                sso: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KubernetesClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl KubernetesClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_upgrade` after provisioning.\n"]
    pub fn auto_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_subnet` after provisioning.\n"]
    pub fn cluster_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_subnet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destroy_all_associated_resources` after provisioning.\n"]
    pub fn destroy_all_associated_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.destroy_all_associated_resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ha` after provisioning.\n"]
    pub fn ha(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ha", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv4_address` after provisioning.\n"]
    pub fn ipv4_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `isolated_workers` after provisioning.\n"]
    pub fn isolated_workers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.isolated_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kube_config` after provisioning.\n"]
    pub fn kube_config(&self) -> terrars::ListRef<KubernetesCluster_KubeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubeconfig_expire_seconds` after provisioning.\n"]
    pub fn kubeconfig_expire_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kubeconfig_expire_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_integration` after provisioning.\n"]
    pub fn registry_integration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_integration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_subnet` after provisioning.\n"]
    pub fn service_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_subnet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `surge_upgrade` after provisioning.\n"]
    pub fn surge_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.surge_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `urn` after provisioning.\n"]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_uuid` after provisioning.\n"]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `worker_subnet_uuid` after provisioning.\n"]
    pub fn worker_subnet_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_subnet_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amd_gpu_device_metrics_exporter_plugin` after provisioning.\n"]
    pub fn amd_gpu_device_metrics_exporter_plugin(
        &self,
    ) -> terrars::ListRef<KubernetesCluster_AmdGpuDeviceMetricsExporterPluginRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.amd_gpu_device_metrics_exporter_plugin", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `amd_gpu_device_plugin` after provisioning.\n"]
    pub fn amd_gpu_device_plugin(&self) -> terrars::ListRef<KubernetesCluster_AmdGpuDevicePluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amd_gpu_device_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amd_gpu_dra_driver` after provisioning.\n"]
    pub fn amd_gpu_dra_driver(&self) -> terrars::ListRef<KubernetesCluster_AmdGpuDraDriverRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amd_gpu_dra_driver", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_autoscaler_configuration` after provisioning.\n"]
    pub fn cluster_autoscaler_configuration(
        &self,
    ) -> terrars::ListRef<KubernetesCluster_ClusterAutoscalerConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_autoscaler_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `control_plane_firewall` after provisioning.\n"]
    pub fn control_plane_firewall(&self) -> terrars::ListRef<KubernetesCluster_ControlPlaneFirewallRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_firewall", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `coredns_autoscaler` after provisioning.\n"]
    pub fn coredns_autoscaler(&self) -> terrars::ListRef<KubernetesCluster_CorednsAutoscalerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.coredns_autoscaler", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<KubernetesCluster_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nfs_csi_plugin` after provisioning.\n"]
    pub fn nfs_csi_plugin(&self) -> terrars::ListRef<KubernetesCluster_NfsCsiPluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_csi_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool` after provisioning.\n"]
    pub fn node_pool(&self) -> terrars::ListRef<KubernetesCluster_NodePoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nvidia_gpu_device_plugin` after provisioning.\n"]
    pub fn nvidia_gpu_device_plugin(&self) -> terrars::ListRef<KubernetesCluster_NvidiaGpuDevicePluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nvidia_gpu_device_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nvidia_gpu_dra_driver` after provisioning.\n"]
    pub fn nvidia_gpu_dra_driver(&self) -> terrars::ListRef<KubernetesCluster_NvidiaGpuDraDriverRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nvidia_gpu_dra_driver", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `p2p_oci_registry_plugin` after provisioning.\n"]
    pub fn p2p_oci_registry_plugin(&self) -> terrars::ListRef<KubernetesCluster_P2pOciRegistryPluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.p2p_oci_registry_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rdma_shared_device_plugin` after provisioning.\n"]
    pub fn rdma_shared_device_plugin(&self) -> terrars::ListRef<KubernetesCluster_RdmaSharedDevicePluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rdma_shared_device_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_agent` after provisioning.\n"]
    pub fn routing_agent(&self) -> terrars::ListRef<KubernetesCluster_RoutingAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sso` after provisioning.\n"]
    pub fn sso(&self) -> terrars::ListRef<KubernetesCluster_SsoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sso", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KubernetesCluster_TimeoutsRef {
        KubernetesCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_KubeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expires_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KubernetesCluster_KubeConfig {
    #[doc = "Set the field `client_certificate`.\n"]
    pub fn set_client_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_key`.\n"]
    pub fn set_client_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_ca_certificate`.\n"]
    pub fn set_cluster_ca_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expires_at`.\n"]
    pub fn set_expires_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expires_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `raw_config`.\n"]
    pub fn set_raw_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.raw_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KubernetesCluster_KubeConfig {
    type O = terrars::BlockAssignable<KubernetesCluster_KubeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_KubeConfig {}

impl BuildKubernetesCluster_KubeConfig {
    pub fn build(self) -> KubernetesCluster_KubeConfig {
        KubernetesCluster_KubeConfig {
            client_certificate: core::default::Default::default(),
            client_key: core::default::Default::default(),
            cluster_ca_certificate: core::default::Default::default(),
            expires_at: core::default::Default::default(),
            host: core::default::Default::default(),
            raw_config: core::default::Default::default(),
            token: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_KubeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_KubeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_KubeConfigRef {
        KubernetesCluster_KubeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_KubeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\n"]
    pub fn client_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `client_key` after provisioning.\n"]
    pub fn client_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_ca_certificate` after provisioning.\n"]
    pub fn cluster_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_ca_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `expires_at` after provisioning.\n"]
    pub fn expires_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `raw_config` after provisioning.\n"]
    pub fn raw_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raw_config", self.base))
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_AmdGpuDeviceMetricsExporterPlugin {
    enabled: terrars::PrimField<bool>,
}

impl KubernetesCluster_AmdGpuDeviceMetricsExporterPlugin { }

impl terrars::ToListMappable for KubernetesCluster_AmdGpuDeviceMetricsExporterPlugin {
    type O = terrars::BlockAssignable<KubernetesCluster_AmdGpuDeviceMetricsExporterPlugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_AmdGpuDeviceMetricsExporterPlugin {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildKubernetesCluster_AmdGpuDeviceMetricsExporterPlugin {
    pub fn build(self) -> KubernetesCluster_AmdGpuDeviceMetricsExporterPlugin {
        KubernetesCluster_AmdGpuDeviceMetricsExporterPlugin { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_AmdGpuDeviceMetricsExporterPluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_AmdGpuDeviceMetricsExporterPluginRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KubernetesCluster_AmdGpuDeviceMetricsExporterPluginRef {
        KubernetesCluster_AmdGpuDeviceMetricsExporterPluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_AmdGpuDeviceMetricsExporterPluginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_AmdGpuDevicePlugin {
    enabled: terrars::PrimField<bool>,
}

impl KubernetesCluster_AmdGpuDevicePlugin { }

impl terrars::ToListMappable for KubernetesCluster_AmdGpuDevicePlugin {
    type O = terrars::BlockAssignable<KubernetesCluster_AmdGpuDevicePlugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_AmdGpuDevicePlugin {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildKubernetesCluster_AmdGpuDevicePlugin {
    pub fn build(self) -> KubernetesCluster_AmdGpuDevicePlugin {
        KubernetesCluster_AmdGpuDevicePlugin { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_AmdGpuDevicePluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_AmdGpuDevicePluginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_AmdGpuDevicePluginRef {
        KubernetesCluster_AmdGpuDevicePluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_AmdGpuDevicePluginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_AmdGpuDraDriver {
    enabled: terrars::PrimField<bool>,
}

impl KubernetesCluster_AmdGpuDraDriver { }

impl terrars::ToListMappable for KubernetesCluster_AmdGpuDraDriver {
    type O = terrars::BlockAssignable<KubernetesCluster_AmdGpuDraDriver>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_AmdGpuDraDriver {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildKubernetesCluster_AmdGpuDraDriver {
    pub fn build(self) -> KubernetesCluster_AmdGpuDraDriver {
        KubernetesCluster_AmdGpuDraDriver { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_AmdGpuDraDriverRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_AmdGpuDraDriverRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_AmdGpuDraDriverRef {
        KubernetesCluster_AmdGpuDraDriverRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_AmdGpuDraDriverRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_ClusterAutoscalerConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expanders: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_down_unneeded_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_down_utilization_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl KubernetesCluster_ClusterAutoscalerConfiguration {
    #[doc = "Set the field `expanders`.\n"]
    pub fn set_expanders(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expanders = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_down_unneeded_time`.\n"]
    pub fn set_scale_down_unneeded_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scale_down_unneeded_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_down_utilization_threshold`.\n"]
    pub fn set_scale_down_utilization_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_down_utilization_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KubernetesCluster_ClusterAutoscalerConfiguration {
    type O = terrars::BlockAssignable<KubernetesCluster_ClusterAutoscalerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_ClusterAutoscalerConfiguration {}

impl BuildKubernetesCluster_ClusterAutoscalerConfiguration {
    pub fn build(self) -> KubernetesCluster_ClusterAutoscalerConfiguration {
        KubernetesCluster_ClusterAutoscalerConfiguration {
            expanders: core::default::Default::default(),
            scale_down_unneeded_time: core::default::Default::default(),
            scale_down_utilization_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_ClusterAutoscalerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_ClusterAutoscalerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KubernetesCluster_ClusterAutoscalerConfigurationRef {
        KubernetesCluster_ClusterAutoscalerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_ClusterAutoscalerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expanders` after provisioning.\n"]
    pub fn expanders(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expanders", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_down_unneeded_time` after provisioning.\n"]
    pub fn scale_down_unneeded_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_down_unneeded_time", self.base))
    }

    #[doc = "Get a reference to the value of field `scale_down_utilization_threshold` after provisioning.\n"]
    pub fn scale_down_utilization_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_down_utilization_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_ControlPlaneFirewall {
    allowed_addresses: terrars::ListField<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
}

impl KubernetesCluster_ControlPlaneFirewall { }

impl terrars::ToListMappable for KubernetesCluster_ControlPlaneFirewall {
    type O = terrars::BlockAssignable<KubernetesCluster_ControlPlaneFirewall>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_ControlPlaneFirewall {
    #[doc = ""]
    pub allowed_addresses: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildKubernetesCluster_ControlPlaneFirewall {
    pub fn build(self) -> KubernetesCluster_ControlPlaneFirewall {
        KubernetesCluster_ControlPlaneFirewall {
            allowed_addresses: self.allowed_addresses,
            enabled: self.enabled,
        }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_ControlPlaneFirewallRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_ControlPlaneFirewallRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_ControlPlaneFirewallRef {
        KubernetesCluster_ControlPlaneFirewallRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_ControlPlaneFirewallRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_addresses` after provisioning.\n"]
    pub fn allowed_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_CorednsAutoscaler {
    enabled: terrars::PrimField<bool>,
}

impl KubernetesCluster_CorednsAutoscaler { }

impl terrars::ToListMappable for KubernetesCluster_CorednsAutoscaler {
    type O = terrars::BlockAssignable<KubernetesCluster_CorednsAutoscaler>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_CorednsAutoscaler {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildKubernetesCluster_CorednsAutoscaler {
    pub fn build(self) -> KubernetesCluster_CorednsAutoscaler {
        KubernetesCluster_CorednsAutoscaler { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_CorednsAutoscalerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_CorednsAutoscalerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_CorednsAutoscalerRef {
        KubernetesCluster_CorednsAutoscalerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_CorednsAutoscalerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_MaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KubernetesCluster_MaintenancePolicy {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KubernetesCluster_MaintenancePolicy {
    type O = terrars::BlockAssignable<KubernetesCluster_MaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_MaintenancePolicy {}

impl BuildKubernetesCluster_MaintenancePolicy {
    pub fn build(self) -> KubernetesCluster_MaintenancePolicy {
        KubernetesCluster_MaintenancePolicy {
            day: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_MaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_MaintenancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_MaintenancePolicyRef {
        KubernetesCluster_MaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_MaintenancePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_NfsCsiPlugin {
    enabled: terrars::PrimField<bool>,
}

impl KubernetesCluster_NfsCsiPlugin { }

impl terrars::ToListMappable for KubernetesCluster_NfsCsiPlugin {
    type O = terrars::BlockAssignable<KubernetesCluster_NfsCsiPlugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_NfsCsiPlugin {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildKubernetesCluster_NfsCsiPlugin {
    pub fn build(self) -> KubernetesCluster_NfsCsiPlugin {
        KubernetesCluster_NfsCsiPlugin { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_NfsCsiPluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_NfsCsiPluginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_NfsCsiPluginRef {
        KubernetesCluster_NfsCsiPluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_NfsCsiPluginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_NodePool_Nodes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    droplet_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KubernetesCluster_NodePool_Nodes {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `droplet_id`.\n"]
    pub fn set_droplet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.droplet_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KubernetesCluster_NodePool_Nodes {
    type O = terrars::BlockAssignable<KubernetesCluster_NodePool_Nodes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_NodePool_Nodes {}

impl BuildKubernetesCluster_NodePool_Nodes {
    pub fn build(self) -> KubernetesCluster_NodePool_Nodes {
        KubernetesCluster_NodePool_Nodes {
            created_at: core::default::Default::default(),
            droplet_id: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            status: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_NodePool_NodesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_NodePool_NodesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_NodePool_NodesRef {
        KubernetesCluster_NodePool_NodesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_NodePool_NodesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `droplet_id` after provisioning.\n"]
    pub fn droplet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.droplet_id", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_NodePool_Taint {
    effect: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl KubernetesCluster_NodePool_Taint { }

impl terrars::ToListMappable for KubernetesCluster_NodePool_Taint {
    type O = terrars::BlockAssignable<KubernetesCluster_NodePool_Taint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_NodePool_Taint {
    #[doc = ""]
    pub effect: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildKubernetesCluster_NodePool_Taint {
    pub fn build(self) -> KubernetesCluster_NodePool_Taint {
        KubernetesCluster_NodePool_Taint {
            effect: self.effect,
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_NodePool_TaintRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_NodePool_TaintRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_NodePool_TaintRef {
        KubernetesCluster_NodePool_TaintRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_NodePool_TaintRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effect` after provisioning.\n"]
    pub fn effect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct KubernetesCluster_NodePoolDynamic {
    taint: std::option::Option<terrars::DynamicBlock<KubernetesCluster_NodePool_Taint>>,
}

#[derive(Serialize)]
pub struct KubernetesCluster_NodePool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_scale: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_partition_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_nodes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_nodes: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    size: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taint: std::option::Option<std::vec::Vec<KubernetesCluster_NodePool_Taint>>,
    dynamic: KubernetesCluster_NodePoolDynamic,
}

impl KubernetesCluster_NodePool {
    #[doc = "Set the field `auto_scale`.\n"]
    pub fn set_auto_scale(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_scale = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gpu_partition_mode`.\n"]
    pub fn set_gpu_partition_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gpu_partition_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_nodes`.\n"]
    pub fn set_max_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_nodes`.\n"]
    pub fn set_min_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `taint`.\n"]
    pub fn set_taint(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesCluster_NodePool_Taint>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.taint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.taint = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KubernetesCluster_NodePool {
    type O = terrars::BlockAssignable<KubernetesCluster_NodePool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_NodePool {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub size: terrars::PrimField<std::string::String>,
}

impl BuildKubernetesCluster_NodePool {
    pub fn build(self) -> KubernetesCluster_NodePool {
        KubernetesCluster_NodePool {
            auto_scale: core::default::Default::default(),
            gpu_partition_mode: core::default::Default::default(),
            labels: core::default::Default::default(),
            max_nodes: core::default::Default::default(),
            min_nodes: core::default::Default::default(),
            name: self.name,
            node_count: core::default::Default::default(),
            size: self.size,
            tags: core::default::Default::default(),
            taint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_NodePoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_NodePoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_NodePoolRef {
        KubernetesCluster_NodePoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_NodePoolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actual_node_count` after provisioning.\n"]
    pub fn actual_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.actual_node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_scale` after provisioning.\n"]
    pub fn auto_scale(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_scale", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu_partition_mode` after provisioning.\n"]
    pub fn gpu_partition_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_partition_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `max_nodes` after provisioning.\n"]
    pub fn max_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `min_nodes` after provisioning.\n"]
    pub fn min_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `nodes` after provisioning.\n"]
    pub fn nodes(&self) -> terrars::ListRef<KubernetesCluster_NodePool_NodesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_NvidiaGpuDevicePlugin {
    enabled: terrars::PrimField<bool>,
}

impl KubernetesCluster_NvidiaGpuDevicePlugin { }

impl terrars::ToListMappable for KubernetesCluster_NvidiaGpuDevicePlugin {
    type O = terrars::BlockAssignable<KubernetesCluster_NvidiaGpuDevicePlugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_NvidiaGpuDevicePlugin {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildKubernetesCluster_NvidiaGpuDevicePlugin {
    pub fn build(self) -> KubernetesCluster_NvidiaGpuDevicePlugin {
        KubernetesCluster_NvidiaGpuDevicePlugin { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_NvidiaGpuDevicePluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_NvidiaGpuDevicePluginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_NvidiaGpuDevicePluginRef {
        KubernetesCluster_NvidiaGpuDevicePluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_NvidiaGpuDevicePluginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_NvidiaGpuDraDriver {
    enabled: terrars::PrimField<bool>,
}

impl KubernetesCluster_NvidiaGpuDraDriver { }

impl terrars::ToListMappable for KubernetesCluster_NvidiaGpuDraDriver {
    type O = terrars::BlockAssignable<KubernetesCluster_NvidiaGpuDraDriver>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_NvidiaGpuDraDriver {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildKubernetesCluster_NvidiaGpuDraDriver {
    pub fn build(self) -> KubernetesCluster_NvidiaGpuDraDriver {
        KubernetesCluster_NvidiaGpuDraDriver { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_NvidiaGpuDraDriverRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_NvidiaGpuDraDriverRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_NvidiaGpuDraDriverRef {
        KubernetesCluster_NvidiaGpuDraDriverRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_NvidiaGpuDraDriverRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_P2pOciRegistryPlugin {
    enabled: terrars::PrimField<bool>,
}

impl KubernetesCluster_P2pOciRegistryPlugin { }

impl terrars::ToListMappable for KubernetesCluster_P2pOciRegistryPlugin {
    type O = terrars::BlockAssignable<KubernetesCluster_P2pOciRegistryPlugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_P2pOciRegistryPlugin {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildKubernetesCluster_P2pOciRegistryPlugin {
    pub fn build(self) -> KubernetesCluster_P2pOciRegistryPlugin {
        KubernetesCluster_P2pOciRegistryPlugin { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_P2pOciRegistryPluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_P2pOciRegistryPluginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_P2pOciRegistryPluginRef {
        KubernetesCluster_P2pOciRegistryPluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_P2pOciRegistryPluginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_RdmaSharedDevicePlugin {
    enabled: terrars::PrimField<bool>,
}

impl KubernetesCluster_RdmaSharedDevicePlugin { }

impl terrars::ToListMappable for KubernetesCluster_RdmaSharedDevicePlugin {
    type O = terrars::BlockAssignable<KubernetesCluster_RdmaSharedDevicePlugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_RdmaSharedDevicePlugin {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildKubernetesCluster_RdmaSharedDevicePlugin {
    pub fn build(self) -> KubernetesCluster_RdmaSharedDevicePlugin {
        KubernetesCluster_RdmaSharedDevicePlugin { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_RdmaSharedDevicePluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_RdmaSharedDevicePluginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_RdmaSharedDevicePluginRef {
        KubernetesCluster_RdmaSharedDevicePluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_RdmaSharedDevicePluginRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_RoutingAgent {
    enabled: terrars::PrimField<bool>,
}

impl KubernetesCluster_RoutingAgent { }

impl terrars::ToListMappable for KubernetesCluster_RoutingAgent {
    type O = terrars::BlockAssignable<KubernetesCluster_RoutingAgent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_RoutingAgent {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildKubernetesCluster_RoutingAgent {
    pub fn build(self) -> KubernetesCluster_RoutingAgent {
        KubernetesCluster_RoutingAgent { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_RoutingAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_RoutingAgentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_RoutingAgentRef {
        KubernetesCluster_RoutingAgentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_RoutingAgentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_Sso {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::PrimField<bool>>,
}

impl KubernetesCluster_Sso {
    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer_url`.\n"]
    pub fn set_issuer_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\n"]
    pub fn set_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KubernetesCluster_Sso {
    type O = terrars::BlockAssignable<KubernetesCluster_Sso>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_Sso {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildKubernetesCluster_Sso {
    pub fn build(self) -> KubernetesCluster_Sso {
        KubernetesCluster_Sso {
            client_id: core::default::Default::default(),
            enabled: self.enabled,
            issuer_url: core::default::Default::default(),
            required: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_SsoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_SsoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_SsoRef {
        KubernetesCluster_SsoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_SsoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer_url` after provisioning.\n"]
    pub fn issuer_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer_url", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }
}

#[derive(Serialize)]
pub struct KubernetesCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KubernetesCluster_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KubernetesCluster_Timeouts {
    type O = terrars::BlockAssignable<KubernetesCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesCluster_Timeouts {}

impl BuildKubernetesCluster_Timeouts {
    pub fn build(self) -> KubernetesCluster_Timeouts {
        KubernetesCluster_Timeouts { create: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct KubernetesCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesCluster_TimeoutsRef {
        KubernetesCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesCluster_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct KubernetesClusterDynamic {
    amd_gpu_device_metrics_exporter_plugin: std
    ::option
    ::Option<terrars::DynamicBlock<KubernetesCluster_AmdGpuDeviceMetricsExporterPlugin>>,
    amd_gpu_device_plugin: std::option::Option<terrars::DynamicBlock<KubernetesCluster_AmdGpuDevicePlugin>>,
    amd_gpu_dra_driver: std::option::Option<terrars::DynamicBlock<KubernetesCluster_AmdGpuDraDriver>>,
    cluster_autoscaler_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KubernetesCluster_ClusterAutoscalerConfiguration>>,
    control_plane_firewall: std::option::Option<terrars::DynamicBlock<KubernetesCluster_ControlPlaneFirewall>>,
    coredns_autoscaler: std::option::Option<terrars::DynamicBlock<KubernetesCluster_CorednsAutoscaler>>,
    maintenance_policy: std::option::Option<terrars::DynamicBlock<KubernetesCluster_MaintenancePolicy>>,
    nfs_csi_plugin: std::option::Option<terrars::DynamicBlock<KubernetesCluster_NfsCsiPlugin>>,
    node_pool: std::option::Option<terrars::DynamicBlock<KubernetesCluster_NodePool>>,
    nvidia_gpu_device_plugin: std::option::Option<terrars::DynamicBlock<KubernetesCluster_NvidiaGpuDevicePlugin>>,
    nvidia_gpu_dra_driver: std::option::Option<terrars::DynamicBlock<KubernetesCluster_NvidiaGpuDraDriver>>,
    p2p_oci_registry_plugin: std::option::Option<terrars::DynamicBlock<KubernetesCluster_P2pOciRegistryPlugin>>,
    rdma_shared_device_plugin: std::option::Option<terrars::DynamicBlock<KubernetesCluster_RdmaSharedDevicePlugin>>,
    routing_agent: std::option::Option<terrars::DynamicBlock<KubernetesCluster_RoutingAgent>>,
    sso: std::option::Option<terrars::DynamicBlock<KubernetesCluster_Sso>>,
}
