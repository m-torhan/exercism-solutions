namespace hellmath {

enum class AccountStatus {
    troll,
    guest,
    user,
    mod,
};

enum class Action {
    read,
    write,
    remove,
};

bool display_post(AccountStatus poster, AccountStatus viewer) {
    if (poster == AccountStatus::troll) {
        return viewer == AccountStatus::troll;
    }

    return true;
}

bool permission_check(Action action, AccountStatus user) {
    switch (user) {
    case AccountStatus::troll:
        return action == Action::read || action == Action::write;
    case AccountStatus::user:
        return action == Action::read || action == Action::write;
    case AccountStatus::guest:
        return action == Action::read;
    case AccountStatus::mod:
        return true;
    }

    return false;
}

bool valid_player_combination(AccountStatus user_a, AccountStatus user_b) {
    if (user_a == AccountStatus::guest || user_b == AccountStatus::guest) {
        return false;
    }

    if (user_a == AccountStatus::troll) {
        return user_b == AccountStatus::troll;
    }

    if (user_b == AccountStatus::troll) {
        return user_a == AccountStatus::troll;
    }

    return true;
}

bool has_priority(AccountStatus user_a, AccountStatus user_b) {
    switch (user_a) {
    case AccountStatus::mod:
        return user_b != AccountStatus::mod;
    case AccountStatus::user:
        return user_b != AccountStatus::mod && user_b != AccountStatus::user;
    case AccountStatus::guest:
        return user_b != AccountStatus::mod && user_b != AccountStatus::user && user_b != AccountStatus::guest;
    case AccountStatus::troll:
        return false;
    }

    return false;
}

} // namespace hellmath
