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

pub struct AdminGuard;
pub struct AnalystGuard;

pub struct OperatorGuard;

#[async_trait::async_trait]
impl Guard for AdminGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if is_admin(ctx) {
            Ok(())
        } else {
            Err("Access Denied: Admin auth needed".into())
        }
    }
}

#[async_trait::async_trait]
impl Guard for AnalystGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if is_analyst(ctx) {
            Ok(())
        } else {
            Err("Access Denied: Analyst or higher auth needed".into())
        }
    }
}

#[async_trait::async_trait]
impl Guard for OperatorGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if is_operator(ctx) {
            Ok(())
        } else {
            Err("Access Denied: Operator or higher auth needed".into())
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