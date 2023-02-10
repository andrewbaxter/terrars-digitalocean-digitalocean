use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct ContainerRegistryDockerCredentialsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiry_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    registry_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write: Option<PrimField<bool>>,
}

struct ContainerRegistryDockerCredentials_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ContainerRegistryDockerCredentialsData>,
}

#[derive(Clone)]
pub struct ContainerRegistryDockerCredentials(Rc<ContainerRegistryDockerCredentials_>);

impl ContainerRegistryDockerCredentials {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
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

    #[doc= "Set the field `expiry_seconds`.\n"]
    pub fn set_expiry_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().expiry_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `write`.\n"]
    pub fn set_write(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().write = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `credential_expiration_time` after provisioning.\n"]
    pub fn credential_expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.credential_expiration_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `docker_credentials` after provisioning.\n"]
    pub fn docker_credentials(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.docker_credentials", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expiry_seconds` after provisioning.\n"]
    pub fn expiry_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiry_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `registry_name` after provisioning.\n"]
    pub fn registry_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.write", self.extract_ref()))
    }
}

impl Resource for ContainerRegistryDockerCredentials {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for ContainerRegistryDockerCredentials {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for ContainerRegistryDockerCredentials {
    type O = ListRef<ContainerRegistryDockerCredentialsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for ContainerRegistryDockerCredentials_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_container_registry_docker_credentials".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContainerRegistryDockerCredentials {
    pub tf_id: String,
    #[doc= ""]
    pub registry_name: PrimField<String>,
}

impl BuildContainerRegistryDockerCredentials {
    pub fn build(self, stack: &mut Stack) -> ContainerRegistryDockerCredentials {
        let out = ContainerRegistryDockerCredentials(Rc::new(ContainerRegistryDockerCredentials_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ContainerRegistryDockerCredentialsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                expiry_seconds: core::default::Default::default(),
                id: core::default::Default::default(),
                registry_name: self.registry_name,
                write: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContainerRegistryDockerCredentialsRef {
    shared: StackShared,
    base: String,
}

impl Ref for ContainerRegistryDockerCredentialsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContainerRegistryDockerCredentialsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `credential_expiration_time` after provisioning.\n"]
    pub fn credential_expiration_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.credential_expiration_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `docker_credentials` after provisioning.\n"]
    pub fn docker_credentials(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.docker_credentials", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expiry_seconds` after provisioning.\n"]
    pub fn expiry_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiry_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `registry_name` after provisioning.\n"]
    pub fn registry_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `write` after provisioning.\n"]
    pub fn write(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.write", self.extract_ref()))
    }
}
