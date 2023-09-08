// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(RoleManagementDirectoryApiClient, ResourceIdentity::RoleManagementDirectory);

impl RoleManagementDirectoryApiClient {
	delete!(
		doc: "Delete navigation property directory for roleManagement", 
		name: delete_directory,
		path: "/directory"
	);
	get!(
		doc: "Get directory from roleManagement", 
		name: get_directory,
		path: "/directory"
	);
	patch!(
		doc: "Update the navigation property directory in roleManagement", 
		name: update_directory,
		path: "/directory",
		body: true
	);
	post!(
		doc: "Create new navigation property to resourceNamespaces for roleManagement", 
		name: create_resource_namespaces,
		path: "/directory/resourceNamespaces",
		body: true
	);
	get!(
		doc: "Get resourceNamespaces from roleManagement", 
		name: list_resource_namespaces,
		path: "/directory/resourceNamespaces"
	);
	get!(
		doc: "Get the number of the resource", 
		name: resource_namespaces,
		path: "/directory/resourceNamespaces/$count"
	);
	delete!(
		doc: "Delete navigation property resourceNamespaces for roleManagement", 
		name: delete_resource_namespaces,
		path: "/directory/resourceNamespaces/{{id}}",
		params: unified_rbac_resource_namespace_id
	);
	get!(
		doc: "Get resourceNamespaces from roleManagement", 
		name: get_resource_namespaces,
		path: "/directory/resourceNamespaces/{{id}}",
		params: unified_rbac_resource_namespace_id
	);
	patch!(
		doc: "Update the navigation property resourceNamespaces in roleManagement", 
		name: update_resource_namespaces,
		path: "/directory/resourceNamespaces/{{id}}",
		body: true,
		params: unified_rbac_resource_namespace_id
	);
	post!(
		doc: "Create new navigation property to resourceActions for roleManagement", 
		name: create_resource_actions,
		path: "/directory/resourceNamespaces/{{id}}/resourceActions",
		body: true,
		params: unified_rbac_resource_namespace_id
	);
	get!(
		doc: "Get resourceActions from roleManagement", 
		name: list_resource_actions,
		path: "/directory/resourceNamespaces/{{id}}/resourceActions",
		params: unified_rbac_resource_namespace_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: resource_actions,
		path: "/directory/resourceNamespaces/{{id}}/resourceActions/$count",
		params: unified_rbac_resource_namespace_id
	);
	delete!(
		doc: "Delete navigation property resourceActions for roleManagement", 
		name: delete_resource_actions,
		path: "/directory/resourceNamespaces/{{id}}/resourceActions/{{id2}}",
		params: unified_rbac_resource_namespace_id, unified_rbac_resource_action_id
	);
	get!(
		doc: "Get resourceActions from roleManagement", 
		name: get_resource_actions,
		path: "/directory/resourceNamespaces/{{id}}/resourceActions/{{id2}}",
		params: unified_rbac_resource_namespace_id, unified_rbac_resource_action_id
	);
	patch!(
		doc: "Update the navigation property resourceActions in roleManagement", 
		name: update_resource_actions,
		path: "/directory/resourceNamespaces/{{id}}/resourceActions/{{id2}}",
		body: true,
		params: unified_rbac_resource_namespace_id, unified_rbac_resource_action_id
	);
	post!(
		doc: "Create new navigation property to roleAssignmentScheduleInstances for roleManagement", 
		name: create_role_assignment_schedule_instances,
		path: "/directory/roleAssignmentScheduleInstances",
		body: true
	);
	get!(
		doc: "List roleAssignmentScheduleInstances", 
		name: list_role_assignment_schedule_instances,
		path: "/directory/roleAssignmentScheduleInstances"
	);
	get!(
		doc: "Get the number of the resource", 
		name: role_assignment_schedule_instances,
		path: "/directory/roleAssignmentScheduleInstances/$count"
	);
	get!(
		doc: "Invoke function filterByCurrentUser", 
		name: filter_by_current_user,
		path: "/directory/roleAssignmentScheduleInstances/filterByCurrentUser(on='{{id}}')",
		params: on
	);
	delete!(
		doc: "Delete navigation property roleAssignmentScheduleInstances for roleManagement", 
		name: delete_role_assignment_schedule_instances,
		path: "/directory/roleAssignmentScheduleInstances/{{id}}",
		params: unified_role_assignment_schedule_instance_id
	);
	get!(
		doc: "Get unifiedRoleAssignmentScheduleInstance", 
		name: get_role_assignment_schedule_instances,
		path: "/directory/roleAssignmentScheduleInstances/{{id}}",
		params: unified_role_assignment_schedule_instance_id
	);
	patch!(
		doc: "Update the navigation property roleAssignmentScheduleInstances in roleManagement", 
		name: update_role_assignment_schedule_instances,
		path: "/directory/roleAssignmentScheduleInstances/{{id}}",
		body: true,
		params: unified_role_assignment_schedule_instance_id
	);
	get!(
		doc: "Get activatedUsing from roleManagement", 
		name: get_activated_using,
		path: "/directory/roleAssignmentScheduleInstances/{{id}}/activatedUsing",
		params: unified_role_assignment_schedule_instance_id
	);
	get!(
		doc: "Get appScope from roleManagement", 
		name: get_app_scope,
		path: "/directory/roleAssignmentScheduleInstances/{{id}}/appScope",
		params: unified_role_assignment_schedule_instance_id
	);
	get!(
		doc: "Get directoryScope from roleManagement", 
		name: get_directory_scope,
		path: "/directory/roleAssignmentScheduleInstances/{{id}}/directoryScope",
		params: unified_role_assignment_schedule_instance_id
	);
	get!(
		doc: "Get principal from roleManagement", 
		name: get_principal,
		path: "/directory/roleAssignmentScheduleInstances/{{id}}/principal",
		params: unified_role_assignment_schedule_instance_id
	);
	get!(
		doc: "Get roleDefinition from roleManagement", 
		name: get_role_definition,
		path: "/directory/roleAssignmentScheduleInstances/{{id}}/roleDefinition",
		params: unified_role_assignment_schedule_instance_id
	);
	post!(
		doc: "Create roleDefinitions", 
		name: create_role_definitions,
		path: "/directory/roleDefinitions",
		body: true
	);
	get!(
		doc: "List roleDefinitions", 
		name: list_role_definitions,
		path: "/directory/roleDefinitions"
	);
	get!(
		doc: "Get the number of the resource", 
		name: role_definitions,
		path: "/directory/roleDefinitions/$count"
	);
	delete!(
		doc: "Delete unifiedRoleDefinition", 
		name: delete_role_definitions,
		path: "/directory/roleDefinitions/{{id}}",
		params: unified_role_definition_id
	);
	get!(
		doc: "Get unifiedRoleDefinition", 
		name: get_role_definitions,
		path: "/directory/roleDefinitions/{{id}}",
		params: unified_role_definition_id
	);
	patch!(
		doc: "Update unifiedRoleDefinition", 
		name: update_role_definitions,
		path: "/directory/roleDefinitions/{{id}}",
		body: true,
		params: unified_role_definition_id
	);
	post!(
		doc: "Create new navigation property to inheritsPermissionsFrom for roleManagement", 
		name: create_inherits_permissions_from,
		path: "/directory/roleDefinitions/{{id}}/inheritsPermissionsFrom",
		body: true,
		params: unified_role_definition_id
	);
	get!(
		doc: "Get inheritsPermissionsFrom from roleManagement", 
		name: list_inherits_permissions_from,
		path: "/directory/roleDefinitions/{{id}}/inheritsPermissionsFrom",
		params: unified_role_definition_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: inherits_permissions_from,
		path: "/directory/roleDefinitions/{{id}}/inheritsPermissionsFrom/$count",
		params: unified_role_definition_id
	);
	delete!(
		doc: "Delete navigation property inheritsPermissionsFrom for roleManagement", 
		name: delete_inherits_permissions_from,
		path: "/directory/roleDefinitions/{{id}}/inheritsPermissionsFrom/{{id2}}",
		params: unified_role_definition_id, unified_role_definition_id_1
	);
	get!(
		doc: "Get inheritsPermissionsFrom from roleManagement", 
		name: get_inherits_permissions_from,
		path: "/directory/roleDefinitions/{{id}}/inheritsPermissionsFrom/{{id2}}",
		params: unified_role_definition_id, unified_role_definition_id_1
	);
	patch!(
		doc: "Update the navigation property inheritsPermissionsFrom in roleManagement", 
		name: update_inherits_permissions_from,
		path: "/directory/roleDefinitions/{{id}}/inheritsPermissionsFrom/{{id2}}",
		body: true,
		params: unified_role_definition_id, unified_role_definition_id_1
	);
}
