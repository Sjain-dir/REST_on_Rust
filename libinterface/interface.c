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

extern int connect(int __fd, const struct sockaddr * __addr, socklen_t __len);

//tried to make it non constant, still testing this because refernce is working in it
//extern int connect(int __fd, struct sockaddr * __addr, socklen_t __len);  

extern ssize_t send(int __fd, const void *__buf, size_t __n, int __flags);

//extern size_t strlen(const char *__s); // can use the rust inbuilt function.

int test_interface(char* domain, int portt, char* payload) {
     int status, valread, client_fd;
     struct sockaddr_in serv_addr; // stucture in socket.h

     char* payloadd = "message to the server and can pur any header file here";
     char buffer[1024] = {0};
     printf("payload toh yeh hai : %s\n  \n port is %d \ndoamin is %s\n", payloadd, portt, domain);

     if ((client_fd = socket(AF_INET, SOCK_STREAM, 0)) < 0 ) {
          printf("Error occur in socket creation\n");
          return -1;
     }
     serv_addr.sin_family = AF_INET;
     serv_addr.sin_port = htons(portt);

     if (inet_pton(AF_INET, domain, &serv_addr.sin_addr) <= 0 ) {
          printf("Address not supported\n");
          return -1;
     }
     printf("inet ne yeh diya %d", serv_addr.sin_addr.s_addr);

     if ((status = connect(client_fd, (struct sockaddr*) &serv_addr, sizeof(serv_addr))) < 0) {
          printf("Connection failed\n");
          return -1;
     }

     send(client_fd, payload, strlen(payloadd), 0);

     return 0;
}

//j+-=-------------------just for the test purpose only------------------------

void test_fun(int i) {
     printf("test fun : %d \n\n", i);
}