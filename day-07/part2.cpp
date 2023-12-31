#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

struct Hand {
  std::string cards;
  uint64_t bid{0};
  uint8_t type{0};
};

const std::unordered_map<char, char> kTranslateP2{
    {'J', 'A'}, {'2', 'B'}, {'3', 'C'}, {'4', 'D'}, {'5', 'E'},
    {'6', 'F'}, {'7', 'G'}, {'8', 'H'}, {'9', 'I'}, {'T', 'J'},
    {'Q', 'K'}, {'K', 'L'}, {'A', 'M'},
};

constexpr std::string_view kLabels{"ABCDEFGHIJKLM"};

// normal case
uint8_t hand_type(const std::string &cards) {
  std::vector<uint8_t> counts;
  // from 5 to 1
  for (size_t i = cards.size(); i > 0; --i) {
    for (const auto &label : kLabels) {
      auto count = std::count_if(cards.begin(), cards.end(),
                                 [&](const auto &c) { return c == label; });
      if (count == i) {
        counts.emplace_back(count);
      }
    }
  }

  if (counts.front() == 5) {
    return 7;
  } else if (counts.front() == 4) {
    return 6;
  } else if (counts.front() == 3 && counts.back() == 2) {
    return 5;
  } else if (counts.front() == 3) {
    return 4;
  } else if (counts.front() == 2 && counts.at(1) == 2) {
    return 3;
  } else if (counts.front() == 2) {
    return 2;
  }
  return 1;
}

uint8_t typep2(const std::string &cards) {
  uint8_t best{hand_type(cards)};
  if (cards.find('A') == std::string::npos) {
    return best;
  }
  for (const auto &r : kLabels) {
    auto replaced = cards;
    for (auto &c : replaced) {
      if (c == 'A') {
        c = r;
      }
    }
    best = std::max(best, hand_type(replaced));
  }
  return best;
}

std::vector<Hand> parse(std::string path,
                        const std::unordered_map<char, char> &translation) {
  std::ifstream s(path);
  std::vector<Hand> hands;
  for (std::string line; std::getline(s, line);) {
    std::stringstream ss(line);
    auto &hand = hands.emplace_back();
    ss >> hand.cards >> hand.bid;
    for (auto &c : hand.cards) {
      c = translation.at(c);
    }
    hand.type = typep2(hand.cards);
  }
  return hands;
}

uint64_t solve(const std::vector<Hand> &hands) {
  auto sorted_hands = hands;
  std::sort(sorted_hands.begin(), sorted_hands.end(),
            [](const auto &a, const auto &b) {
              return (a.type != b.type) ? (a.type < b.type) : a.cards < b.cards;
            });
  uint64_t answ{0};
  for (int rank{0}; rank < sorted_hands.size(); ++rank) {
    const auto &hand = sorted_hands[rank];
    answ += hand.bid * (rank + 1);
  }
  return answ;
}

int main() {
  std::vector<Hand> hands = parse("day7.txt", kTranslateP2);
  std::cout << solve(hands) << std::endl;
  return 0;
}
