#include <fcntl.h>
#include <stdio.h>
#include <sys/mman.h>
#include <sys/stat.h>

#define MAX_RED 12
#define MAX_GREEN 13
#define MAX_BLUE 14

typedef enum Color {
  RED = 0,
  GREEN = 1,
  BLUE = 2,
} color_t;

void skip_whitespace(char **p) {
  while (**p == ' ') ++(*p);
}

int expect_int(char **p) {
  skip_whitespace(p);
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

color_t expect_color(char **p) {
  skip_whitespace(p);
  switch (**p) {
    case 'r':
      *p += 3;
      return RED;
    case 'g':
      *p += 5;
      return GREEN;
    case 'b':
      *p += 4;
      return BLUE;
    default:
      return -1;
  }
}

int main() {
  int fd = open("input.txt", O_RDONLY);
  struct stat sb;
  fstat(fd, &sb);

  char *fp = mmap(NULL, sb.st_size, PROT_READ, MAP_PRIVATE, fd, 0);
  char *fp_max = fp + sb.st_size;

  const int cube_max[] = {MAX_RED, MAX_GREEN, MAX_BLUE};
  int sum = 0;
  while (fp < fp_max) {
    fp += 5;  // skipping "Game "

    int game_count = expect_int(&fp);

    ++fp;  // skipping ":"

    int is_below_max = 1;
    while (*fp != '\n') {
      int cube_count = expect_int(&fp);
      color_t color = expect_color(&fp);

      if (cube_count > cube_max[color]) {
        is_below_max = 0;
        while (*fp != '\n') ++fp;  // skip to eol
      }

      if (*fp == ',' || *fp == ';') {
        ++fp;
      }
    }

    if (is_below_max) {
      sum += game_count;
    }
    ++fp;  // continue to the next line!
  }
  printf("sum: %d\n", sum);
}
