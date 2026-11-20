use cedar_policy::{Authorizer, Context, Decision, Entities, EntityUid, PolicySet, Request};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

/// Display demo header with request details
pub fn display_header(user: &str, resource: &str, policy_name: &str) {
    println!("=== Cedar Policy Authorization Demo ({policy_name}) ===");
    println!("User: {user}");
    println!("Resource: {resource}\n");
}

/// Check if the user is authorized to perform an action on a resource
pub fn is_authorized(
    user: &str,
    action: &str,
    resource: &str,
    policy_file: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Load and parse the Cedar policy
    let policy_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(format!("../policies/{policy_file}"));
    let policy_content = fs::read_to_string(&policy_path)?;
    let policies = policy_content.parse::<PolicySet>()?;

    // Build the authorization request
    let principal = EntityUid::from_str(&format!(r#"User::"{user}""#))?;
    let action_uid = EntityUid::from_str(&format!(r#"Action::"{action}""#))?;
    let resource_uid = EntityUid::from_str(&format!(r#"Resource::"{resource}""#))?;

    let request = Request::new(principal, action_uid, resource_uid, Context::empty(), None)?;
    let entities = Entities::empty();

    // Evaluate the request against the policy
    let decision = Authorizer::new()
        .is_authorized(&request, &policies, &entities)
        .decision();

    Ok(decision == Decision::Allow)
}
