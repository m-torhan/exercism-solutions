#include <sstream>
#include <string>

#include "run_length_encoding.h"

namespace run_length_encoding {

std::string encode(std::string s) {
    char symbol = 0;
    int count = 0;

    std::stringstream out;

    for (char &c : s) {
        if (symbol == c) {
            ++count;
            continue;
        }

        if (symbol != 0) {
            if (count > 1) {
                out << std::to_string(count);
            }
            out << symbol;
        }

        count = 1;
        symbol = c;
    }

    if (count != 0) {
        if (count > 1) {
            out << std::to_string(count);
        }
        out << symbol;
    }

    return out.str();
}

std::string decode(std::string s) {
    std::stringstream out;

    int count = 0;

    for (char &c : s) {
        if ('0' <= c && c <= '9') {
            count *= 10;
            count += (c - '0');
        } else {
            if (count == 0) {
                count = 1;
            }
            for (int i = 0; i < count; ++i) {
                out << c;
            }
            count = 0;
        }
    }

    return out.str();
}

} /* namespace run_length_encoding */
