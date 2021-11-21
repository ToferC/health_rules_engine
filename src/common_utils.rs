use async_graphql::Guard;
use async_graphql::*;

#[derive(Eq, PartialEq, Display, EnumString, Copy, Clone)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Admin,
    Operator,
    Analyst,
    User,
}

pub struct RoleGuard {
    pub role: Role,
}

impl RoleGuard {
    pub fn new(role: Role) -> Self {
        Self { role }
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, context: &Context<'_>) -> Result<(), async_graphql::Error> {
        
        if context.data_opt::<Role>() == Some(&self.role) || context.data_opt::<Role>() == Some(&Role::Admin) {
            Ok(())
        } else {
            let guard_error = context.data_opt::<jsonwebtoken::errors::Error>().clone();
            match guard_error {
                Some(e) => return Err(format!("{:?}", e.kind()).into()),
                None => return Err(format!("Access denied: {} role required", &self.role).into())
            }
        }
    }
}

/// Field will be visible to users with Role::Admin and
/// Role::Analyst
pub fn is_analyst(ctx: &Context<'_>) -> bool {
    ctx.data_opt::<Role>() == Some(&Role::Admin) ||
    ctx.data_opt::<Role>() == Some(&Role::Analyst)
}

/// Field will be visible to users with Role::Admin and
/// Role::Analyst
pub fn is_operator(ctx: &Context<'_>) -> bool {
    ctx.data_opt::<Role>() == Some(&Role::Admin) ||
    ctx.data_opt::<Role>() == Some(&Role::Operator)
}

/// Field will only be visible to users with Role::Admin
pub fn is_admin(ctx: &Context<'_>) -> bool {
    ctx.data_opt::<Role>() == Some(&Role::Admin)
}