#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#if 1
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

int binary2int(char *bin)
{
    int bp = 0;
    int r = 0;
    for(int i=BITS_COUNT-1; i >= 0; i--)
    {
        //printf("%c %d\n", bin[i], (int)pow(2,bp));
        if (bin[i] == '1')
        {
            r += (1 * (int)pow(2,bp) );
        }
        bp++;
    }
    return r;
}

void part1(const char *filename)
{
    FILE *fp;
    char *line = NULL;
    size_t len = 0;
    ssize_t nread;
    int x;
    int bits[BITS_COUNT][2];
    char gamma_s[BITS_COUNT+1];
    char epsilon_s[BITS_COUNT+1];
    int result;

    fp = fopen(filename, "r");

    if (fp == NULL)
    {
        perror("Error while openging the file.\n");
        exit(-1);
    }

    memset(bits,0, sizeof(bits));
    gamma_s[BITS_COUNT] = '\0';
    epsilon_s[BITS_COUNT] = '\0';

    while ((nread = getline(&line, &len, fp)) != -1)
    {
        for (int i=0; i < BITS_COUNT; i++)
        {
            x = (int)line[i] - 48;
            bits[i][x]++;
        }
    }
    for(int i=0; i< BITS_COUNT;i++)
    {
        if (bits[i][0] > bits[i][1])
        {
            gamma_s[i] = '0';
            epsilon_s[i] = '1';
        } else
        {
            gamma_s[i] = '1';
            epsilon_s[i] = '0';
        }
    }
    printf("Gamma %s %d\n\n", gamma_s, binary2int(gamma_s));
    printf("Epsilon %s %d\n\n", epsilon_s, binary2int(epsilon_s));
    printf("Result part 1: %d\n\n", binary2int(gamma_s) * binary2int(epsilon_s));
    fclose(fp);
}

unsigned int string2binary(char *binary_str)
{
    unsigned int value =0;

    for (int i = 0; i < BITS_COUNT; i++)
    {
        if (binary_str[i] == '1')
        {
            value = value | 1;

        }
        if (i != BITS_COUNT-1)
        {
            value = value << 1;
        }
    }
    return value;
}

char* binary2string(unsigned int value, char *binary_str)
{
    for (unsigned int mask = 1 << (BITS_COUNT-1); mask !=0; mask >>=1)
    {
        strcat(binary_str, (mask & value) != 0 ? "1": "0");
    }
    return binary_str;
}

void dumpxs(unsigned int xs[], BOOL xs_removed[], int xs_sz)
{   
    for(int i = 0; i < xs_sz; ++i) {
        printf("%3d ", xs[i]);
        if (!xs_removed[i]) {
            for (int b = (BITS_COUNT-1); b >= 0; --b) {
                printf("%c", ((xs[i]>>b) & 1) + '0');
            }
            printf(" : %d\n", xs_removed[i]);
        }
    }
}

int readxs(unsigned int xs[], BOOL xs_removed[])
{
    FILE *fp;
    char *line = NULL;
    char *endp;
    size_t len=0;
    ssize_t nread;
    int xs_sz = 0;
    unsigned int x;

    fp = fopen(INPUT_FILE, "r");
    if (fp == NULL) {
        perror("Error while opening the file.\n");
        exit(1);
    }

    while ((nread = getline(&line, &len, fp)) != -1) {
        x = (unsigned int)strtol(line, &endp, 2);
        xs[xs_sz] = x;
        xs_removed[xs_sz] = FALSE;
        xs_sz++;
    }
    fclose(fp);

    return xs_sz;
}

void bitsOfXs(unsigned int *xs, BOOL *xs_removed, int xs_sz, int bit_number, int *bits)
{
    //memset(bits, 0, sizeof(bits));
    bits[0] = 0;
    bits[1] = 0;
        
    for (int i = 0; i < xs_sz; ++i) {
        if (!xs_removed[i]) {
            bits[(xs[i]>>bit_number)&1]++;
        }
    }
}

/* 
 * Finde the valuse on the bits
 *
 */
unsigned int searchValue(unsigned int *xs, BOOL *xs_removed, int xs_sz, BOOL commen)
{
    int bit_number = BITS_COUNT-1;
    int bits[2];
    int bit;

    for (int i=0; i < xs_sz; ++i) {
        xs_removed[i] = FALSE;
    }
    int xs_cnt = xs_sz;


    while (xs_cnt > 1) {
        bitsOfXs(xs, xs_removed, xs_sz, bit_number, bits);
        for (int i =0; i < xs_sz; ++i) {
            if (!xs_removed[i]) {
                bit = (xs[i]>>bit_number)&1;
                if (commen) {
                    if ((bits[1] >= bits[0]) != (bit ==1)) {
                        xs_removed[i] = TRUE;
                        xs_cnt--;
                    }
                } else {
                    if ((bits[1] >= bits[0]) == (bit ==1)) {
                        xs_removed[i] = TRUE;
                        xs_cnt--;
                    }
                }
            }
        }
        bit_number--;
    }

    for (int i=0; i < xs_sz; ++i) {
        if (!xs_removed[i]) {
            return xs[i];
        }
    }
    return 0;
}


void part2()
{
    unsigned int xs[XS_CAP];
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
    printf("Part 2 %d\n", oxygen * co2);
}   


int main(void)
{

    char filename[25] = INPUT_FILE;
    part1(filename);
    part2();
    return 0;
}
