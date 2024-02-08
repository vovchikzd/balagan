#include <iostream>
#include <optional>
#include <expected>
#include <vector>
#include <iterator>

int main(int argc, char* argv[]) {
  std::vector<std::string> args(argv, argv + argc);
  std::ostream_iterator<std::string> osit(std::cout, " ");
  std::copy(args.begin(), args.end(), osit);
  return 0;
}
