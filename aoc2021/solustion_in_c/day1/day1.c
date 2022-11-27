#include <stdio.h>
#include <stdlib.h>

int main()
{
    FILE *fp;
    //char filename[25] = "sampel.txt";
    char filename[25] = "input.txt";
    char *line = NULL;
    size_t len = 0;
    ssize_t nread;
    int result = 0;
    int num = 0;
    int prev = -1;

    fp = fopen(filename, "r");

    if (fp == NULL)
    {
        perror("Error while openging the file.\n");
        exit(-1);
    }

    while ((nread = getline(&line, &len, fp)) != -1)
    {
        num = atoi(line);
        if (prev > 0 && prev < num )
        {
            result += 1;
        }
        prev = num;
    }
    printf("Result part 1 = %d \n\n", result);

    fclose(fp);

    int a, b, c, d;
    result = 0;

    fp = fopen(filename, "r");

    if (fp == NULL)
    {
        perror("Error while openging the file.\n");
        exit(-1);
    }

    nread = getline(&line, &len, fp);
    a = atoi(line);

    nread = getline(&line, &len, fp);
    b = atoi(line);

    nread = getline(&line, &len, fp);
    c = atoi(line);

    while ((nread = getline(&line, &len, fp)) != -1)
    {
        d = atoi(line);
        if ((a + b + c) < (b + c + d))
        {
            result += 1;
        }
        a = b;
        b = c;
        c = d;
    }
    printf("Result part 2 = %d \n\n", result);

    fclose(fp);



    return 0;
}

