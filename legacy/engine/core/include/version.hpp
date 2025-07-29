#pragma once

#include <cstdint>
#include <string>
#include <tuple>

namespace lp_engine {
namespace version {
bool isVersionAtLeast(uint32_t major, uint32_t minor, uint32_t patch);
bool isVersionLessThan(uint32_t major, uint32_t minor, uint32_t patch);
bool isVersionEqualTo(uint32_t major, uint32_t minor, uint32_t patch);
bool isVersionEqualTo(const std::string &version);

uint32_t getMajor();
uint32_t getMinor();
uint32_t getPatch();
std::string getPrerelease();
std::string getProjectVersion();
std::string getProjectFullVersion();
std::string getGitCommitHash();

std::tuple<uint32_t, uint32_t, uint32_t> getSemanticVersion();

bool isPrerelease();
} // namespace version
} // namespace lp_engine
