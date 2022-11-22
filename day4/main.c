#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#if 0
#define BITS_COUNT 12
#define INPUT_FILE "./input.txt"
#else
#define BITS_COUNT 5
#define INPUT_FILE "./sample.txt"
#endif
#define XS_CAP 1024
#define BOOL char
#define FALSE 0
#define TRUE 1


int readxs(int xs[])
{
    FILE *fp;
    char *line = NULL;
    char *endp;
    size_t len=0;
    ssize_t nread;
    int xs_sz = 0;
    unsigned int x;
    
    int init_size = 0;
    char delim[] = ",";
    char *ptr;

    fp = fopen(INPUT_FILE, "r");
    if (fp == NULL) {
        perror("Error while opening the file.\n");
        exit(1);
    }

    if ((nread = getline(&line, &len, fp)) != -1) {
        init_size = strlen(line);
        ptr = strtok(line, delim);
        while(ptr != NULL) {
            printf("%s ", ptr);
            ptr = strtok(NULL, delim);
            xs_sz++;
        }
    }
/*    while ((nread = getline(&line, &len, fp)) != -1) {
        x = (unsigned int)strtol(line, &endp, 2);
        xs[xs_sz] = x;
        xs_removed[xs_sz] = FALSE;
        xs_sz++;
    } */

    fclose(fp);

    return xs_sz;
}

void part1()
{
    int numbers[XS_CAP];
    int i = 0;
    i = readxs(numbers);

    printf("Count numbers %d", i);
}


void dumpxs(unsigned int xs[])
{   
}

void part2()
{
  /*  unsigned int xs[XS_CAP];
    BOOL xs_removed[XS_CAP];
    int xs_sz = 0, xs_cnt = 0;
    unsigned int oxygen = 0;
    unsigned int co2 = 0;

    xs_sz = readxs(xs, xs_removed);
    printf("Readed %d\n", xs_sz);
    dumpxs(xs, xs_removed, xs_sz);
    oxygen = searchValue(xs, xs_removed, xs_sz, TRUE);
    co2    = searchValue(xs, xs_removed, xs_sz, FALSE);

    printf("Oxygen %d\n", oxygen);
    printf("Co2    %d\n", co2);
    printf("Part 2 %d\n", oxygen * co2); */
}   


int main(void)
{
    part1();
//    part2();
    return 0;
}
