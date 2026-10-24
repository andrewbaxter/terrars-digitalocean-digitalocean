use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct KubernetesClusterData {
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
    destroy_all_associated_resources: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ha: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_integration: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    surge_upgrade: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_policy: std::option::Option<std::vec::Vec<KubernetesClusterMaintenancePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_pool: std::option::Option<std::vec::Vec<KubernetesClusterNodePoolEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<KubernetesClusterTimeoutsEl>,
    dynamic: KubernetesClusterDynamic,
}

struct KubernetesCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KubernetesClusterData>,
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

    #[doc = "Set the field `registry_integration`.\n"]
    pub fn set_registry_integration(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().registry_integration = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesClusterMaintenancePolicyEl>>,
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

    #[doc = "Set the field `node_pool`.\n"]
    pub fn set_node_pool(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesClusterNodePoolEl>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<KubernetesClusterTimeoutsEl>) -> Self {
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

    #[doc = "Get a reference to the value of field `kube_config` after provisioning.\n"]
    pub fn kube_config(&self) -> terrars::ListRef<KubernetesClusterKubeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_config", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<KubernetesClusterMaintenancePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool` after provisioning.\n"]
    pub fn node_pool(&self) -> terrars::ListRef<KubernetesClusterNodePoolElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KubernetesClusterTimeoutsElRef {
        KubernetesClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(KubernetesClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `kube_config` after provisioning.\n"]
    pub fn kube_config(&self) -> terrars::ListRef<KubernetesClusterKubeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kube_config", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<KubernetesClusterMaintenancePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_pool` after provisioning.\n"]
    pub fn node_pool(&self) -> terrars::ListRef<KubernetesClusterNodePoolElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KubernetesClusterTimeoutsElRef {
        KubernetesClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KubernetesClusterKubeConfigEl {
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

impl KubernetesClusterKubeConfigEl {
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

impl terrars::ToListMappable for KubernetesClusterKubeConfigEl {
    type O = terrars::BlockAssignable<KubernetesClusterKubeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesClusterKubeConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesClusterKubeConfigElRef {
        KubernetesClusterKubeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesClusterKubeConfigElRef {
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
pub struct KubernetesClusterMaintenancePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KubernetesClusterMaintenancePolicyEl {
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

impl terrars::ToListMappable for KubernetesClusterMaintenancePolicyEl {
    type O = terrars::BlockAssignable<KubernetesClusterMaintenancePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesClusterMaintenancePolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesClusterMaintenancePolicyElRef {
        KubernetesClusterMaintenancePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesClusterMaintenancePolicyElRef {
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
pub struct KubernetesClusterNodePoolElNodesEl {
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

impl KubernetesClusterNodePoolElNodesEl {
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

impl terrars::ToListMappable for KubernetesClusterNodePoolElNodesEl {
    type O = terrars::BlockAssignable<KubernetesClusterNodePoolElNodesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesClusterNodePoolElNodesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesClusterNodePoolElNodesElRef {
        KubernetesClusterNodePoolElNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesClusterNodePoolElNodesElRef {
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
pub struct KubernetesClusterNodePoolElTaintEl {
    effect: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl KubernetesClusterNodePoolElTaintEl { }

impl terrars::ToListMappable for KubernetesClusterNodePoolElTaintEl {
    type O = terrars::BlockAssignable<KubernetesClusterNodePoolElTaintEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesClusterNodePoolElTaintEl {
    #[doc = ""]
    pub effect: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesClusterNodePoolElTaintElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesClusterNodePoolElTaintElRef {
        KubernetesClusterNodePoolElTaintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesClusterNodePoolElTaintElRef {
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
struct KubernetesClusterNodePoolElDynamic {
    taint: std::option::Option<terrars::DynamicBlock<KubernetesClusterNodePoolElTaintEl>>,
}

#[derive(Serialize)]
pub struct KubernetesClusterNodePoolEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_scale: std::option::Option<terrars::PrimField<bool>>,
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
    taint: std::option::Option<std::vec::Vec<KubernetesClusterNodePoolElTaintEl>>,
    dynamic: KubernetesClusterNodePoolElDynamic,
}

impl KubernetesClusterNodePoolEl {
    #[doc = "Set the field `auto_scale`.\n"]
    pub fn set_auto_scale(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_scale = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<KubernetesClusterNodePoolElTaintEl>>,
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

impl terrars::ToListMappable for KubernetesClusterNodePoolEl {
    type O = terrars::BlockAssignable<KubernetesClusterNodePoolEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKubernetesClusterNodePoolEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub size: terrars::PrimField<std::string::String>,
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KubernetesClusterNodePoolElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesClusterNodePoolElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesClusterNodePoolElRef {
        KubernetesClusterNodePoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesClusterNodePoolElRef {
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
    pub fn nodes(&self) -> terrars::ListRef<KubernetesClusterNodePoolElNodesElRef> {
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
pub struct KubernetesClusterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KubernetesClusterTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KubernetesClusterTimeoutsEl {
    type O = terrars::BlockAssignable<KubernetesClusterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KubernetesClusterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KubernetesClusterTimeoutsElRef {
        KubernetesClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KubernetesClusterTimeoutsElRef {
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
    maintenance_policy: std::option::Option<terrars::DynamicBlock<KubernetesClusterMaintenancePolicyEl>>,
    node_pool: std::option::Option<terrars::DynamicBlock<KubernetesClusterNodePoolEl>>,
}
