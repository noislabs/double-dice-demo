#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Order};
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg};
use cw2::set_contract_version;

use nois_proxy::{Data, NoisCallbackMsg};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{NOIS_PROXY, DOUBLE_DICE_OUTCOME};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:double-dice-roll";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let nois_proxy_addr = deps
        .api
        .addr_validate(&msg.nois_proxy)
        .map_err(|_| ContractError::InvalidProxyAddress)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    NOIS_PROXY.save(deps.storage, &nois_proxy_addr)?;
    

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RollDice { job_id } => execute_roll_dice(deps, env, info, job_id),
        ExecuteMsg::Receive(NoisCallbackMsg {
            id: callback_id,
            randomness,
        }) => execute_receive(deps, env, info, callback_id, randomness),
    }
}

pub fn execute_roll_dice(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    job_id: String,
) -> Result<Response, ContractError> {
    let nois_proxy = NOIS_PROXY.load(deps.storage)?;

    let res = Response::new().add_message(WasmMsg::Execute {
        contract_addr: nois_proxy.into(),
        msg: to_binary(&nois_proxy::ExecuteMsg::GetNextRandomness {
            callback_id: Some(job_id),
        })?,
        funds: vec![],
    });
    Ok(res)
}

pub fn execute_receive(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    callback_id: String,
    randomness: Data,
) -> Result<Response, ContractError> {
    let randomness: [u8; 32] = randomness
        .to_array()
        .map_err(|_| ContractError::InvalidRandomness)?;

    let dice_outcome_1= randomness[0] % 6 + 1;  
    let dice_outcome_2= randomness[1] % 6 + 1;
    // randomness[0] is a hex between 0 - 15
    // which is not a multiple of 6 so this a good dice. 
    // So should be more lik [0..16]
    //but whatever this is just a demo

    let double_dice_outcome = dice_outcome_1 + dice_outcome_2;

    DOUBLE_DICE_OUTCOME.save(deps.storage, &callback_id, &double_dice_outcome)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetHistoryOfRounds{} => to_binary(&query_history(deps)?),
        QueryMsg::QueryOutcome { job_id } => to_binary(&query_outcome(deps, job_id)?),

    }
}

fn query_outcome(deps: Deps, job_id: String) -> StdResult<Option<u8>> {
    let outcome = DOUBLE_DICE_OUTCOME.may_load(deps.storage, &job_id)?;
    Ok(outcome)
}

fn query_history(deps: Deps) -> StdResult<Vec<String>> {
    let out: Vec<String> = DOUBLE_DICE_OUTCOME
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| item.map(|(id, value)| format!("{id}:{value}")))
        .collect::<StdResult<_>>()?;
    Ok(out)
}



#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {nois_proxy: "address123".to_string(),};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());}
}
