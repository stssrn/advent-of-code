#include <fcntl.h>
#include <stdio.h>
#include <sys/mman.h>
#include <sys/stat.h>

#define WINNING_NUMBERS_PER_CARD 10
#define SCRATCHED_NUMBERS_PER_CARD 25

int expect_int(char **p) {
  while (**p == ' ') ++(*p);  // skip spaces
  int n = 0;
  for (;;) {
    int x = **p;
    if (x >= '0' && x <= '9') {
      n *= 10;
      n += x - '0';
      ++(*p);
    } else
      break;
  }
  return n;
}

int main() {
  int fd = open("input.txt", O_RDONLY);
  struct stat sb;
  fstat(fd, &sb);

  char *fp = mmap(NULL, sb.st_size, PROT_READ, MAP_PRIVATE, fd, 0);
  char *fp_end = fp + sb.st_size;

  int points = 0;
  while (fp < fp_end) {
    int won_number_count = 0;
    fp += 10;  // skip "Card    n: "

    int winning_numbers[WINNING_NUMBERS_PER_CARD];
    for (int j = 0; j < WINNING_NUMBERS_PER_CARD; j++) {
      winning_numbers[j] = expect_int(&fp);
    }

    fp += 3;  // skip " | "

    for (int j = 0; j < SCRATCHED_NUMBERS_PER_CARD; j++) {
      int scratched_number = expect_int(&fp);
      for (int k = 0; k < WINNING_NUMBERS_PER_CARD; k++) {
        if (winning_numbers[k] == scratched_number) {
          ++won_number_count;
        }
      }
    }

    if (won_number_count != 0) {
      points += 1 << (won_number_count - 1);
    }

    ++fp;  // continue to next card! (skipping \n)
  }
  printf("you won %d points\n", points);
}
