// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use std::net::TcpStream;

use anyhow::Result;
use serde_json::Value;

#[path = "utils.rs"]
mod utils;

// This module contains the handling logic for dyno gputrace

/// Gputrace command triggers GPU profiling on pytorch apps
pub fn run_gputrace(
    client: TcpStream,
    job_id: u64,
    pids: &str,
    duration_ms: u64,
    iterations: i64,
    log_file: &str,
    profile_start_time: u64,
    profile_start_iteration_roundup: u64,
    process_limit: u32,
    activities: &str,
    verbose_log_level: i64,
    activities_warmup_period_secs: u64,

) -> Result<()> {
    let trigger_config = if iterations > 0 {
        format!(
            r#"PROFILE_START_ITERATION=0\nPROFILE_START_ITERATION_ROUNDUP={}\nACTIVITIES_ITERATIONS={}"#,
            profile_start_iteration_roundup, iterations
        )
    } else {
        format!(
            r#"PROFILE_START_TIME={}\nACTIVITIES_DURATION_MSECS={}"#,
            profile_start_time, duration_ms
        )
    };

    let kineto_config = format!(r#"ACTIVITIES_LOG_FILE={}\nACTIVITY_TYPES={}\nVERBOSE_LOG_LEVEL={}\nACTIVITIES_WARMUP_PERIOD_SECS={}\n{}"#,
    log_file, activities, verbose_log_level, activities_warmup_period_secs, trigger_config);

    println!("Kineto config = \n{}", kineto_config);

    let request_json = format!(
        r#"
{{
    "fn": "setKinetOnDemandRequest",
    "config": "{}",
    "job_id": {},
    "pids": [{}],
    "process_limit": {}
}}"#,
        kineto_config, job_id, pids, process_limit
    );

    utils::send_msg(&client, &request_json).expect("Error sending message to service");

    let resp_str = utils::get_resp(&client).expect("Unable to decode output bytes");

    println!("response = {}", resp_str);

    let resp_v: Value = serde_json::from_str(&resp_str)?;
    let processes = resp_v["processesMatched"].as_array().unwrap();

    if processes.is_empty() {
        println!("No processes were matched, please check --job-id or --pids flags");
    } else {
        println!("Matched {} processes", processes.len());
        println!("Trace output files will be written to:");

        for pid in processes {
            let pid = pid.as_i64().unwrap();
            println!(
                "    {}",
                log_file.replace(".json", &format!("_{}.json", pid))
            );
        }
    }

    Ok(())
}

/// GputraceState command get GPU profiling state on pytorch apps
pub fn run_gputrace_state(
    client: TcpStream,
    job_id: u64,
    pids: &str,
) -> Result<()> {
    let request_json = format!(
        r#"
{{
    "fn": "getKinetOnDemandProfilingStateRequest",
    "job_id": {},
    "pids": [{}]
}}"#,
        job_id, pids
    );

    utils::send_msg(&client, &request_json).expect("Error sending message to service");

    let resp_str = utils::get_resp(&client).expect("Unable to decode output bytes");

    println!("response = {}", resp_str);

    Ok(())
}

/// GetGpuTraceChildPids command get GPU profiling child pids on pytorch apps
pub fn get_gputrace_child_pids(
    client: TcpStream,
    job_id: u64,
) -> Result<()> {
    let request_json = format!(
        r#"
{{
    "fn": "getKinetOnDemandProfilingChildPidsRequest",
    "job_id": {}
}}"#,
        job_id
    );

    utils::send_msg(&client, &request_json).expect("Error sending message to service");

    let resp_str = utils::get_resp(&client).expect("Unable to decode output bytes");

    println!("response = {}", resp_str);

    Ok(())
}