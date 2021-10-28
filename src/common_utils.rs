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
            let guard_error = context.data_opt::<jsonwebtoken::errors::Error>().clone();
            match guard_error {
                Some(e) => return Err(format!("{:?}", e.kind()).into()),
                None => return Err("Unable to decode token".into())
            }
        }
    }
}