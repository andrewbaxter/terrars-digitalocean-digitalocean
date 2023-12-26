use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct KubernetesClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_upgrade: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destroy_all_associated_resources: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ha: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    region: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_integration: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    surge_upgrade: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_uuid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_policy: Option<Vec<KubernetesClusterMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_pool: Option<Vec<KubernetesClusterNodePoolEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<KubernetesClusterTimeoutsEl>,
    dynamic: KubernetesClusterDynamic,
}

struct KubernetesCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<KubernetesClusterData>,
}

#[derive(Clone)]
pub struct KubernetesCluster(Rc<KubernetesCluster_>);

impl KubernetesCluster {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderDigitalocean) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `auto_upgrade`.\n"]
    pub fn set_auto_upgrade(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_upgrade = Some(v.into());
        self
    }

    #[doc= "Set the field `destroy_all_associated_resources`.\n"]
    pub fn set_destroy_all_associated_resources(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().destroy_all_associated_resources = Some(v.into());
        self
    }

    #[doc= "Set the field `ha`.\n"]
    pub fn set_ha(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ha = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `registry_integration`.\n"]
    pub fn set_registry_integration(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().registry_integration = Some(v.into());
        self
    }

    #[doc= "Set the field `surge_upgrade`.\n"]
    pub fn set_surge_upgrade(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().surge_upgrade = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `vpc_uuid`.\n"]
    pub fn set_vpc_uuid(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc_uuid = Some(v.into());
        self
    }

    #[doc= "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(self, v: impl Into<BlockAssignable<KubernetesClusterMaintenancePolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `node_pool`.\n"]
    pub fn set_node_pool(self, v: impl Into<BlockAssignable<KubernetesClusterNodePoolEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_pool = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_pool = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<KubernetesClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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

    #[doc= "Get a reference to the value of field `destroy_all_associated_resources` after provisioning.\n"]
    pub fn destroy_all_associated_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.destroy_all_associated_resources", self.extract_ref()))
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
    pub fn kube_config(&self) -> ListRef<KubernetesClusterKubeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kube_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `registry_integration` after provisioning.\n"]
    pub fn registry_integration(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_integration", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<KubernetesClusterMaintenancePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_pool` after provisioning.\n"]
    pub fn node_pool(&self) -> ListRef<KubernetesClusterNodePoolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KubernetesClusterTimeoutsElRef {
        KubernetesClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for KubernetesCluster {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for KubernetesCluster { }

impl ToListMappable for KubernetesCluster {
    type O = ListRef<KubernetesClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for KubernetesCluster_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_kubernetes_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKubernetesCluster {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub region: PrimField<String>,
    #[doc= ""]
    pub version: PrimField<String>,
}

impl BuildKubernetesCluster {
    pub fn build(self, stack: &mut Stack) -> KubernetesCluster {
        let out = KubernetesCluster(Rc::new(KubernetesCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(KubernetesClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_upgrade: core::default::Default::default(),
                destroy_all_associated_resources: core::default::Default::default(),
                ha: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: self.region,
                registry_integration: core::default::Default::default(),
                surge_upgrade: core::default::Default::default(),
                tags: core::default::Default::default(),
                version: self.version,
                vpc_uuid: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                node_pool: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KubernetesClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for KubernetesClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl KubernetesClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
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

    #[doc= "Get a reference to the value of field `destroy_all_associated_resources` after provisioning.\n"]
    pub fn destroy_all_associated_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.destroy_all_associated_resources", self.extract_ref()))
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
    pub fn kube_config(&self) -> ListRef<KubernetesClusterKubeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kube_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `registry_integration` after provisioning.\n"]
    pub fn registry_integration(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_integration", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> ListRef<KubernetesClusterMaintenancePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_pool` after provisioning.\n"]
    pub fn node_pool(&self) -> ListRef<KubernetesClusterNodePoolElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KubernetesClusterTimeoutsElRef {
        KubernetesClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KubernetesClusterKubeConfigEl {
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

impl KubernetesClusterKubeConfigEl {
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

impl ToListMappable for KubernetesClusterKubeConfigEl {
    type O = BlockAssignable<KubernetesClusterKubeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesClusterKubeConfigEl {}

impl BuildKubernetesClusterKubeConfigEl {
    pub fn build(self) -> KubernetesClusterKubeConfigEl {
        KubernetesClusterKubeConfigEl {
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

pub struct KubernetesClusterKubeConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KubernetesClusterKubeConfigElRef {
    fn new(shared: StackShared, base: String) -> KubernetesClusterKubeConfigElRef {
        KubernetesClusterKubeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesClusterKubeConfigElRef {
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
pub struct KubernetesClusterMaintenancePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}

impl KubernetesClusterMaintenancePolicyEl {
    #[doc= "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.day = Some(v.into());
        self
    }

    #[doc= "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}

impl ToListMappable for KubernetesClusterMaintenancePolicyEl {
    type O = BlockAssignable<KubernetesClusterMaintenancePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesClusterMaintenancePolicyEl {}

impl BuildKubernetesClusterMaintenancePolicyEl {
    pub fn build(self) -> KubernetesClusterMaintenancePolicyEl {
        KubernetesClusterMaintenancePolicyEl {
            day: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

pub struct KubernetesClusterMaintenancePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KubernetesClusterMaintenancePolicyElRef {
    fn new(shared: StackShared, base: String) -> KubernetesClusterMaintenancePolicyElRef {
        KubernetesClusterMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesClusterMaintenancePolicyElRef {
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
pub struct KubernetesClusterNodePoolElNodesEl {
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

impl KubernetesClusterNodePoolElNodesEl {
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

impl ToListMappable for KubernetesClusterNodePoolElNodesEl {
    type O = BlockAssignable<KubernetesClusterNodePoolElNodesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesClusterNodePoolElNodesEl {}

impl BuildKubernetesClusterNodePoolElNodesEl {
    pub fn build(self) -> KubernetesClusterNodePoolElNodesEl {
        KubernetesClusterNodePoolElNodesEl {
            created_at: core::default::Default::default(),
            droplet_id: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            status: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

pub struct KubernetesClusterNodePoolElNodesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KubernetesClusterNodePoolElNodesElRef {
    fn new(shared: StackShared, base: String) -> KubernetesClusterNodePoolElNodesElRef {
        KubernetesClusterNodePoolElNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesClusterNodePoolElNodesElRef {
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
pub struct KubernetesClusterNodePoolElTaintEl {
    effect: PrimField<String>,
    key: PrimField<String>,
    value: PrimField<String>,
}

impl KubernetesClusterNodePoolElTaintEl { }

impl ToListMappable for KubernetesClusterNodePoolElTaintEl {
    type O = BlockAssignable<KubernetesClusterNodePoolElTaintEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesClusterNodePoolElTaintEl {
    #[doc= ""]
    pub effect: PrimField<String>,
    #[doc= ""]
    pub key: PrimField<String>,
    #[doc= ""]
    pub value: PrimField<String>,
}

impl BuildKubernetesClusterNodePoolElTaintEl {
    pub fn build(self) -> KubernetesClusterNodePoolElTaintEl {
        KubernetesClusterNodePoolElTaintEl {
            effect: self.effect,
            key: self.key,
            value: self.value,
        }
    }
}

pub struct KubernetesClusterNodePoolElTaintElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KubernetesClusterNodePoolElTaintElRef {
    fn new(shared: StackShared, base: String) -> KubernetesClusterNodePoolElTaintElRef {
        KubernetesClusterNodePoolElTaintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesClusterNodePoolElTaintElRef {
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

#[derive(Serialize, Default)]
struct KubernetesClusterNodePoolElDynamic {
    taint: Option<DynamicBlock<KubernetesClusterNodePoolElTaintEl>>,
}

#[derive(Serialize)]
pub struct KubernetesClusterNodePoolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_scale: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_nodes: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
    size: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taint: Option<Vec<KubernetesClusterNodePoolElTaintEl>>,
    dynamic: KubernetesClusterNodePoolElDynamic,
}

impl KubernetesClusterNodePoolEl {
    #[doc= "Set the field `auto_scale`.\n"]
    pub fn set_auto_scale(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_scale = Some(v.into());
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

    #[doc= "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }

    #[doc= "Set the field `taint`.\n"]
    pub fn set_taint(mut self, v: impl Into<BlockAssignable<KubernetesClusterNodePoolElTaintEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.taint = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.taint = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for KubernetesClusterNodePoolEl {
    type O = BlockAssignable<KubernetesClusterNodePoolEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesClusterNodePoolEl {
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub size: PrimField<String>,
}

impl BuildKubernetesClusterNodePoolEl {
    pub fn build(self) -> KubernetesClusterNodePoolEl {
        KubernetesClusterNodePoolEl {
            auto_scale: core::default::Default::default(),
            labels: core::default::Default::default(),
            max_nodes: core::default::Default::default(),
            min_nodes: core::default::Default::default(),
            name: self.name,
            node_count: core::default::Default::default(),
            size: self.size,
            tags: core::default::Default::default(),
            taint: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KubernetesClusterNodePoolElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KubernetesClusterNodePoolElRef {
    fn new(shared: StackShared, base: String) -> KubernetesClusterNodePoolElRef {
        KubernetesClusterNodePoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesClusterNodePoolElRef {
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
    pub fn nodes(&self) -> ListRef<KubernetesClusterNodePoolElNodesElRef> {
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
}

#[derive(Serialize)]
pub struct KubernetesClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl KubernetesClusterTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for KubernetesClusterTimeoutsEl {
    type O = BlockAssignable<KubernetesClusterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesClusterTimeoutsEl {}

impl BuildKubernetesClusterTimeoutsEl {
    pub fn build(self) -> KubernetesClusterTimeoutsEl {
        KubernetesClusterTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct KubernetesClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KubernetesClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> KubernetesClusterTimeoutsElRef {
        KubernetesClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesClusterTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct KubernetesClusterDynamic {
    maintenance_policy: Option<DynamicBlock<KubernetesClusterMaintenancePolicyEl>>,
    node_pool: Option<DynamicBlock<KubernetesClusterNodePoolEl>>,
}
