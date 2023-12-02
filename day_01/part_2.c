#include <fcntl.h>
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>

int main() {
  struct stat sb;
  int fd = open("input.txt", O_RDONLY);

  const char *words[] = {"one", "two",   "three", "four", "five",
                         "six", "seven", "eight", "nine"};

  fstat(fd, &sb);
  char *fp = mmap(NULL, sb.st_size, PROT_READ, MAP_PRIVATE, fd, 0);

  int sum = 0, first = 0, last = 0;

  for (size_t i = 0; i < sb.st_size; i++) {
    int n = 0;

    if (fp[i] == '\n') {
      sum += first + last;
      first = last = 0;
    }

    // check if it's a digit
    else if (fp[i] >= '0' && fp[i] <= '9')
      n = fp[i] - '0';  // ascii to correct digit

    else
      for (size_t j = 0; j < 9; j++) {
        const char *word = words[j];
        int is_word = memcmp(&fp[i], word, strlen(word));
        if (is_word == 0) {
          n = j + 1;
          break;
        };
      }

    if (n != 0) {
      if (first == 0) {
        first = n * 10;
      }
      last = n;
    }
  }

  printf("calibration values sum = %d\n", sum);
}
