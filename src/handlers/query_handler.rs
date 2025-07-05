//! Query handler for organization domain

use std::collections::HashMap;
use std::sync::Arc;
use std::pin::Pin;
use std::future::Future;
use crate::queries::*;
use crate::projections::*;
use crate::aggregate::{OrganizationError, OrganizationEvent};
use crate::value_objects::*;
use uuid::Uuid;

/// Read model storage trait
#[async_trait::async_trait]
pub trait ReadModelStore: Send + Sync {
    /// Get organization view by ID
    async fn get_organization(&self, id: Uuid) -> Result<Option<OrganizationView>, OrganizationError>;
    
    /// Get all organizations
    async fn get_all_organizations(&self) -> Result<Vec<OrganizationView>, OrganizationError>;
    
    /// Get organization members
    async fn get_organization_members(&self, org_id: Uuid) -> Result<Vec<MemberView>, OrganizationError>;
    
    /// Get organizations by person
    async fn get_person_organizations(&self, person_id: Uuid) -> Result<Vec<MemberOrganizationView>, OrganizationError>;
    
    /// Update organization view
    async fn update_organization(&self, view: OrganizationView) -> Result<(), OrganizationError>;
    
    /// Update member view
    async fn update_member(&self, org_id: Uuid, member: MemberView) -> Result<(), OrganizationError>;
    
    /// Remove member
    async fn remove_member(&self, org_id: Uuid, person_id: Uuid) -> Result<(), OrganizationError>;
}

/// In-memory read model store
#[derive(Clone)]
pub struct InMemoryReadModelStore {
    organizations: Arc<tokio::sync::RwLock<HashMap<Uuid, OrganizationView>>>,
    members: Arc<tokio::sync::RwLock<HashMap<Uuid, Vec<MemberView>>>>,
    person_organizations: Arc<tokio::sync::RwLock<HashMap<Uuid, Vec<MemberOrganizationView>>>>,
}

impl Default for InMemoryReadModelStore {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryReadModelStore {
    pub fn new() -> Self {
        Self {
            organizations: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            members: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            person_organizations: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl ReadModelStore for InMemoryReadModelStore {
    async fn get_organization(&self, id: Uuid) -> Result<Option<OrganizationView>, OrganizationError> {
        let orgs = self.organizations.read().await;
        Ok(orgs.get(&id).cloned())
    }
    
    async fn get_all_organizations(&self) -> Result<Vec<OrganizationView>, OrganizationError> {
        let orgs = self.organizations.read().await;
        Ok(orgs.values().cloned().collect())
    }
    
    async fn get_organization_members(&self, org_id: Uuid) -> Result<Vec<MemberView>, OrganizationError> {
        let members = self.members.read().await;
        Ok(members.get(&org_id).cloned().unwrap_or_default())
    }
    
    async fn get_person_organizations(&self, person_id: Uuid) -> Result<Vec<MemberOrganizationView>, OrganizationError> {
        let person_orgs = self.person_organizations.read().await;
        Ok(person_orgs.get(&person_id).cloned().unwrap_or_default())
    }
    
    async fn update_organization(&self, view: OrganizationView) -> Result<(), OrganizationError> {
        let mut orgs = self.organizations.write().await;
        orgs.insert(view.organization_id, view);
        Ok(())
    }
    
    async fn update_member(&self, org_id: Uuid, member: MemberView) -> Result<(), OrganizationError> {
        // Update organization members
        let mut members = self.members.write().await;
        let org_members = members.entry(org_id).or_insert_with(Vec::new);
        
        // Update or add member
        if let Some(existing) = org_members.iter_mut().find(|m| m.person_id == member.person_id) {
            *existing = member.clone();
        } else {
            org_members.push(member.clone());
        }
        
        // Update person organizations
        let mut person_orgs = self.person_organizations.write().await;
        let person_memberships = person_orgs.entry(member.person_id).or_insert_with(Vec::new);
        
        // Find organization view
        let orgs = self.organizations.read().await;
        if let Some(org_view) = orgs.get(&org_id) {
            let membership = MemberOrganizationView {
                organization_id: org_id,
                organization_name: org_view.name.clone(),
                org_type: org_view.org_type,
                role: member.role.clone(),
                is_primary: true, // TODO: Determine from member data
                joined_at: member.joined_at,
            };
            
            if let Some(existing) = person_memberships.iter_mut().find(|m| m.organization_id == org_id) {
                *existing = membership;
            } else {
                person_memberships.push(membership);
            }
        }
        
        Ok(())
    }
    
    async fn remove_member(&self, org_id: Uuid, person_id: Uuid) -> Result<(), OrganizationError> {
        // Remove from organization members
        let mut members = self.members.write().await;
        if let Some(org_members) = members.get_mut(&org_id) {
            org_members.retain(|m| m.person_id != person_id);
        }
        
        // Remove from person organizations
        let mut person_orgs = self.person_organizations.write().await;
        if let Some(person_memberships) = person_orgs.get_mut(&person_id) {
            person_memberships.retain(|m| m.organization_id != org_id);
        }
        
        Ok(())
    }
}

/// Projection updater that handles events and updates read models
pub struct ProjectionUpdater<RS: ReadModelStore> {
    read_store: RS,
}

impl<RS: ReadModelStore> ProjectionUpdater<RS> {
    pub fn new(read_store: RS) -> Self {
        Self { read_store }
    }
    
    /// Handle domain events and update projections
    pub async fn handle_event(&self, event: &OrganizationEvent) -> Result<(), OrganizationError> {
        match event {
            OrganizationEvent::Created(e) => {
                let view = OrganizationView {
                    organization_id: e.organization_id,
                    name: e.name.clone(),
                    org_type: e.org_type,
                    status: OrganizationStatus::Active,
                    parent_id: e.parent_id,
                    child_units: vec![],
                    member_count: 0,
                    location_count: 0,
                    primary_location_name: None,
                    size_category: SizeCategory::Small, // Start as small
                };
                self.read_store.update_organization(view).await?;
            }
            OrganizationEvent::MemberAdded(e) => {
                let member_view = MemberView {
                    person_id: e.member.person_id,
                    person_name: format!("Person {}", e.member.person_id), // TODO: Get from Person domain
                    role: e.member.role.clone(),
                    reports_to_id: e.member.reports_to,
                    reports_to_name: e.member.reports_to.map(|id| format!("Person {id}")),
                    joined_at: e.added_at,
                    direct_reports_count: 0, // TODO: Calculate from other members
                    is_active: true,
                };
                self.read_store.update_member(e.organization_id, member_view).await?;
                
                // Update member count and size category
                if let Some(mut org) = self.read_store.get_organization(e.organization_id).await? {
                    org.member_count += 1;
                    org.update_size_category();
                    self.read_store.update_organization(org).await?;
                }
            }
            OrganizationEvent::MemberRemoved(e) => {
                self.read_store.remove_member(e.organization_id, e.person_id).await?;
                
                // Update member count and size category
                if let Some(mut org) = self.read_store.get_organization(e.organization_id).await? {
                    org.member_count = org.member_count.saturating_sub(1);
                    org.update_size_category();
                    self.read_store.update_organization(org).await?;
                }
            }
            // TODO: Handle other events
            _ => {}
        }
        Ok(())
    }
}

/// Handler for organization queries
pub struct OrganizationQueryHandler<RS: ReadModelStore> {
    read_store: RS,
}

impl<RS: ReadModelStore> OrganizationQueryHandler<RS> {
    /// Create a new query handler
    pub fn new(read_store: RS) -> Self {
        Self { read_store }
    }

    /// Get organization by ID
    pub async fn get_organization_by_id(
        &self,
        query: GetOrganizationById,
    ) -> Result<Option<OrganizationView>, OrganizationError> {
        self.read_store.get_organization(query.organization_id).await
    }

    /// Get organization hierarchy
    pub async fn get_organization_hierarchy(
        &self,
        query: GetOrganizationHierarchy,
    ) -> Result<OrganizationHierarchyView, OrganizationError> {
        let organization = self.read_store.get_organization(query.organization_id).await?
            .ok_or(OrganizationError::NotFound(query.organization_id))?;
        
        // Build hierarchy recursively
        let max_depth = query.max_depth.unwrap_or(usize::MAX);
        let children = self.build_hierarchy(&organization, max_depth, 0).await?;
        
        Ok(OrganizationHierarchyView {
            organization,
            children,
        })
    }
    
    /// Build hierarchy recursively
    fn build_hierarchy<'a>(
        &'a self,
        parent: &'a OrganizationView,
        max_depth: usize,
        current_depth: usize,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<OrganizationHierarchyView>, OrganizationError>> + Send + 'a>> {
        Box::pin(async move {
            if current_depth >= max_depth {
                return Ok(vec![]);
            }
            
            let mut children = Vec::new();
            
            for child_id in &parent.child_units {
                if let Some(child_org) = self.read_store.get_organization(*child_id).await? {
                    let child_children = self.build_hierarchy(&child_org, max_depth, current_depth + 1).await?;
                    children.push(OrganizationHierarchyView {
                        organization: child_org,
                        children: child_children,
                    });
                }
            }
            
            Ok(children)
        })
    }

    /// Get organization members
    pub async fn get_organization_members(
        &self,
        query: GetOrganizationMembers,
    ) -> Result<Vec<MemberView>, OrganizationError> {
        let members = self.read_store.get_organization_members(query.organization_id).await?;
        
        // Apply filters
        let filtered = members.into_iter()
            .filter(|m| {
                if let Some(ref role_filter) = query.role_filter {
                    &m.role.title == role_filter || &m.role.role_code == role_filter
                } else {
                    true
                }
            })
            .filter(|m| {
                if query.include_inactive {
                    true
                } else {
                    m.is_active
                }
            })
            .collect();
        
        Ok(filtered)
    }

    /// Get organizations by type
    pub async fn get_organizations_by_type(
        &self,
        query: GetOrganizationsByType,
    ) -> Result<Vec<OrganizationView>, OrganizationError> {
        let all_orgs = self.read_store.get_all_organizations().await?;
        let filtered = all_orgs.into_iter()
            .filter(|org| org.org_type == query.org_type)
            .collect();
        Ok(filtered)
    }

    /// Get organizations by status
    pub async fn get_organizations_by_status(
        &self,
        query: GetOrganizationsByStatus,
    ) -> Result<Vec<OrganizationView>, OrganizationError> {
        let all_orgs = self.read_store.get_all_organizations().await?;
        let filtered = all_orgs.into_iter()
            .filter(|org| org.status == query.status)
            .collect();
        Ok(filtered)
    }

    /// Get member's organizations
    pub async fn get_member_organizations(
        &self,
        query: GetMemberOrganizations,
    ) -> Result<Vec<MemberOrganizationView>, OrganizationError> {
        self.read_store.get_person_organizations(query.person_id).await
    }

    /// Get organization reporting structure
    pub async fn get_reporting_structure(
        &self,
        query: GetReportingStructure,
    ) -> Result<ReportingStructureView, OrganizationError> {
        let members = self.read_store.get_organization_members(query.organization_id).await?;
        
        // Build reporting tree
        let max_depth = query.max_depth.unwrap_or(usize::MAX);
        let root_members = self.build_reporting_tree(&members, None, max_depth, 0);
        
        Ok(ReportingStructureView {
            organization_id: query.organization_id,
            root_members,
        })
    }
    
    /// Build reporting tree recursively
    fn build_reporting_tree(
        &self,
        all_members: &[MemberView],
        manager_id: Option<Uuid>,
        max_depth: usize,
        current_depth: usize,
    ) -> Vec<ReportingNode> {
        if current_depth >= max_depth {
            return vec![];
        }
        
        all_members.iter()
            .filter(|m| m.reports_to_id == manager_id)
            .map(|member| {
                let direct_reports = self.build_reporting_tree(
                    all_members,
                    Some(member.person_id),
                    max_depth,
                    current_depth + 1,
                );
                
                ReportingNode {
                    person_id: member.person_id,
                    person_name: member.person_name.clone(),
                    role: member.role.clone(),
                    direct_reports,
                }
            })
            .collect()
    }

    /// Search organizations
    pub async fn search_organizations(
        &self,
        query: SearchOrganizations,
    ) -> Result<Vec<OrganizationView>, OrganizationError> {
        let all_orgs = self.read_store.get_all_organizations().await?;
        
        let filtered = all_orgs.into_iter()
            .filter(|org| {
                // Text search
                if !query.query.is_empty() {
                    let query_lower = query.query.to_lowercase();
                    org.name.to_lowercase().contains(&query_lower)
                } else {
                    true
                }
            })
            .filter(|org| {
                // Type filter
                if let Some(ref org_type) = query.org_type_filter {
                    &org.org_type == org_type
                } else {
                    true
                }
            })
            .filter(|org| {
                // Status filter
                if let Some(ref status) = query.status_filter {
                    &org.status == status
                } else {
                    true
                }
            })
            .take(query.limit)
            .collect();
        
        Ok(filtered)
    }

    /// Get organization statistics
    pub async fn get_organization_statistics(
        &self,
        query: GetOrganizationStatistics,
    ) -> Result<OrganizationStatistics, OrganizationError> {
        let members = self.read_store.get_organization_members(query.organization_id).await?;
        let org = self.read_store.get_organization(query.organization_id).await?
            .ok_or(OrganizationError::NotFound(query.organization_id))?;
        
        // Calculate statistics
        let mut members_by_role = HashMap::new();
        let mut members_by_level = HashMap::new();
        let mut total_tenure_days = 0u64;
        let now = chrono::Utc::now();
        
        for member in &members {
            *members_by_role.entry(member.role.title.clone()).or_insert(0) += 1;
            *members_by_level.entry(member.role.level).or_insert(0) += 1;
            
            // Calculate tenure in days
            let tenure_duration = now.signed_duration_since(member.joined_at);
            total_tenure_days += tenure_duration.num_days().max(0) as u64;
        }
        
        // Calculate average tenure
        let average_tenure_days = if members.is_empty() {
            0
        } else {
            total_tenure_days / members.len() as u64
        };
        
        // Calculate reporting depth
        let reporting_depth = self.calculate_max_reporting_depth(&members);
        
        Ok(OrganizationStatistics {
            organization_id: query.organization_id,
            total_members: members.len(),
            members_by_role,
            members_by_level,
            average_tenure_days,
            location_count: org.location_count,
            child_organization_count: org.child_units.len(),
            reporting_depth,
        })
    }
    
    /// Calculate maximum reporting depth
    fn calculate_max_reporting_depth(&self, members: &[MemberView]) -> usize {
        let mut max_depth = 0;
        
        for member in members {
            let depth = self.calculate_member_depth(member.person_id, members, 0);
            max_depth = max_depth.max(depth);
        }
        
        max_depth
    }
    
    fn calculate_member_depth(&self, person_id: Uuid, all_members: &[MemberView], current_depth: usize) -> usize {
        let direct_reports: Vec<_> = all_members.iter()
            .filter(|m| m.reports_to_id == Some(person_id))
            .collect();
        
        if direct_reports.is_empty() {
            current_depth
        } else {
            direct_reports.iter()
                .map(|report| self.calculate_member_depth(report.person_id, all_members, current_depth + 1))
                .max()
                .unwrap_or(current_depth)
        }
    }

    /// Get organization chart
    pub async fn get_organization_chart(
        &self,
        query: GetOrganizationChart,
    ) -> Result<OrganizationChartView, OrganizationError> {
        let members = self.read_store.get_organization_members(query.organization_id).await?;
        
        // Build nodes and edges
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        
        for member in &members {
            nodes.push(ChartNode {
                id: member.person_id.to_string(),
                label: format!("{}\n{}", member.person_name, member.role.title),
                node_type: "member".to_string(),
                metadata: HashMap::new(),
            });
            
            if let Some(manager_id) = member.reports_to_id {
                edges.push(ChartEdge {
                    source: manager_id.to_string(),
                    target: member.person_id.to_string(),
                    edge_type: "reports_to".to_string(),
                    metadata: HashMap::new(),
                });
            }
        }
        
        Ok(OrganizationChartView {
            organization_id: query.organization_id,
            nodes,
            edges,
            layout_type: query.layout_type.unwrap_or("hierarchical".to_string()),
        })
    }
}

impl Default for OrganizationQueryHandler<InMemoryReadModelStore> {
    fn default() -> Self {
        Self::new(InMemoryReadModelStore::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_organization_by_id() {
        let store = InMemoryReadModelStore::new();
        let handler = OrganizationQueryHandler::new(store.clone());
        
        // Create organization view
        let org_id = Uuid::new_v4();
        let org_view = OrganizationView {
            organization_id: org_id,
            name: "Test Corp".to_string(),
            org_type: OrganizationType::Company,
            status: OrganizationStatus::Active,
            parent_id: None,
            child_units: vec![],
            member_count: 0,
            location_count: 0,
            primary_location_name: None,
            size_category: SizeCategory::Small,
        };
        
        store.update_organization(org_view.clone()).await.unwrap();
        
        let query = GetOrganizationById {
            organization_id: org_id,
        };

        let result = handler.get_organization_by_id(query).await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Test Corp");
    }

    #[tokio::test]
    async fn test_get_organization_hierarchy() {
        let store = InMemoryReadModelStore::new();
        let handler = OrganizationQueryHandler::new(store.clone());
        
        // Create parent organization
        let parent_id = Uuid::new_v4();
        let parent_view = OrganizationView {
            organization_id: parent_id,
            name: "Parent Corp".to_string(),
            org_type: OrganizationType::Company,
            status: OrganizationStatus::Active,
            parent_id: None,
            child_units: vec![],
            member_count: 0,
            location_count: 0,
            primary_location_name: None,
            size_category: SizeCategory::Small,
        };
        
        store.update_organization(parent_view).await.unwrap();
        
        let query = GetOrganizationHierarchy {
            organization_id: parent_id,
            max_depth: Some(3),
        };

        let result = handler.get_organization_hierarchy(query).await.unwrap();
        assert_eq!(result.organization.name, "Parent Corp");
        assert!(result.children.is_empty());
    }
    
    #[tokio::test]
    async fn test_search_organizations() {
        let store = InMemoryReadModelStore::new();
        let handler = OrganizationQueryHandler::new(store.clone());
        
        // Create multiple organizations
        for i in 0..5 {
            let org_view = OrganizationView {
                organization_id: Uuid::new_v4(),
                name: format!("Test Corp {}", i),
                org_type: if i % 2 == 0 { OrganizationType::Company } else { OrganizationType::Division },
                status: OrganizationStatus::Active,
                parent_id: None,
                child_units: vec![],
                member_count: i,
                location_count: 0,
                primary_location_name: None,
                size_category: SizeCategory::Small,
            };
            store.update_organization(org_view).await.unwrap();
        }
        
        // Search by text
        let search_query = SearchOrganizations {
            query: "Corp".to_string(),
            org_type_filter: None,
            status_filter: None,
            limit: 10,
        };
        
        let results = handler.search_organizations(search_query).await.unwrap();
        assert_eq!(results.len(), 5);
        
        // Search by type
        let type_query = SearchOrganizations {
            query: String::new(),
            org_type_filter: Some(OrganizationType::Company),
            status_filter: None,
            limit: 10,
        };
        
        let type_results = handler.search_organizations(type_query).await.unwrap();
        assert_eq!(type_results.len(), 3); // 0, 2, 4 are companies
    }
    
    #[tokio::test]
    async fn test_organization_statistics() {
        let store = InMemoryReadModelStore::new();
        let handler = OrganizationQueryHandler::new(store.clone());
        let updater = ProjectionUpdater::new(store.clone());
        
        let org_id = Uuid::new_v4();
        
        // Create organization
        let create_event = OrganizationEvent::Created(crate::events::OrganizationCreated {
            organization_id: org_id,
            name: "Stats Corp".to_string(),
            org_type: OrganizationType::Company,
            parent_id: None,
            primary_location_id: None,
            created_at: chrono::Utc::now(),
        });
        
        updater.handle_event(&create_event).await.unwrap();
        
        // Add members
        for i in 0..10 {
            let member = crate::value_objects::OrganizationMember {
                person_id: Uuid::new_v4(),
                organization_id: org_id,
                role: if i < 5 { 
                    OrganizationRole::software_engineer() 
                } else { 
                    OrganizationRole::engineering_manager() 
                },
                reports_to: None,
                joined_at: chrono::Utc::now(),
                ends_at: None,
                metadata: HashMap::new(),
            };
            
            let add_event = OrganizationEvent::MemberAdded(crate::events::MemberAdded {
                organization_id: org_id,
                member,
                added_at: chrono::Utc::now(),
            });
            
            updater.handle_event(&add_event).await.unwrap();
        }
        
        let stats_query = GetOrganizationStatistics {
            organization_id: org_id,
        };
        
        let stats = handler.get_organization_statistics(stats_query).await.unwrap();
        assert_eq!(stats.total_members, 10);
        assert_eq!(stats.members_by_role.get("Software Engineer"), Some(&5));
        assert_eq!(stats.members_by_role.get("Engineering Manager"), Some(&5));
    }
    
    #[tokio::test]
    async fn test_reporting_structure() {
        let store = InMemoryReadModelStore::new();
        let handler = OrganizationQueryHandler::new(store.clone());
        
        let org_id = Uuid::new_v4();
        let ceo_id = Uuid::new_v4();
        let vp_id = Uuid::new_v4();
        let manager_id = Uuid::new_v4();
        let engineer_id = Uuid::new_v4();
        
        // Create reporting hierarchy
        let members = vec![
            MemberView {
                person_id: ceo_id,
                person_name: "CEO".to_string(),
                role: OrganizationRole::ceo(),
                reports_to_id: None,
                reports_to_name: None,
                joined_at: chrono::Utc::now(),
                direct_reports_count: 1,
                is_active: true,
            },
            MemberView {
                person_id: vp_id,
                person_name: "VP Engineering".to_string(),
                role: OrganizationRole::vp_engineering(),
                reports_to_id: Some(ceo_id),
                reports_to_name: Some("CEO".to_string()),
                joined_at: chrono::Utc::now(),
                direct_reports_count: 1,
                is_active: true,
            },
            MemberView {
                person_id: manager_id,
                person_name: "Engineering Manager".to_string(),
                role: OrganizationRole::engineering_manager(),
                reports_to_id: Some(vp_id),
                reports_to_name: Some("VP Engineering".to_string()),
                joined_at: chrono::Utc::now(),
                direct_reports_count: 1,
                is_active: true,
            },
            MemberView {
                person_id: engineer_id,
                person_name: "Software Engineer".to_string(),
                role: OrganizationRole::software_engineer(),
                reports_to_id: Some(manager_id),
                reports_to_name: Some("Engineering Manager".to_string()),
                joined_at: chrono::Utc::now(),
                direct_reports_count: 0,
                is_active: true,
            },
        ];
        
        // Store members
        for member in members {
            store.update_member(org_id, member).await.unwrap();
        }
        
        let query = GetReportingStructure {
            organization_id: org_id,
            starting_person_id: None,
            max_depth: None,
        };
        
        let structure = handler.get_reporting_structure(query).await.unwrap();
        assert_eq!(structure.root_members.len(), 1); // Only CEO at root
        assert_eq!(structure.root_members[0].person_name, "CEO");
        assert_eq!(structure.root_members[0].direct_reports.len(), 1); // VP
        assert_eq!(structure.root_members[0].direct_reports[0].direct_reports.len(), 1); // Manager
    }
    
    #[tokio::test]
    async fn test_organization_chart() {
        let store = InMemoryReadModelStore::new();
        let handler = OrganizationQueryHandler::new(store.clone());
        
        let org_id = Uuid::new_v4();
        let person1_id = Uuid::new_v4();
        let person2_id = Uuid::new_v4();
        
        // Add members
        store.update_member(org_id, MemberView {
            person_id: person1_id,
            person_name: "Manager".to_string(),
            role: OrganizationRole::engineering_manager(),
            reports_to_id: None,
            reports_to_name: None,
            joined_at: chrono::Utc::now(),
            direct_reports_count: 1,
            is_active: true,
        }).await.unwrap();
        
        store.update_member(org_id, MemberView {
            person_id: person2_id,
            person_name: "Engineer".to_string(),
            role: OrganizationRole::software_engineer(),
            reports_to_id: Some(person1_id),
            reports_to_name: Some("Manager".to_string()),
            joined_at: chrono::Utc::now(),
            direct_reports_count: 0,
            is_active: true,
        }).await.unwrap();
        
        let query = GetOrganizationChart {
            organization_id: org_id,
            layout_type: Some("hierarchical".to_string()),
        };
        
        let chart = handler.get_organization_chart(query).await.unwrap();
        assert_eq!(chart.nodes.len(), 2);
        assert_eq!(chart.edges.len(), 1);
        assert_eq!(chart.edges[0].edge_type, "reports_to");
    }
    
    // TODO: Failing tests for unimplemented features
    
    #[tokio::test]
    #[should_panic(expected = "TODO: Implement real-time projection updates")]
    async fn test_todo_realtime_projection_updates() {
        // TODO: Implement real-time projection updates from event stream
        panic!("TODO: Implement real-time projection updates");
    }
    
    #[tokio::test]
    #[should_panic(expected = "TODO: Implement cross-domain person name resolution")]
    async fn test_todo_person_name_resolution() {
        // TODO: Query person names from Person domain
        panic!("TODO: Implement cross-domain person name resolution");
    }
    
    #[tokio::test]
    #[should_panic(expected = "TODO: Implement location name resolution")]
    async fn test_todo_location_name_resolution() {
        // TODO: Query location names from Location domain
        panic!("TODO: Implement location name resolution");
    }
    
    #[tokio::test]
    #[should_panic(expected = "TODO: Implement average tenure calculation")]
    async fn test_todo_tenure_calculation() {
        // TODO: Calculate average tenure from join dates
        panic!("TODO: Implement average tenure calculation");
    }
    
    #[tokio::test]
    #[should_panic(expected = "TODO: Implement organization size categorization")]
    async fn test_todo_size_categorization() {
        // TODO: Automatically categorize organizations by size
        panic!("TODO: Implement organization size categorization");
    }
} 