use async_graphql::guard::Guard;
use async_graphql::*;

#[derive(Eq, PartialEq, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Admin,
    Operator,
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
            Err("Forbidden".into())
        }
    }
}

struct Age(i32);

struct AgeGuard {
    age: i32,
}

#[async_trait::async_trait]
impl Guard for AgeGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if ctx.data_opt::<Age>().map(|name| &name.0) == Some(&self.age) {
            Ok(())
        } else {
            Err("Forbidden - Token Expired".into())
        }
    }
}