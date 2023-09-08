// GENERATED CODE

use crate::api_default_imports::*;
use crate::role_management::*;

resource_api_client!(RoleManagementApiClient, ResourceIdentity::RoleManagement);

impl RoleManagementApiClient {api_client_link!(directory, RoleManagementDirectoryApiClient);

	get!(
		doc: "Get roleManagement", 
		name: get_role_management,
		path: "/roleManagement"
	);
	patch!(
		doc: "Update roleManagement", 
		name: update_role_management,
		path: "/roleManagement",
		body: true
	);
}
