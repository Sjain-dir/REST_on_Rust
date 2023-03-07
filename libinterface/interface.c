#include <arpa/inet.h>  
#include <stdio.h>  
#include <string.h>  
#include <sys/socket.h>  
#include <unistd.h>  

// char test (int input) {
//     printf("this is your number %d \n", input);
//     return 'a';
// }

extern struct sockaddr_in;

extern int socket(int domain, int type, int protocol);

extern uint16_t htons (uint16_t __hostshort)
     __THROW __attribute__ ((__const__));

extern int inet_pton (int __af, const char *__restrict __cp,
		      void *__restrict __buf) __THROW;

extern int connect(int __fd, const struct sockaddr *__addr, socklen_t __len);

//extern size_t strlen(const char *__s); // can use the rust inbuilt function.