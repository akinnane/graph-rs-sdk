// GENERATED CODE

use crate::api_default_imports::*;
use crate::security::*;

resource_api_client!(SecurityApiClient, ResourceIdentity::Security);

impl SecurityApiClient {
	get!(
		doc: "Get security", 
		name: get_security,
		path: "/security"
	);
	patch!(
		doc: "Update security", 
		name: update_security,
		path: "/security",
		body: true
	);
	post!(
		doc: "Create new navigation property to secureScores for security", 
		name: create_secure_scores,
		path: "/security/secureScores",
		body: true
	);
	get!(
		doc: "List secureScores", 
		name: list_secure_scores,
		path: "/security/secureScores"
	);
	get!(
		doc: "Get the number of the resource", 
		name: secure_scores,
		path: "/security/secureScores/$count"
	);
	delete!(
		doc: "Delete navigation property secureScores for security", 
		name: delete_secure_scores,
		path: "/security/secureScores/{{id}}",
		params: secure_score_id
	);
	get!(
		doc: "Get secureScore", 
		name: get_secure_scores,
		path: "/security/secureScores/{{id}}",
		params: secure_score_id
	);
	patch!(
		doc: "Update the navigation property secureScores in security", 
		name: update_secure_scores,
		path: "/security/secureScores/{{id}}",
		body: true,
		params: secure_score_id
	);
}
