use async_graphql::guard::Guard;
use async_graphql::*;

#[derive(Eq, PartialEq, Display, EnumString)]
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

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, context: &Context<'_>) -> Result<(), async_graphql::Error> {
        
        if context.data_opt::<Role>() == Some(&self.role) {
            Ok(())
        } else {
            let guard_error = context.data_opt::<jsonwebtoken::errors::Error>().clone();
            match guard_error {
                Some(e) => return Err(format!("{:?}", e.kind()).into()),
                None => return Err("No authentication token found".into())
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

/// Field will only be visible to users with Role::Admin
pub fn is_admin(ctx: &Context<'_>) -> bool {
    ctx.data_opt::<Role>() == Some(&Role::Admin)
}