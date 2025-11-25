#include <string>
#include <vector>

namespace election {

// The election result struct is already created for you:

struct ElectionResult {
    // Name of the candidate
    std::string name{};
    // Number of votes the candidate has
    int votes{};
};

int vote_count(const struct ElectionResult &result) { return result.votes; }

void increment_vote_count(struct ElectionResult &result, int votes) { result.votes += votes; }

struct ElectionResult &determine_result(std::vector<ElectionResult> &results) {
    int winner_idx = 0;

    for (int i = 1; i < results.size(); ++i) {
        if (results[i].votes > results[winner_idx].votes) {
            winner_idx = i;
        }
    }

    results[winner_idx].name = "President " + results[winner_idx].name;

    return results[winner_idx];
}

} // namespace election
