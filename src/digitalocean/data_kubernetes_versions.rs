use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataKubernetesVersionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_prefix: Option<PrimField<String>>,
}

struct DataKubernetesVersions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKubernetesVersionsData>,
}

#[derive(Clone)]
pub struct DataKubernetesVersions(Rc<DataKubernetesVersions_>);

impl DataKubernetesVersions {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
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

    #[doc= "Set the field `version_prefix`.\n"]
    pub fn set_version_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version_prefix = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `latest_version` after provisioning.\n"]
    pub fn latest_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.latest_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_versions` after provisioning.\n"]
    pub fn valid_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.valid_versions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version_prefix` after provisioning.\n"]
    pub fn version_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_prefix", self.extract_ref()))
    }
}

impl Datasource for DataKubernetesVersions {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataKubernetesVersions {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataKubernetesVersions {
    type O = ListRef<DataKubernetesVersionsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataKubernetesVersions_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_kubernetes_versions".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKubernetesVersions {
    pub tf_id: String,
}

impl BuildDataKubernetesVersions {
    pub fn build(self, stack: &mut Stack) -> DataKubernetesVersions {
        let out = DataKubernetesVersions(Rc::new(DataKubernetesVersions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKubernetesVersionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                version_prefix: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKubernetesVersionsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKubernetesVersionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataKubernetesVersionsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `latest_version` after provisioning.\n"]
    pub fn latest_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.latest_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_versions` after provisioning.\n"]
    pub fn valid_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.valid_versions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version_prefix` after provisioning.\n"]
    pub fn version_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_prefix", self.extract_ref()))
    }
}
