#include <array>
#include <string>
#include <vector>

// Round down all provided student scores.
std::vector<int> round_down_scores(const std::vector<double> &student_scores) {
    std::vector<int> ret(student_scores.size());

    for (int i = 0; i < student_scores.size(); ++i) {
        ret[i] = static_cast<int>(student_scores[i]);
    }

    return ret;
}

// Count the number of failing students out of the group provided.
int count_failed_students(const std::vector<int> &student_scores) {
    int ret = 0;

    for (const int &score : student_scores) {
        if (score <= 40) {
            ++ret;
        }
    }

    return ret;
}

// Create a list of grade thresholds based on the provided highest grade.
std::array<int, 4> letter_grades(int highest_score) {
    return {41 + (highest_score - 40) * 0 / 4,
            41 + (highest_score - 40) * 1 / 4,
            41 + (highest_score - 40) * 2 / 4,
            41 + (highest_score - 40) * 3 / 4};
}

// Organize the student's rank, name, and grade information in ascending order.
std::vector<std::string> student_ranking(const std::vector<int> &student_scores,
                                         const std::vector<std::string> &student_names) {
    std::vector<std::string> ret(student_scores.size());

    for (int i = 0; i < student_scores.size(); ++i) {
        ret[i] = std::to_string(i + 1) + ". " + student_names[i] + ": " + std::to_string(student_scores[i]);
    }

    return ret;
}

// Create a string that contains the name of the first student to make a perfect
// score on the exam.
std::string perfect_score(const std::vector<int> &student_scores, const std::vector<std::string> &student_names) {
    for (int i = 0; i < student_scores.size(); ++i) {
        if (student_scores[i] == 100) {
            return student_names[i];
        }
    }
    return "";
}
