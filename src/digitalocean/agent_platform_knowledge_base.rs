#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct AgentPlatformKnowledgeBaseData_ {
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
    added_to_agent_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_id: std::option::Option<terrars::PrimField<std::string::String>>,
    embedding_model_uuid: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_public: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datasources: std::option::Option<std::vec::Vec<AgentPlatformKnowledgeBase_Datasources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_indexing_job: std::option::Option<std::vec::Vec<AgentPlatformKnowledgeBase_LastIndexingJob>>,
    dynamic: AgentPlatformKnowledgeBaseDynamic,
}

struct AgentPlatformKnowledgeBase_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AgentPlatformKnowledgeBaseData_>,
}

#[derive(Clone)]
pub struct AgentPlatformKnowledgeBase(std::rc::Rc<AgentPlatformKnowledgeBase_>);

impl AgentPlatformKnowledgeBase {
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

    #[doc = "Set the field `added_to_agent_at`.\nThe time when the knowledge base was added to the agent."]
    pub fn set_added_to_agent_at(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().added_to_agent_at = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `database_id`.\nThe unique identifier of the DigitalOcean OpenSearch database this knowledge base will use"]
    pub fn set_database_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_public`.\nIndicates whether the knowledge base is public or private."]
    pub fn set_is_public(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_public = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `vpc_uuid`.\nThe unique identifier of the VPC to which the knowledge base belongs."]
    pub fn set_vpc_uuid(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `datasources`.\n"]
    pub fn set_datasources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgentPlatformKnowledgeBase_Datasources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().datasources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.datasources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `last_indexing_job`.\n"]
    pub fn set_last_indexing_job(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgentPlatformKnowledgeBase_LastIndexingJob>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().last_indexing_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.last_indexing_job = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `added_to_agent_at` after provisioning.\nThe time when the knowledge base was added to the agent."]
    pub fn added_to_agent_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.added_to_agent_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe time when the knowledge base was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_id` after provisioning.\nThe unique identifier of the DigitalOcean OpenSearch database this knowledge base will use"]
    pub fn database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `embedding_model_uuid` after provisioning.\nThe unique identifier of the embedding model"]
    pub fn embedding_model_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_model_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_public` after provisioning.\nIndicates whether the knowledge base is public or private."]
    pub fn is_public(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_public", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the knowledge base."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe unique identifier of the project to which the knowledge base belongs."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_uuid` after provisioning.\nThe unique identifier of the VPC to which the knowledge base belongs."]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `datasources` after provisioning.\n"]
    pub fn datasources(&self) -> terrars::ListRef<AgentPlatformKnowledgeBase_DatasourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datasources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_indexing_job` after provisioning.\n"]
    pub fn last_indexing_job(&self) -> terrars::ListRef<AgentPlatformKnowledgeBase_LastIndexingJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_indexing_job", self.extract_ref()))
    }
}

impl terrars::Referable for AgentPlatformKnowledgeBase {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AgentPlatformKnowledgeBase { }

impl terrars::ToListMappable for AgentPlatformKnowledgeBase {
    type O = terrars::ListRef<AgentPlatformKnowledgeBaseRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AgentPlatformKnowledgeBase_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_agent_platform_knowledge_base".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAgentPlatformKnowledgeBase {
    pub tf_id: std::string::String,
    #[doc = "The unique identifier of the embedding model"]
    pub embedding_model_uuid: terrars::PrimField<std::string::String>,
    #[doc = "The name of the knowledge base."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The unique identifier of the project to which the knowledge base belongs."]
    pub project_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildAgentPlatformKnowledgeBase {
    pub fn build(self, stack: &mut terrars::Stack) -> AgentPlatformKnowledgeBase {
        let out = AgentPlatformKnowledgeBase(std::rc::Rc::new(AgentPlatformKnowledgeBase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AgentPlatformKnowledgeBaseData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                added_to_agent_at: core::default::Default::default(),
                database_id: core::default::Default::default(),
                embedding_model_uuid: self.embedding_model_uuid,
                id: core::default::Default::default(),
                is_public: core::default::Default::default(),
                name: self.name,
                project_id: self.project_id,
                region: self.region,
                tags: core::default::Default::default(),
                vpc_uuid: core::default::Default::default(),
                datasources: core::default::Default::default(),
                last_indexing_job: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AgentPlatformKnowledgeBaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformKnowledgeBaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AgentPlatformKnowledgeBaseRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `added_to_agent_at` after provisioning.\nThe time when the knowledge base was added to the agent."]
    pub fn added_to_agent_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.added_to_agent_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe time when the knowledge base was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_id` after provisioning.\nThe unique identifier of the DigitalOcean OpenSearch database this knowledge base will use"]
    pub fn database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `embedding_model_uuid` after provisioning.\nThe unique identifier of the embedding model"]
    pub fn embedding_model_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_model_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_public` after provisioning.\nIndicates whether the knowledge base is public or private."]
    pub fn is_public(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_public", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the knowledge base."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe unique identifier of the project to which the knowledge base belongs."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_uuid` after provisioning.\nThe unique identifier of the VPC to which the knowledge base belongs."]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `datasources` after provisioning.\n"]
    pub fn datasources(&self) -> terrars::ListRef<AgentPlatformKnowledgeBase_DatasourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datasources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_indexing_job` after provisioning.\n"]
    pub fn last_indexing_job(&self) -> terrars::ListRef<AgentPlatformKnowledgeBase_LastIndexingJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_indexing_job", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AgentPlatformKnowledgeBaseDynamic {
    datasources: std::option::Option<terrars::DynamicBlock<AgentPlatformKnowledgeBase_Datasources>>,
    last_indexing_job: std::option::Option<terrars::DynamicBlock<AgentPlatformKnowledgeBase_LastIndexingJob>>,
}

#[derive(Serialize)]
pub struct AgentPlatformKnowledgeBase_Datasources_FileUploadDataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    original_file_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_in_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stored_object_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgentPlatformKnowledgeBase_Datasources_FileUploadDataSource {
    #[doc = "Set the field `original_file_name`.\nThe original name of the uploaded file"]
    pub fn set_original_file_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.original_file_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_in_bytes`.\nThe size of the file in bytes"]
    pub fn set_size_in_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.size_in_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stored_object_key`.\nThe stored object key for the file"]
    pub fn set_stored_object_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stored_object_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentPlatformKnowledgeBase_Datasources_FileUploadDataSource {
    type O = terrars::BlockAssignable<AgentPlatformKnowledgeBase_Datasources_FileUploadDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentPlatformKnowledgeBase_Datasources_FileUploadDataSource {}

impl BuildAgentPlatformKnowledgeBase_Datasources_FileUploadDataSource {
    pub fn build(self) -> AgentPlatformKnowledgeBase_Datasources_FileUploadDataSource {
        AgentPlatformKnowledgeBase_Datasources_FileUploadDataSource {
            original_file_name: core::default::Default::default(),
            size_in_bytes: core::default::Default::default(),
            stored_object_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentPlatformKnowledgeBase_Datasources_FileUploadDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformKnowledgeBase_Datasources_FileUploadDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgentPlatformKnowledgeBase_Datasources_FileUploadDataSourceRef {
        AgentPlatformKnowledgeBase_Datasources_FileUploadDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentPlatformKnowledgeBase_Datasources_FileUploadDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `original_file_name` after provisioning.\nThe original name of the uploaded file"]
    pub fn original_file_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.original_file_name", self.base))
    }

    #[doc = "Get a reference to the value of field `size_in_bytes` after provisioning.\nThe size of the file in bytes"]
    pub fn size_in_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_in_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stored_object_key` after provisioning.\nThe stored object key for the file"]
    pub fn stored_object_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stored_object_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AgentPlatformKnowledgeBase_LastIndexingJob {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    completed_datasources: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_uuids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phase: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tokens: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_datasources: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgentPlatformKnowledgeBase_LastIndexingJob {
    #[doc = "Set the field `completed_datasources`.\nNumber of completed datasources in the last indexing job"]
    pub fn set_completed_datasources(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.completed_datasources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_uuids`.\nDatasource UUIDs for the last indexing job"]
    pub fn set_data_source_uuids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.data_source_uuids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phase`.\nPhase of the last indexing job"]
    pub fn set_phase(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tokens`.\nNumber of tokens processed in the last indexing job"]
    pub fn set_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_datasources`.\nTotal number of datasources in the last indexing job"]
    pub fn set_total_datasources(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_datasources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\nUUID  of the last indexing job"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentPlatformKnowledgeBase_LastIndexingJob {
    type O = terrars::BlockAssignable<AgentPlatformKnowledgeBase_LastIndexingJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentPlatformKnowledgeBase_LastIndexingJob {}

impl BuildAgentPlatformKnowledgeBase_LastIndexingJob {
    pub fn build(self) -> AgentPlatformKnowledgeBase_LastIndexingJob {
        AgentPlatformKnowledgeBase_LastIndexingJob {
            completed_datasources: core::default::Default::default(),
            data_source_uuids: core::default::Default::default(),
            phase: core::default::Default::default(),
            tokens: core::default::Default::default(),
            total_datasources: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentPlatformKnowledgeBase_LastIndexingJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformKnowledgeBase_LastIndexingJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AgentPlatformKnowledgeBase_LastIndexingJobRef {
        AgentPlatformKnowledgeBase_LastIndexingJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentPlatformKnowledgeBase_LastIndexingJobRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `completed_datasources` after provisioning.\nNumber of completed datasources in the last indexing job"]
    pub fn completed_datasources(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed_datasources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nCreated At timestamp for the last indexing job"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_source_uuids` after provisioning.\nDatasource UUIDs for the last indexing job"]
    pub fn data_source_uuids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source_uuids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `finished_at` after provisioning.\nTimestamp when the last indexing job finished"]
    pub fn finished_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finished_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `knowledge_base_uuid` after provisioning.\nUUID  of the Knowledge Base for the last indexing job"]
    pub fn knowledge_base_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `phase` after provisioning.\nPhase of the last indexing job"]
    pub fn phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `started_at` after provisioning.\nTimestamp when the last indexing job started"]
    pub fn started_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.started_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tokens` after provisioning.\nNumber of tokens processed in the last indexing job"]
    pub fn tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tokens", self.base))
    }

    #[doc =
        "Get a reference to the value of field `total_datasources` after provisioning.\nTotal number of datasources in the last indexing job"]
    pub fn total_datasources(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_datasources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the last indexing job updated"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nUUID  of the last indexing job"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct AgentPlatformKnowledgeBase_Datasources_SpacesDataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    item_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgentPlatformKnowledgeBase_Datasources_SpacesDataSource {
    #[doc = "Set the field `bucket_name`.\nThe name of the Spaces bucket"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `item_path`.\nThe path to the item in the bucket"]
    pub fn set_item_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.item_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe region of the Spaces bucket"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentPlatformKnowledgeBase_Datasources_SpacesDataSource {
    type O = terrars::BlockAssignable<AgentPlatformKnowledgeBase_Datasources_SpacesDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentPlatformKnowledgeBase_Datasources_SpacesDataSource {}

impl BuildAgentPlatformKnowledgeBase_Datasources_SpacesDataSource {
    pub fn build(self) -> AgentPlatformKnowledgeBase_Datasources_SpacesDataSource {
        AgentPlatformKnowledgeBase_Datasources_SpacesDataSource {
            bucket_name: core::default::Default::default(),
            item_path: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentPlatformKnowledgeBase_Datasources_SpacesDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformKnowledgeBase_Datasources_SpacesDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgentPlatformKnowledgeBase_Datasources_SpacesDataSourceRef {
        AgentPlatformKnowledgeBase_Datasources_SpacesDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentPlatformKnowledgeBase_Datasources_SpacesDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nThe name of the Spaces bucket"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `item_path` after provisioning.\nThe path to the item in the bucket"]
    pub fn item_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.item_path", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the Spaces bucket"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct AgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawling_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embed_media: std::option::Option<terrars::PrimField<bool>>,
}

impl AgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSource {
    #[doc = "Set the field `base_url`.\nThe base URL to crawl"]
    pub fn set_base_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.base_url = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `crawling_option`.\nOptions for specifying how URLs found on pages should be handled. \n- UNKNOWN: Default unknown value\n- SCOPED: Only include the base URL.\n- PATH: Crawl the base URL and linked pages within the URL path.\n- DOMAIN: Crawl the base URL and linked pages within the same domain.\n- SUBDOMAINS: Crawl the base URL and linked pages for any subdomain."]
    pub fn set_crawling_option(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.crawling_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `embed_media`.\nWhether to embed media content"]
    pub fn set_embed_media(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.embed_media = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSource {
    type O = terrars::BlockAssignable<AgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSource {}

impl BuildAgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSource {
    pub fn build(self) -> AgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSource {
        AgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSource {
            base_url: core::default::Default::default(),
            crawling_option: core::default::Default::default(),
            embed_media: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSourceRef {
        AgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_url` after provisioning.\nThe base URL to crawl"]
    pub fn base_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `crawling_option` after provisioning.\nOptions for specifying how URLs found on pages should be handled. \n- UNKNOWN: Default unknown value\n- SCOPED: Only include the base URL.\n- PATH: Crawl the base URL and linked pages within the URL path.\n- DOMAIN: Crawl the base URL and linked pages within the same domain.\n- SUBDOMAINS: Crawl the base URL and linked pages for any subdomain."]
    pub fn crawling_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crawling_option", self.base))
    }

    #[doc = "Get a reference to the value of field `embed_media` after provisioning.\nWhether to embed media content"]
    pub fn embed_media(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embed_media", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgentPlatformKnowledgeBase_DatasourcesDynamic {
    file_upload_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<AgentPlatformKnowledgeBase_Datasources_FileUploadDataSource>>,
    last_indexing_job: std::option::Option<terrars::DynamicBlock<AgentPlatformKnowledgeBase_LastIndexingJob>>,
    spaces_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<AgentPlatformKnowledgeBase_Datasources_SpacesDataSource>>,
    web_crawler_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<AgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSource>>,
}

#[derive(Serialize)]
pub struct AgentPlatformKnowledgeBase_Datasources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_upload_data_source: std
    ::option
    ::Option<std::vec::Vec<AgentPlatformKnowledgeBase_Datasources_FileUploadDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_indexing_job: std::option::Option<std::vec::Vec<AgentPlatformKnowledgeBase_LastIndexingJob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spaces_data_source: std::option::Option<std::vec::Vec<AgentPlatformKnowledgeBase_Datasources_SpacesDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_crawler_data_source: std
    ::option
    ::Option<std::vec::Vec<AgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSource>>,
    dynamic: AgentPlatformKnowledgeBase_DatasourcesDynamic,
}

impl AgentPlatformKnowledgeBase_Datasources {
    #[doc = "Set the field `uuid`.\nUUID of the Knowledge Base"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_upload_data_source`.\n"]
    pub fn set_file_upload_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AgentPlatformKnowledgeBase_Datasources_FileUploadDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file_upload_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file_upload_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `last_indexing_job`.\n"]
    pub fn set_last_indexing_job(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgentPlatformKnowledgeBase_LastIndexingJob>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.last_indexing_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.last_indexing_job = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spaces_data_source`.\n"]
    pub fn set_spaces_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AgentPlatformKnowledgeBase_Datasources_SpacesDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spaces_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spaces_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_crawler_data_source`.\n"]
    pub fn set_web_crawler_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.web_crawler_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.web_crawler_data_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AgentPlatformKnowledgeBase_Datasources {
    type O = terrars::BlockAssignable<AgentPlatformKnowledgeBase_Datasources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentPlatformKnowledgeBase_Datasources {}

impl BuildAgentPlatformKnowledgeBase_Datasources {
    pub fn build(self) -> AgentPlatformKnowledgeBase_Datasources {
        AgentPlatformKnowledgeBase_Datasources {
            uuid: core::default::Default::default(),
            file_upload_data_source: core::default::Default::default(),
            last_indexing_job: core::default::Default::default(),
            spaces_data_source: core::default::Default::default(),
            web_crawler_data_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentPlatformKnowledgeBase_DatasourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformKnowledgeBase_DatasourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AgentPlatformKnowledgeBase_DatasourcesRef {
        AgentPlatformKnowledgeBase_DatasourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentPlatformKnowledgeBase_DatasourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nCreated At timestamp for the Knowledge Base"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the Knowledge Base was updated"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nUUID of the Knowledge Base"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `file_upload_data_source` after provisioning.\n"]
    pub fn file_upload_data_source(
        &self,
    ) -> terrars::ListRef<AgentPlatformKnowledgeBase_Datasources_FileUploadDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_upload_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `last_indexing_job` after provisioning.\n"]
    pub fn last_indexing_job(&self) -> terrars::ListRef<AgentPlatformKnowledgeBase_LastIndexingJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_indexing_job", self.base))
    }

    #[doc = "Get a reference to the value of field `spaces_data_source` after provisioning.\n"]
    pub fn spaces_data_source(&self) -> terrars::ListRef<AgentPlatformKnowledgeBase_Datasources_SpacesDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spaces_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `web_crawler_data_source` after provisioning.\n"]
    pub fn web_crawler_data_source(
        &self,
    ) -> terrars::ListRef<AgentPlatformKnowledgeBase_Datasources_WebCrawlerDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_crawler_data_source", self.base))
    }
}
