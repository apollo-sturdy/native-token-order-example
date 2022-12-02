#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:native-token-order-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {} => {
            deps.api.debug("Deposit");
            deps.api.debug(&format!("Deposit from {}", info.sender));
            deps.api.debug(&format!("Funds: {:?}", info.funds));
            Ok(Response::new())
        }
        ExecuteMsg::DepositAndRefund {} => {
            deps.api.debug("DepositAndRefund");
            deps.api.debug(&format!("Deposit from {}", info.sender));
            deps.api.debug(&format!("Funds: {:?}", info.funds));
            let msg = BankMsg::Send {
                to_address: info.sender.to_string(),
                amount: info.funds,
            };
            Ok(Response::new().add_message(msg))
        }
        ExecuteMsg::DepositAndRefundReverseSorted {} => {
            deps.api.debug("DepositAndRefundReverseSorted");
            deps.api.debug(&format!("Deposit from {}", info.sender));
            deps.api.debug(&format!("Funds: {:?}", info.funds));
            let mut funds = info.funds.clone();
            funds.sort_by(|a, b| b.denom.cmp(&a.denom));
            deps.api
                .debug(&format!("Reverse sorted funds: {:?}", funds));

            let msg = BankMsg::Send {
                to_address: info.sender.to_string(),
                amount: funds,
            };
            Ok(Response::new().add_message(msg))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
