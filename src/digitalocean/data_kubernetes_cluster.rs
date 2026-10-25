#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataKubernetesClusterData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubeconfig_expire_seconds: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amd_gpu_device_metrics_exporter_plugin: std
    ::option
    ::Option<std::vec::Vec<DataKubernetesCluster_AmdGpuDeviceMetricsExporterPlugin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amd_gpu_device_plugin: std::option::Option<std::vec::Vec<DataKubernetesCluster_AmdGpuDevicePlugin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amd_gpu_dra_driver: std::option::Option<std::vec::Vec<DataKubernetesCluster_AmdGpuDraDriver>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_autoscaler_configuration: std
    ::option
    ::Option<std::vec::Vec<DataKubernetesCluster_ClusterAutoscalerConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coredns_autoscaler: std::option::Option<std::vec::Vec<DataKubernetesCluster_CorednsAutoscaler>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs_csi_plugin: std::option::Option<std::vec::Vec<DataKubernetesCluster_NfsCsiPlugin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nvidia_gpu_device_plugin: std::option::Option<std::vec::Vec<DataKubernetesCluster_NvidiaGpuDevicePlugin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nvidia_gpu_dra_driver: std::option::Option<std::vec::Vec<DataKubernetesCluster_NvidiaGpuDraDriver>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    p2p_oci_registry_plugin: std::option::Option<std::vec::Vec<DataKubernetesCluster_P2pOciRegistryPlugin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rdma_shared_device_plugin: std::option::Option<std::vec::Vec<DataKubernetesCluster_RdmaSharedDevicePlugin>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_agent: std::option::Option<std::vec::Vec<DataKubernetesCluster_RoutingAgent>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sso: std::option::Option<std::vec::Vec<DataKubernetesCluster_Sso>>,
    dynamic: DataKubernetesClusterDynamic,
}

struct DataKubernetesCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataKubernetesClusterData_>,
}

#[derive(Clone)]
pub struct DataKubernetesCluster(std::rc::Rc<DataKubernetesCluster_>);

impl DataKubernetesCluster {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderDigitalocean) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kubeconfig_expire_seconds`.\n"]
    pub fn set_kubeconfig_expire_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().kubeconfig_expire_seconds = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `amd_gpu_device_metrics_exporter_plugin`.\n"]
    pub fn set_amd_gpu_device_metrics_exporter_plugin(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataKubernetesCluster_AmdGpuDeviceMetricsExporterPlugin>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataKubernetesCluster_AmdGpuDevicePlugin>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataKubernetesCluster_AmdGpuDraDriver>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataKubernetesCluster_ClusterAutoscalerConfiguration>>,
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

    #[doc = "Set the field `coredns_autoscaler`.\n"]
    pub fn set_coredns_autoscaler(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataKubernetesCluster_CorednsAutoscaler>>,
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

    #[doc = "Set the field `nfs_csi_plugin`.\n"]
    pub fn set_nfs_csi_plugin(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataKubernetesCluster_NfsCsiPlugin>>,
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

    #[doc = "Set the field `nvidia_gpu_device_plugin`.\n"]
    pub fn set_nvidia_gpu_device_plugin(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataKubernetesCluster_NvidiaGpuDevicePlugin>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataKubernetesCluster_NvidiaGpuDraDriver>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataKubernetesCluster_P2pOciRegistryPlugin>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataKubernetesCluster_RdmaSharedDevicePlugin>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataKubernetesCluster_RoutingAgent>>,
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
    pub fn set_sso(self, v: impl std::convert::Into<terrars::BlockAssignable<DataKubernetesCluster_Sso>>) -> Self {
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

    #[doc = "Get a reference to the value of field `auto_upgrade` after provisioning.\n"]
    pub fn auto_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_subnet` after provisioning.\n"]
    pub fn cluster_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_subnet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_firewall` after provisioning.\n"]
    pub fn control_plane_firewall(&self) -> terrars::ListRef<DataKubernetesCluster_ControlPlaneFirewallRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_firewall", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
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
    pub fn kube_config(&self) -> terrars::ListRef<DataKubernetesCluster_KubeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubeconfig_expire_seconds` after provisioning.\n"]
    pub fn kubeconfig_expire_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kubeconfig_expire_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<DataKubernetesCluster_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool` after provisioning.\n"]
    pub fn node_pool(&self) -> terrars::ListRef<DataKubernetesCluster_NodePoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    ) -> terrars::ListRef<DataKubernetesCluster_AmdGpuDeviceMetricsExporterPluginRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.amd_gpu_device_metrics_exporter_plugin", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `amd_gpu_device_plugin` after provisioning.\n"]
    pub fn amd_gpu_device_plugin(&self) -> terrars::ListRef<DataKubernetesCluster_AmdGpuDevicePluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amd_gpu_device_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amd_gpu_dra_driver` after provisioning.\n"]
    pub fn amd_gpu_dra_driver(&self) -> terrars::ListRef<DataKubernetesCluster_AmdGpuDraDriverRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amd_gpu_dra_driver", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_autoscaler_configuration` after provisioning.\n"]
    pub fn cluster_autoscaler_configuration(
        &self,
    ) -> terrars::ListRef<DataKubernetesCluster_ClusterAutoscalerConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_autoscaler_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `coredns_autoscaler` after provisioning.\n"]
    pub fn coredns_autoscaler(&self) -> terrars::ListRef<DataKubernetesCluster_CorednsAutoscalerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.coredns_autoscaler", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nfs_csi_plugin` after provisioning.\n"]
    pub fn nfs_csi_plugin(&self) -> terrars::ListRef<DataKubernetesCluster_NfsCsiPluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_csi_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nvidia_gpu_device_plugin` after provisioning.\n"]
    pub fn nvidia_gpu_device_plugin(&self) -> terrars::ListRef<DataKubernetesCluster_NvidiaGpuDevicePluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nvidia_gpu_device_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nvidia_gpu_dra_driver` after provisioning.\n"]
    pub fn nvidia_gpu_dra_driver(&self) -> terrars::ListRef<DataKubernetesCluster_NvidiaGpuDraDriverRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nvidia_gpu_dra_driver", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `p2p_oci_registry_plugin` after provisioning.\n"]
    pub fn p2p_oci_registry_plugin(&self) -> terrars::ListRef<DataKubernetesCluster_P2pOciRegistryPluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.p2p_oci_registry_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rdma_shared_device_plugin` after provisioning.\n"]
    pub fn rdma_shared_device_plugin(&self) -> terrars::ListRef<DataKubernetesCluster_RdmaSharedDevicePluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rdma_shared_device_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_agent` after provisioning.\n"]
    pub fn routing_agent(&self) -> terrars::ListRef<DataKubernetesCluster_RoutingAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sso` after provisioning.\n"]
    pub fn sso(&self) -> terrars::ListRef<DataKubernetesCluster_SsoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sso", self.extract_ref()))
    }
}

impl terrars::Referable for DataKubernetesCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataKubernetesCluster { }

impl terrars::ToListMappable for DataKubernetesCluster {
    type O = terrars::ListRef<DataKubernetesClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataKubernetesCluster_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_kubernetes_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKubernetesCluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataKubernetesCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> DataKubernetesCluster {
        let out = DataKubernetesCluster(std::rc::Rc::new(DataKubernetesCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataKubernetesClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                kubeconfig_expire_seconds: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                amd_gpu_device_metrics_exporter_plugin: core::default::Default::default(),
                amd_gpu_device_plugin: core::default::Default::default(),
                amd_gpu_dra_driver: core::default::Default::default(),
                cluster_autoscaler_configuration: core::default::Default::default(),
                coredns_autoscaler: core::default::Default::default(),
                nfs_csi_plugin: core::default::Default::default(),
                nvidia_gpu_device_plugin: core::default::Default::default(),
                nvidia_gpu_dra_driver: core::default::Default::default(),
                p2p_oci_registry_plugin: core::default::Default::default(),
                rdma_shared_device_plugin: core::default::Default::default(),
                routing_agent: core::default::Default::default(),
                sso: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKubernetesClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataKubernetesClusterRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `auto_upgrade` after provisioning.\n"]
    pub fn auto_upgrade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_subnet` after provisioning.\n"]
    pub fn cluster_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_subnet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_plane_firewall` after provisioning.\n"]
    pub fn control_plane_firewall(&self) -> terrars::ListRef<DataKubernetesCluster_ControlPlaneFirewallRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_plane_firewall", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
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
    pub fn kube_config(&self) -> terrars::ListRef<DataKubernetesCluster_KubeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubeconfig_expire_seconds` after provisioning.\n"]
    pub fn kubeconfig_expire_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kubeconfig_expire_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<DataKubernetesCluster_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool` after provisioning.\n"]
    pub fn node_pool(&self) -> terrars::ListRef<DataKubernetesCluster_NodePoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    ) -> terrars::ListRef<DataKubernetesCluster_AmdGpuDeviceMetricsExporterPluginRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.amd_gpu_device_metrics_exporter_plugin", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `amd_gpu_device_plugin` after provisioning.\n"]
    pub fn amd_gpu_device_plugin(&self) -> terrars::ListRef<DataKubernetesCluster_AmdGpuDevicePluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amd_gpu_device_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amd_gpu_dra_driver` after provisioning.\n"]
    pub fn amd_gpu_dra_driver(&self) -> terrars::ListRef<DataKubernetesCluster_AmdGpuDraDriverRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amd_gpu_dra_driver", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_autoscaler_configuration` after provisioning.\n"]
    pub fn cluster_autoscaler_configuration(
        &self,
    ) -> terrars::ListRef<DataKubernetesCluster_ClusterAutoscalerConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_autoscaler_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `coredns_autoscaler` after provisioning.\n"]
    pub fn coredns_autoscaler(&self) -> terrars::ListRef<DataKubernetesCluster_CorednsAutoscalerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.coredns_autoscaler", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nfs_csi_plugin` after provisioning.\n"]
    pub fn nfs_csi_plugin(&self) -> terrars::ListRef<DataKubernetesCluster_NfsCsiPluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_csi_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nvidia_gpu_device_plugin` after provisioning.\n"]
    pub fn nvidia_gpu_device_plugin(&self) -> terrars::ListRef<DataKubernetesCluster_NvidiaGpuDevicePluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nvidia_gpu_device_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nvidia_gpu_dra_driver` after provisioning.\n"]
    pub fn nvidia_gpu_dra_driver(&self) -> terrars::ListRef<DataKubernetesCluster_NvidiaGpuDraDriverRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nvidia_gpu_dra_driver", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `p2p_oci_registry_plugin` after provisioning.\n"]
    pub fn p2p_oci_registry_plugin(&self) -> terrars::ListRef<DataKubernetesCluster_P2pOciRegistryPluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.p2p_oci_registry_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rdma_shared_device_plugin` after provisioning.\n"]
    pub fn rdma_shared_device_plugin(&self) -> terrars::ListRef<DataKubernetesCluster_RdmaSharedDevicePluginRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rdma_shared_device_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_agent` after provisioning.\n"]
    pub fn routing_agent(&self) -> terrars::ListRef<DataKubernetesCluster_RoutingAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sso` after provisioning.\n"]
    pub fn sso(&self) -> terrars::ListRef<DataKubernetesCluster_SsoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sso", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataKubernetesClusterDynamic {
    amd_gpu_device_metrics_exporter_plugin: std
    ::option
    ::Option<terrars::DynamicBlock<DataKubernetesCluster_AmdGpuDeviceMetricsExporterPlugin>>,
    amd_gpu_device_plugin: std::option::Option<terrars::DynamicBlock<DataKubernetesCluster_AmdGpuDevicePlugin>>,
    amd_gpu_dra_driver: std::option::Option<terrars::DynamicBlock<DataKubernetesCluster_AmdGpuDraDriver>>,
    cluster_autoscaler_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<DataKubernetesCluster_ClusterAutoscalerConfiguration>>,
    coredns_autoscaler: std::option::Option<terrars::DynamicBlock<DataKubernetesCluster_CorednsAutoscaler>>,
    nfs_csi_plugin: std::option::Option<terrars::DynamicBlock<DataKubernetesCluster_NfsCsiPlugin>>,
    nvidia_gpu_device_plugin: std
    ::option
    ::Option<terrars::DynamicBlock<DataKubernetesCluster_NvidiaGpuDevicePlugin>>,
    nvidia_gpu_dra_driver: std::option::Option<terrars::DynamicBlock<DataKubernetesCluster_NvidiaGpuDraDriver>>,
    p2p_oci_registry_plugin: std::option::Option<terrars::DynamicBlock<DataKubernetesCluster_P2pOciRegistryPlugin>>,
    rdma_shared_device_plugin: std
    ::option
    ::Option<terrars::DynamicBlock<DataKubernetesCluster_RdmaSharedDevicePlugin>>,
    routing_agent: std::option::Option<terrars::DynamicBlock<DataKubernetesCluster_RoutingAgent>>,
    sso: std::option::Option<terrars::DynamicBlock<DataKubernetesCluster_Sso>>,
}

#[derive(Serialize)]
pub struct DataKubernetesCluster_ControlPlaneFirewall {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataKubernetesCluster_ControlPlaneFirewall {
    #[doc = "Set the field `allowed_addresses`.\n"]
    pub fn set_allowed_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKubernetesCluster_ControlPlaneFirewall {
    type O = terrars::BlockAssignable<DataKubernetesCluster_ControlPlaneFirewall>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_ControlPlaneFirewall {}

impl BuildDataKubernetesCluster_ControlPlaneFirewall {
    pub fn build(self) -> DataKubernetesCluster_ControlPlaneFirewall {
        DataKubernetesCluster_ControlPlaneFirewall {
            allowed_addresses: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_ControlPlaneFirewallRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_ControlPlaneFirewallRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKubernetesCluster_ControlPlaneFirewallRef {
        DataKubernetesCluster_ControlPlaneFirewallRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_ControlPlaneFirewallRef {
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
pub struct DataKubernetesCluster_KubeConfig {
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

impl DataKubernetesCluster_KubeConfig {
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

impl terrars::ToListMappable for DataKubernetesCluster_KubeConfig {
    type O = terrars::BlockAssignable<DataKubernetesCluster_KubeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_KubeConfig {}

impl BuildDataKubernetesCluster_KubeConfig {
    pub fn build(self) -> DataKubernetesCluster_KubeConfig {
        DataKubernetesCluster_KubeConfig {
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
pub struct DataKubernetesCluster_KubeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_KubeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKubernetesCluster_KubeConfigRef {
        DataKubernetesCluster_KubeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_KubeConfigRef {
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
pub struct DataKubernetesCluster_MaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKubernetesCluster_MaintenancePolicy {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKubernetesCluster_MaintenancePolicy {
    type O = terrars::BlockAssignable<DataKubernetesCluster_MaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_MaintenancePolicy {}

impl BuildDataKubernetesCluster_MaintenancePolicy {
    pub fn build(self) -> DataKubernetesCluster_MaintenancePolicy {
        DataKubernetesCluster_MaintenancePolicy {
            day: core::default::Default::default(),
            duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_MaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_MaintenancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKubernetesCluster_MaintenancePolicyRef {
        DataKubernetesCluster_MaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_MaintenancePolicyRef {
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
pub struct DataKubernetesCluster_NodePool_Nodes {
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

impl DataKubernetesCluster_NodePool_Nodes {
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

impl terrars::ToListMappable for DataKubernetesCluster_NodePool_Nodes {
    type O = terrars::BlockAssignable<DataKubernetesCluster_NodePool_Nodes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_NodePool_Nodes {}

impl BuildDataKubernetesCluster_NodePool_Nodes {
    pub fn build(self) -> DataKubernetesCluster_NodePool_Nodes {
        DataKubernetesCluster_NodePool_Nodes {
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
pub struct DataKubernetesCluster_NodePool_NodesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_NodePool_NodesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKubernetesCluster_NodePool_NodesRef {
        DataKubernetesCluster_NodePool_NodesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_NodePool_NodesRef {
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
pub struct DataKubernetesCluster_NodePool_Taint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKubernetesCluster_NodePool_Taint {
    #[doc = "Set the field `effect`.\n"]
    pub fn set_effect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKubernetesCluster_NodePool_Taint {
    type O = terrars::BlockAssignable<DataKubernetesCluster_NodePool_Taint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_NodePool_Taint {}

impl BuildDataKubernetesCluster_NodePool_Taint {
    pub fn build(self) -> DataKubernetesCluster_NodePool_Taint {
        DataKubernetesCluster_NodePool_Taint {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_NodePool_TaintRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_NodePool_TaintRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKubernetesCluster_NodePool_TaintRef {
        DataKubernetesCluster_NodePool_TaintRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_NodePool_TaintRef {
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

#[derive(Serialize)]
pub struct DataKubernetesCluster_NodePool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actual_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_scale: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_partition_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_nodes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_nodes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nodes: std::option::Option<terrars::ListField<DataKubernetesCluster_NodePool_Nodes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    taint: std::option::Option<terrars::SetField<DataKubernetesCluster_NodePool_Taint>>,
}

impl DataKubernetesCluster_NodePool {
    #[doc = "Set the field `actual_node_count`.\n"]
    pub fn set_actual_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.actual_node_count = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nodes`.\n"]
    pub fn set_nodes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataKubernetesCluster_NodePool_Nodes>>,
    ) -> Self {
        self.nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.size = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::SetField<DataKubernetesCluster_NodePool_Taint>>,
    ) -> Self {
        self.taint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKubernetesCluster_NodePool {
    type O = terrars::BlockAssignable<DataKubernetesCluster_NodePool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_NodePool {}

impl BuildDataKubernetesCluster_NodePool {
    pub fn build(self) -> DataKubernetesCluster_NodePool {
        DataKubernetesCluster_NodePool {
            actual_node_count: core::default::Default::default(),
            auto_scale: core::default::Default::default(),
            gpu_partition_mode: core::default::Default::default(),
            id: core::default::Default::default(),
            labels: core::default::Default::default(),
            max_nodes: core::default::Default::default(),
            min_nodes: core::default::Default::default(),
            name: core::default::Default::default(),
            node_count: core::default::Default::default(),
            nodes: core::default::Default::default(),
            size: core::default::Default::default(),
            tags: core::default::Default::default(),
            taint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_NodePoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_NodePoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKubernetesCluster_NodePoolRef {
        DataKubernetesCluster_NodePoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_NodePoolRef {
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
    pub fn nodes(&self) -> terrars::ListRef<DataKubernetesCluster_NodePool_NodesRef> {
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

    #[doc = "Get a reference to the value of field `taint` after provisioning.\n"]
    pub fn taint(&self) -> terrars::SetRef<DataKubernetesCluster_NodePool_TaintRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.taint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKubernetesCluster_AmdGpuDeviceMetricsExporterPlugin {}

impl DataKubernetesCluster_AmdGpuDeviceMetricsExporterPlugin { }

impl terrars::ToListMappable for DataKubernetesCluster_AmdGpuDeviceMetricsExporterPlugin {
    type O = terrars::BlockAssignable<DataKubernetesCluster_AmdGpuDeviceMetricsExporterPlugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_AmdGpuDeviceMetricsExporterPlugin {}

impl BuildDataKubernetesCluster_AmdGpuDeviceMetricsExporterPlugin {
    pub fn build(self) -> DataKubernetesCluster_AmdGpuDeviceMetricsExporterPlugin {
        DataKubernetesCluster_AmdGpuDeviceMetricsExporterPlugin {}
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_AmdGpuDeviceMetricsExporterPluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_AmdGpuDeviceMetricsExporterPluginRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKubernetesCluster_AmdGpuDeviceMetricsExporterPluginRef {
        DataKubernetesCluster_AmdGpuDeviceMetricsExporterPluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_AmdGpuDeviceMetricsExporterPluginRef {
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
pub struct DataKubernetesCluster_AmdGpuDevicePlugin {}

impl DataKubernetesCluster_AmdGpuDevicePlugin { }

impl terrars::ToListMappable for DataKubernetesCluster_AmdGpuDevicePlugin {
    type O = terrars::BlockAssignable<DataKubernetesCluster_AmdGpuDevicePlugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_AmdGpuDevicePlugin {}

impl BuildDataKubernetesCluster_AmdGpuDevicePlugin {
    pub fn build(self) -> DataKubernetesCluster_AmdGpuDevicePlugin {
        DataKubernetesCluster_AmdGpuDevicePlugin {}
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_AmdGpuDevicePluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_AmdGpuDevicePluginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKubernetesCluster_AmdGpuDevicePluginRef {
        DataKubernetesCluster_AmdGpuDevicePluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_AmdGpuDevicePluginRef {
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
pub struct DataKubernetesCluster_AmdGpuDraDriver {}

impl DataKubernetesCluster_AmdGpuDraDriver { }

impl terrars::ToListMappable for DataKubernetesCluster_AmdGpuDraDriver {
    type O = terrars::BlockAssignable<DataKubernetesCluster_AmdGpuDraDriver>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_AmdGpuDraDriver {}

impl BuildDataKubernetesCluster_AmdGpuDraDriver {
    pub fn build(self) -> DataKubernetesCluster_AmdGpuDraDriver {
        DataKubernetesCluster_AmdGpuDraDriver {}
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_AmdGpuDraDriverRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_AmdGpuDraDriverRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKubernetesCluster_AmdGpuDraDriverRef {
        DataKubernetesCluster_AmdGpuDraDriverRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_AmdGpuDraDriverRef {
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
pub struct DataKubernetesCluster_ClusterAutoscalerConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expanders: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_down_unneeded_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_down_utilization_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl DataKubernetesCluster_ClusterAutoscalerConfiguration {
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

impl terrars::ToListMappable for DataKubernetesCluster_ClusterAutoscalerConfiguration {
    type O = terrars::BlockAssignable<DataKubernetesCluster_ClusterAutoscalerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_ClusterAutoscalerConfiguration {}

impl BuildDataKubernetesCluster_ClusterAutoscalerConfiguration {
    pub fn build(self) -> DataKubernetesCluster_ClusterAutoscalerConfiguration {
        DataKubernetesCluster_ClusterAutoscalerConfiguration {
            expanders: core::default::Default::default(),
            scale_down_unneeded_time: core::default::Default::default(),
            scale_down_utilization_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_ClusterAutoscalerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_ClusterAutoscalerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKubernetesCluster_ClusterAutoscalerConfigurationRef {
        DataKubernetesCluster_ClusterAutoscalerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_ClusterAutoscalerConfigurationRef {
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
pub struct DataKubernetesCluster_CorednsAutoscaler {}

impl DataKubernetesCluster_CorednsAutoscaler { }

impl terrars::ToListMappable for DataKubernetesCluster_CorednsAutoscaler {
    type O = terrars::BlockAssignable<DataKubernetesCluster_CorednsAutoscaler>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_CorednsAutoscaler {}

impl BuildDataKubernetesCluster_CorednsAutoscaler {
    pub fn build(self) -> DataKubernetesCluster_CorednsAutoscaler {
        DataKubernetesCluster_CorednsAutoscaler {}
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_CorednsAutoscalerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_CorednsAutoscalerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKubernetesCluster_CorednsAutoscalerRef {
        DataKubernetesCluster_CorednsAutoscalerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_CorednsAutoscalerRef {
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
pub struct DataKubernetesCluster_NfsCsiPlugin {}

impl DataKubernetesCluster_NfsCsiPlugin { }

impl terrars::ToListMappable for DataKubernetesCluster_NfsCsiPlugin {
    type O = terrars::BlockAssignable<DataKubernetesCluster_NfsCsiPlugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_NfsCsiPlugin {}

impl BuildDataKubernetesCluster_NfsCsiPlugin {
    pub fn build(self) -> DataKubernetesCluster_NfsCsiPlugin {
        DataKubernetesCluster_NfsCsiPlugin {}
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_NfsCsiPluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_NfsCsiPluginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKubernetesCluster_NfsCsiPluginRef {
        DataKubernetesCluster_NfsCsiPluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_NfsCsiPluginRef {
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
pub struct DataKubernetesCluster_NvidiaGpuDevicePlugin {}

impl DataKubernetesCluster_NvidiaGpuDevicePlugin { }

impl terrars::ToListMappable for DataKubernetesCluster_NvidiaGpuDevicePlugin {
    type O = terrars::BlockAssignable<DataKubernetesCluster_NvidiaGpuDevicePlugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_NvidiaGpuDevicePlugin {}

impl BuildDataKubernetesCluster_NvidiaGpuDevicePlugin {
    pub fn build(self) -> DataKubernetesCluster_NvidiaGpuDevicePlugin {
        DataKubernetesCluster_NvidiaGpuDevicePlugin {}
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_NvidiaGpuDevicePluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_NvidiaGpuDevicePluginRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKubernetesCluster_NvidiaGpuDevicePluginRef {
        DataKubernetesCluster_NvidiaGpuDevicePluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_NvidiaGpuDevicePluginRef {
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
pub struct DataKubernetesCluster_NvidiaGpuDraDriver {}

impl DataKubernetesCluster_NvidiaGpuDraDriver { }

impl terrars::ToListMappable for DataKubernetesCluster_NvidiaGpuDraDriver {
    type O = terrars::BlockAssignable<DataKubernetesCluster_NvidiaGpuDraDriver>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_NvidiaGpuDraDriver {}

impl BuildDataKubernetesCluster_NvidiaGpuDraDriver {
    pub fn build(self) -> DataKubernetesCluster_NvidiaGpuDraDriver {
        DataKubernetesCluster_NvidiaGpuDraDriver {}
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_NvidiaGpuDraDriverRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_NvidiaGpuDraDriverRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKubernetesCluster_NvidiaGpuDraDriverRef {
        DataKubernetesCluster_NvidiaGpuDraDriverRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_NvidiaGpuDraDriverRef {
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
pub struct DataKubernetesCluster_P2pOciRegistryPlugin {}

impl DataKubernetesCluster_P2pOciRegistryPlugin { }

impl terrars::ToListMappable for DataKubernetesCluster_P2pOciRegistryPlugin {
    type O = terrars::BlockAssignable<DataKubernetesCluster_P2pOciRegistryPlugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_P2pOciRegistryPlugin {}

impl BuildDataKubernetesCluster_P2pOciRegistryPlugin {
    pub fn build(self) -> DataKubernetesCluster_P2pOciRegistryPlugin {
        DataKubernetesCluster_P2pOciRegistryPlugin {}
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_P2pOciRegistryPluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_P2pOciRegistryPluginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKubernetesCluster_P2pOciRegistryPluginRef {
        DataKubernetesCluster_P2pOciRegistryPluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_P2pOciRegistryPluginRef {
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
pub struct DataKubernetesCluster_RdmaSharedDevicePlugin {}

impl DataKubernetesCluster_RdmaSharedDevicePlugin { }

impl terrars::ToListMappable for DataKubernetesCluster_RdmaSharedDevicePlugin {
    type O = terrars::BlockAssignable<DataKubernetesCluster_RdmaSharedDevicePlugin>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_RdmaSharedDevicePlugin {}

impl BuildDataKubernetesCluster_RdmaSharedDevicePlugin {
    pub fn build(self) -> DataKubernetesCluster_RdmaSharedDevicePlugin {
        DataKubernetesCluster_RdmaSharedDevicePlugin {}
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_RdmaSharedDevicePluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_RdmaSharedDevicePluginRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKubernetesCluster_RdmaSharedDevicePluginRef {
        DataKubernetesCluster_RdmaSharedDevicePluginRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_RdmaSharedDevicePluginRef {
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
pub struct DataKubernetesCluster_RoutingAgent {}

impl DataKubernetesCluster_RoutingAgent { }

impl terrars::ToListMappable for DataKubernetesCluster_RoutingAgent {
    type O = terrars::BlockAssignable<DataKubernetesCluster_RoutingAgent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_RoutingAgent {}

impl BuildDataKubernetesCluster_RoutingAgent {
    pub fn build(self) -> DataKubernetesCluster_RoutingAgent {
        DataKubernetesCluster_RoutingAgent {}
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_RoutingAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_RoutingAgentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKubernetesCluster_RoutingAgentRef {
        DataKubernetesCluster_RoutingAgentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_RoutingAgentRef {
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
pub struct DataKubernetesCluster_Sso {}

impl DataKubernetesCluster_Sso { }

impl terrars::ToListMappable for DataKubernetesCluster_Sso {
    type O = terrars::BlockAssignable<DataKubernetesCluster_Sso>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesCluster_Sso {}

impl BuildDataKubernetesCluster_Sso {
    pub fn build(self) -> DataKubernetesCluster_Sso {
        DataKubernetesCluster_Sso {}
    }
}

#[allow(dead_code)]
pub struct DataKubernetesCluster_SsoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKubernetesCluster_SsoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKubernetesCluster_SsoRef {
        DataKubernetesCluster_SsoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesCluster_SsoRef {
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
