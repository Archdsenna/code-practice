/*
 * 在 C 语言中，scanf 函数是一个非常常用的函数，用于从标准输入（通常是键盘）读取格式化的输入。
 * 它是标准库函数 stdio.h 的一部分，可以读取各种类型的数据，包括整数、浮点数、字符和字符串。
 *
 * 下面是 scanf 的基本用法和一些常见的格式说明符：
 * 基本语法
 * scanf 的基本语法如下：
 *      int scanf(const char *format, ...);
 *      format：一个字符串，包含一个或多个格式说明符，这些说明符定义了如何读取输入的数据。
 *      ...：一个或多个指向变量的指针，这些变量将存储读取的数据。
 *
 *      格式说明符
 *      %d：读取一个十进制整数。
 *      %i：读取一个整数（自动检测十进制、八进制或十六进制）。
 *      %u：读取一个无符号整数。
 *      %f、%lf、%Lf：分别用于读取 float、double 和 long double 类型的浮点数。
 *      %c：读取一个字符。
 *      %s：读取一个字符串，直到遇到空白字符（如空格、制表符或换行符）。
 *      %x、%o：分别用于读取十六进制和八进制数。*JK/
 */

#include <stdio.h>

int main(void)
{
        char word[16];

        printf("Please input a word: ");
        scanf("%15s", word);
        printf("word is %s", word);
}
