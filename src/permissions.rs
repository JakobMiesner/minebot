use serenity::all::{CommandInteraction, Permissions};
use std::env;

pub fn parse_allowed_roles() -> Vec<u64> {
  env::var("ALLOWED_ROLE_IDS")
    .ok()
    .map(|roles| roles.split(',').filter_map(|id| id.trim().parse::<u64>().ok()).collect())
    .unwrap_or_default()
}

pub async fn has_permission(command: &CommandInteraction) -> bool {
  // Check if user has administrator permissions
  if let Some(member) = &command.member {
    if let Some(permissions) = member.permissions {
      if permissions.contains(Permissions::ADMINISTRATOR) {
        return true;
      }
    }

    // Check if user has any of the allowed roles
    let allowed_roles = parse_allowed_roles();
    if !allowed_roles.is_empty() {
      for role_id in &member.roles {
        if allowed_roles.contains(&role_id.get()) {
          return true;
        }
      }
    }
  }

  false
}

pub fn requires_permission(command_name: &str) -> bool {
  matches!(command_name, "whitelist")
}
