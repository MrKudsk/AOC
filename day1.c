#include <stdio.h>
#include <stdlib.h>

int main()
{
    FILE *fp;
    char filename[25] = "input.txt";
    char *line = NULL;
    size_t len = 0;
    ssize_t nread;

    fp = fopen(filename, "r");

    if (fp == NULL)
    {
        perror("Error while openging the file.\n");
        exit(-1);
    }

    while ((nread = getline(&line, &len, fp)) != -1)
    {
        printf("Line %s, %d \n", line, nread);
    }

    fclose(fp);
    return 0;
}

