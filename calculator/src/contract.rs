use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config, config_read, Config};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let config_state = Config {};

    config(deps.storage).save(&config_state)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Addition { num1, num2 } => execute_addition(deps, env, info, num1, num2),
        ExecuteMsg::Subtraction { num1, num2 } => execute_subtraction(deps, env, info, num1, num2),
    }
}

pub fn execute_addition(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    num1: i16,
    num2: i16,
) -> Result<Response, ContractError> {
    let op = num1 + num2;

    Ok(Response::new()
        .add_attribute("num1", num1.to_string())
        .add_attribute("num2", num2.to_string())
        .add_attribute("result", op.to_string()))
}

pub fn execute_subtraction(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    num1: i16,
    num2: i16,
) -> Result<Response, ContractError> {
    let op = num1 - num2;

    Ok(Response::new()
        .add_attribute("num1", num1.to_string())
        .add_attribute("num2", num2.to_string())
        .add_attribute("result", op.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&config_read(deps.storage).load()?),
    }
}
