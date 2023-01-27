use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataKubernetesClusterData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
}

struct DataKubernetesCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKubernetesClusterData>,
}

#[derive(Clone)]
pub struct DataKubernetesCluster(Rc<DataKubernetesCluster_>);

impl DataKubernetesCluster {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn set_provider(&self, provider: &ProviderDigitalocean) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `auto_upgrade` after provisioning.\n"]
    pub fn auto_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cluster_subnet` after provisioning.\n"]
    pub fn cluster_subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_subnet", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ha` after provisioning.\n"]
    pub fn ha(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_address` after provisioning.\n"]
    pub fn ipv4_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kube_config` after provisioning.\n"]
    pub fn kube_config(&self) -> ListRef<DataKubernetesClusterKubeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kube_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<DataKubernetesClusterMaintenancePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_pool` after provisioning.\n"]
    pub fn node_pool(&self) -> ListRef<DataKubernetesClusterNodePoolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_subnet` after provisioning.\n"]
    pub fn service_subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_subnet", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `surge_upgrade` after provisioning.\n"]
    pub fn surge_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.surge_upgrade", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\n"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_uuid` after provisioning.\n"]
    pub fn vpc_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }
}

impl Datasource for DataKubernetesCluster {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataKubernetesCluster {
    type O = ListRef<DataKubernetesClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataKubernetesCluster_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_kubernetes_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKubernetesCluster {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildDataKubernetesCluster {
    pub fn build(self, stack: &mut Stack) -> DataKubernetesCluster {
        let out = DataKubernetesCluster(Rc::new(DataKubernetesCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKubernetesClusterData {
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKubernetesClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKubernetesClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataKubernetesClusterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `auto_upgrade` after provisioning.\n"]
    pub fn auto_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cluster_subnet` after provisioning.\n"]
    pub fn cluster_subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_subnet", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ha` after provisioning.\n"]
    pub fn ha(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ha", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_address` after provisioning.\n"]
    pub fn ipv4_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kube_config` after provisioning.\n"]
    pub fn kube_config(&self) -> ListRef<DataKubernetesClusterKubeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kube_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<DataKubernetesClusterMaintenancePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_pool` after provisioning.\n"]
    pub fn node_pool(&self) -> ListRef<DataKubernetesClusterNodePoolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_subnet` after provisioning.\n"]
    pub fn service_subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_subnet", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `surge_upgrade` after provisioning.\n"]
    pub fn surge_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.surge_upgrade", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\n"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_uuid` after provisioning.\n"]
    pub fn vpc_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataKubernetesClusterKubeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_ca_certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
}

impl DataKubernetesClusterKubeConfigEl {
    #[doc= "Set the field `client_certificate`.\n"]
    pub fn set_client_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `client_key`.\n"]
    pub fn set_client_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_key = Some(v.into());
        self
    }

    #[doc= "Set the field `cluster_ca_certificate`.\n"]
    pub fn set_cluster_ca_certificate(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_ca_certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `expires_at`.\n"]
    pub fn set_expires_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expires_at = Some(v.into());
        self
    }

    #[doc= "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host = Some(v.into());
        self
    }

    #[doc= "Set the field `raw_config`.\n"]
    pub fn set_raw_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_config = Some(v.into());
        self
    }

    #[doc= "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token = Some(v.into());
        self
    }
}

impl ToListMappable for DataKubernetesClusterKubeConfigEl {
    type O = BlockAssignable<DataKubernetesClusterKubeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesClusterKubeConfigEl {}

impl BuildDataKubernetesClusterKubeConfigEl {
    pub fn build(self) -> DataKubernetesClusterKubeConfigEl {
        DataKubernetesClusterKubeConfigEl {
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

pub struct DataKubernetesClusterKubeConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKubernetesClusterKubeConfigElRef {
    fn new(shared: StackShared, base: String) -> DataKubernetesClusterKubeConfigElRef {
        DataKubernetesClusterKubeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesClusterKubeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `client_certificate` after provisioning.\n"]
    pub fn client_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_certificate", self.base))
    }

    #[doc= "Get a reference to the value of field `client_key` after provisioning.\n"]
    pub fn client_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc= "Get a reference to the value of field `cluster_ca_certificate` after provisioning.\n"]
    pub fn cluster_ca_certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_ca_certificate", self.base))
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\n"]
    pub fn expires_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.base))
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc= "Get a reference to the value of field `raw_config` after provisioning.\n"]
    pub fn raw_config(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_config", self.base))
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKubernetesClusterMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}

impl DataKubernetesClusterMaintenancePolicyEl {
    #[doc= "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day = Some(v.into());
        self
    }

    #[doc= "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.duration = Some(v.into());
        self
    }

    #[doc= "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}

impl ToListMappable for DataKubernetesClusterMaintenancePolicyEl {
    type O = BlockAssignable<DataKubernetesClusterMaintenancePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesClusterMaintenancePolicyEl {}

impl BuildDataKubernetesClusterMaintenancePolicyEl {
    pub fn build(self) -> DataKubernetesClusterMaintenancePolicyEl {
        DataKubernetesClusterMaintenancePolicyEl {
            day: core::default::Default::default(),
            duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

pub struct DataKubernetesClusterMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKubernetesClusterMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataKubernetesClusterMaintenancePolicyElRef {
        DataKubernetesClusterMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesClusterMaintenancePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc= "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc= "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKubernetesClusterNodePoolElNodesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    droplet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
}

impl DataKubernetesClusterNodePoolElNodesEl {
    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `droplet_id`.\n"]
    pub fn set_droplet_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.droplet_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc= "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated_at = Some(v.into());
        self
    }
}

impl ToListMappable for DataKubernetesClusterNodePoolElNodesEl {
    type O = BlockAssignable<DataKubernetesClusterNodePoolElNodesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesClusterNodePoolElNodesEl {}

impl BuildDataKubernetesClusterNodePoolElNodesEl {
    pub fn build(self) -> DataKubernetesClusterNodePoolElNodesEl {
        DataKubernetesClusterNodePoolElNodesEl {
            created_at: core::default::Default::default(),
            droplet_id: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            status: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

pub struct DataKubernetesClusterNodePoolElNodesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKubernetesClusterNodePoolElNodesElRef {
    fn new(shared: StackShared, base: String) -> DataKubernetesClusterNodePoolElNodesElRef {
        DataKubernetesClusterNodePoolElNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesClusterNodePoolElNodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `droplet_id` after provisioning.\n"]
    pub fn droplet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.droplet_id", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKubernetesClusterNodePoolElTaintEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataKubernetesClusterNodePoolElTaintEl {
    #[doc= "Set the field `effect`.\n"]
    pub fn set_effect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effect = Some(v.into());
        self
    }

    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataKubernetesClusterNodePoolElTaintEl {
    type O = BlockAssignable<DataKubernetesClusterNodePoolElTaintEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesClusterNodePoolElTaintEl {}

impl BuildDataKubernetesClusterNodePoolElTaintEl {
    pub fn build(self) -> DataKubernetesClusterNodePoolElTaintEl {
        DataKubernetesClusterNodePoolElTaintEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataKubernetesClusterNodePoolElTaintElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKubernetesClusterNodePoolElTaintElRef {
    fn new(shared: StackShared, base: String) -> DataKubernetesClusterNodePoolElTaintElRef {
        DataKubernetesClusterNodePoolElTaintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesClusterNodePoolElTaintElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `effect` after provisioning.\n"]
    pub fn effect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKubernetesClusterNodePoolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    actual_node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_scale: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nodes: Option<ListField<DataKubernetesClusterNodePoolElNodesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taint: Option<SetField<DataKubernetesClusterNodePoolElTaintEl>>,
}

impl DataKubernetesClusterNodePoolEl {
    #[doc= "Set the field `actual_node_count`.\n"]
    pub fn set_actual_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.actual_node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_scale`.\n"]
    pub fn set_auto_scale(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_scale = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }

    #[doc= "Set the field `max_nodes`.\n"]
    pub fn set_max_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_nodes = Some(v.into());
        self
    }

    #[doc= "Set the field `min_nodes`.\n"]
    pub fn set_min_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_nodes = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `nodes`.\n"]
    pub fn set_nodes(mut self, v: impl Into<ListField<DataKubernetesClusterNodePoolElNodesEl>>) -> Self {
        self.nodes = Some(v.into());
        self
    }

    #[doc= "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.size = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }

    #[doc= "Set the field `taint`.\n"]
    pub fn set_taint(mut self, v: impl Into<SetField<DataKubernetesClusterNodePoolElTaintEl>>) -> Self {
        self.taint = Some(v.into());
        self
    }
}

impl ToListMappable for DataKubernetesClusterNodePoolEl {
    type O = BlockAssignable<DataKubernetesClusterNodePoolEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKubernetesClusterNodePoolEl {}

impl BuildDataKubernetesClusterNodePoolEl {
    pub fn build(self) -> DataKubernetesClusterNodePoolEl {
        DataKubernetesClusterNodePoolEl {
            actual_node_count: core::default::Default::default(),
            auto_scale: core::default::Default::default(),
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

pub struct DataKubernetesClusterNodePoolElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKubernetesClusterNodePoolElRef {
    fn new(shared: StackShared, base: String) -> DataKubernetesClusterNodePoolElRef {
        DataKubernetesClusterNodePoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKubernetesClusterNodePoolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `actual_node_count` after provisioning.\n"]
    pub fn actual_node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.actual_node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `auto_scale` after provisioning.\n"]
    pub fn auto_scale(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_scale", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc= "Get a reference to the value of field `max_nodes` after provisioning.\n"]
    pub fn max_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_nodes", self.base))
    }

    #[doc= "Get a reference to the value of field `min_nodes` after provisioning.\n"]
    pub fn min_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_nodes", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `nodes` after provisioning.\n"]
    pub fn nodes(&self) -> ListRef<DataKubernetesClusterNodePoolElNodesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nodes", self.base))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc= "Get a reference to the value of field `taint` after provisioning.\n"]
    pub fn taint(&self) -> SetRef<DataKubernetesClusterNodePoolElTaintElRef> {
        SetRef::new(self.shared().clone(), format!("{}.taint", self.base))
    }
}
