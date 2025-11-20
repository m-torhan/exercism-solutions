#include <string>

namespace log_line {
std::string message(std::string line) {
    size_t idx = line.find(":");

    if (idx == std::string::npos || idx + 2 >= line.size()) {
        return line;
    }

    return line.substr(idx + 2);
}

std::string log_level(std::string line) {
    size_t idx = line.find(":");

    if (idx == std::string::npos || idx <= 1) {
        return line;
    }

    return line.substr(1, idx - 2);
}

std::string reformat(std::string line) { return message(line) + " (" + log_level(line) + ")"; }
} /* namespace log_line */
