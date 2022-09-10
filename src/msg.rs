use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use nois_proxy::NoisCallbackMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub nois_proxy: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RollDice {
        /// An ID for this job which allows for gathering the results.
        job_id: String,
    },
    Receive(NoisCallbackMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number

    
    QueryOutcome {job_id: String },
    GetHistoryOfRounds{},
}