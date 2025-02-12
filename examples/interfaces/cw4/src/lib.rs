use cosmwasm_std::{Response, StdError};

use sylvia::types::{ExecCtx, QueryCtx};
use sylvia::{interface, schemars};

#[interface]
pub trait Cw4 {
    type Error: From<StdError>;

    #[msg(exec)]
    fn update_admin(&self, ctx: ExecCtx, admin: String) -> Result<Response, Self::Error>;

    #[msg(exec)]
    fn update_members(&self, ctx: ExecCtx, members: Vec<String>) -> Result<Response, Self::Error>;

    #[msg(exec)]
    fn add_hook(&self, ctx: ExecCtx, hook: String) -> Result<Response, Self::Error>;

    #[msg(exec)]
    fn remove_hook(&self, ctx: ExecCtx, hook: String) -> Result<Response, Self::Error>;

    #[msg(query)]
    fn member(&self, ctx: QueryCtx, member: String) -> Result<Response, Self::Error>;

    #[msg(query)]
    fn list_members(&self, ctx: QueryCtx) -> Result<Response, Self::Error>;

    #[msg(query)]
    fn total_weight(&self, ctx: QueryCtx) -> Result<Response, Self::Error>;

    #[msg(query)]
    fn admin(&self, ctx: QueryCtx) -> Result<Response, Self::Error>;

    #[msg(query)]
    fn hooks(&self, ctx: QueryCtx) -> Result<Response, Self::Error>;
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_json, to_json_binary};

    use super::sv::*;

    #[test]
    fn execute() {
        let original_msg = Cw4ExecMsg::UpdateAdmin {
            admin: "admin_name".to_owned(),
        };

        let serialized_msg = to_json_binary(&original_msg).unwrap();
        let serialized_msg: Cw4ExecMsg = from_json(serialized_msg).unwrap();

        assert_eq!(serialized_msg, original_msg);
    }

    #[test]
    fn query() {
        let original_msg = Cw4QueryMsg::Admin {};

        let serialized_msg = to_json_binary(&original_msg).unwrap();
        let serialized_msg: Cw4QueryMsg = from_json(serialized_msg).unwrap();

        assert_eq!(serialized_msg, original_msg);
    }

    #[test]
    fn execute_from_json() {
        let deserialized: Cw4ExecMsg =
            from_json(br#"{"update_admin": {"admin": "admin_name"}}"#).unwrap();
        assert_eq!(
            deserialized,
            Cw4ExecMsg::UpdateAdmin {
                admin: "admin_name".to_owned()
            }
        );
    }

    #[test]
    fn query_from_json() {
        let deserialized: Cw4QueryMsg = from_json(br#"{"admin": {}}"#).unwrap();
        assert_eq!(deserialized, Cw4QueryMsg::Admin {});
    }

    #[test]
    fn exec_msgs() {
        assert_eq!(
            execute_messages(),
            ["add_hook", "remove_hook", "update_admin", "update_members"]
        );
    }

    #[test]
    fn query_msgs() {
        assert_eq!(
            query_messages(),
            ["admin", "hooks", "list_members", "member", "total_weight"]
        );
    }
}
