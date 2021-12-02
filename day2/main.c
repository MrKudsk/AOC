#include <stdio.h>
#include <stdlib.h>

void part1(const char *filename)
{
    FILE *fp;
    char *line = NULL;
    size_t len = 0;
    ssize_t nread;
    int result = 0;
    int num = 0;
    int horizontal = 0;
    int depth = 0;
    char direction[20];

    fp = fopen(filename, "r");

    if (fp == NULL)
    {
        perror("Error while openging the file.\n");
        exit(-1);
    }

    while ((nread = getline(&line, &len, fp)) != -1)
    {
        sscanf(line, "%s %d", direction, &num);
        // printf("%s, %d antal\n", direction, num);
        switch(direction[0])
        {
            case 'f':
                horizontal += num;
                break;
            case 'd':
                depth += num;
                break;
            case 'u':
                depth -= num;
                break;
            default:
                printf("Error in reading file");
                exit(1);
        }

    }
    printf("horizontal %d and depth %d\n\n", horizontal, depth);
    printf("Result part 1 = %d \n\n", (horizontal * depth));

    fclose(fp);
}

void part2(const char *filename)
{
    FILE *fp;
    char *line = NULL;
    size_t len = 0;
    ssize_t nread;
    int aim = 0;
    int num = 0;
    int horizontal = 0;
    int depth = 0;
    char direction[20];

    fp = fopen(filename, "r");

    if (fp == NULL)
    {
        perror("Error while openging the file.\n");
        exit(-1);
    }

    while ((nread = getline(&line, &len, fp)) != -1)
    {
        sscanf(line, "%s %d", direction, &num);
        // printf("%s, %d antal\n", direction, num);
        switch(direction[0])
        {
            case 'f':
                horizontal += num;
                depth += (aim * num);
                break;
            case 'd': // down
                //depth += num;
                aim += num;
                break;
            case 'u': // up
                //depth -= num;
                aim -= num;
                break;
            default:
                printf("Error in reading file");
                exit(1);
        }
        //printf("horizontal %d and depth %d and aim %d\n\n", horizontal, depth, aim);
    }
    printf("horizontal %d and depth %d and aim %d\n\n", horizontal, depth, aim);
    printf("Result part 2 = %d \n\n", (horizontal * depth));

    fclose(fp);
}


int main(void)
{

    //char filename[25] = "sampel.txt";
    char filename[25] = "input.txt";
    part1(filename);
    part2(filename);

    return 0;
}

