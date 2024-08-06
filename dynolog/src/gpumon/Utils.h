// (c) Meta Platforms, Inc. and affiliates. Confidential and proprietary.

#pragma once

#include <string>
#include <unordered_map>
#include <vector>

namespace dynolog {

// Assume at most one pid running on each gpu device_id
// Result[i] is the pid running on device_id i,
// or -1 if no process is running
std::vector<pid_t> getPidsOnGpu();
// Input map is type <environment variable name, metadata name>
// output map is type <metadata name, metadata value>
std::unordered_map<std::string, std::string> getMetadataForPid(
    pid_t pid,
    const std::unordered_map<std::string, std::string>& keysMap);

} // namespace dynolog
