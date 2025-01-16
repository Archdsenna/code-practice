/* 在 C 语言中，宏定义的先后顺序对于它们的使用和展开是有影响的。
 * 具体来说，一个宏在被使用之前必须先被定义。这意味着如果一个宏依赖于另一个宏，
 * 那么被依赖的宏必须先定义
 */

// 不推荐的写法: 
#define Xprintf_buffer Xprintf (buffer)    // Xprintf (buffer) 也可以写成 Xprintf(buffer),
                                           // 在C语言中,宏调用时参数列表周围的空格是可选的, 这意味着
                                           // 在宏名称和左括号之间是否有空格不会影响宏的功能或其展开方式。
                                           // 在C/C++中,宏的调用看起来很像函数调用,但它们在预处理阶段处理,
                                           // 而不是在编译或运行时,因此以下两种写法是等效的:
                                           //   Xprintf (buffer)
                                           //   Xprintf(buffer)
#define Xprintf(n) __printf__##n
// 虽然上面的写法，预处理器也可以正确将宏展开,原因是预处理器可能会进行多遍扫描,
// 预处理器在第一遍扫描时会记录所有的宏定义，但不立即展开它们。
// 在随后的扫描中，预处理器会使用这些记录的定义来展开宏

// 正确的写法:
//    Xprintf_buffer宏依赖Xprintf(n)宏,所有应当先定义被依赖的宏,如Xprintf(n),
//    然后再定义Xprintf_buffer宏
#define Xprintf(n) __printf__##n
#define Xprintf_buffer Xprintf (buffer)

/* 总结:
 *      虽然在某些情况下，编译器能够处理这种 seemingly out-of-order 的宏定义，
 *      依赖于这种行为并不是一个好的编程实践。为了保证代码的可移植性和避免依赖于特定编译器的行为，
 *      最好还是按照宏的依赖关系来顺序定义宏，即确保任何宏在被引用之前已经被定义。
 *      这样可以避免潜在的跨编译器的兼容性问题，并使代码更加清晰易懂。 */

int main(void)
{
        /* Xprintf_buffer; */
        /* printf("%s", Xprintf_buffer); */
}
