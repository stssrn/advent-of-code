#include <stdio.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>

int main() {
  struct stat sb;
  int fd = open("input.txt", O_RDONLY);

  fstat(fd, &sb);
  char *fp = mmap(NULL, sb.st_size, PROT_READ, MAP_PRIVATE, fd, 0);

  int sum = 0, first = 0, last = 0;

  for (size_t i = 0; i < sb.st_size; i++) {
    char c = fp[i];
    if (c == '\n') {
      sum += first + last;
      first = last = 0;
    }

    // check if it's a digit
    if (c >= '0' && c <= '9') {
      c -= '0';  // ascii to correct digit
      if (first == 0) {
        first = c * 10;
      }
      last = c;
    }
  }

  printf("calibration values sum = %d\n", sum);
}
