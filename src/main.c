#include <arpa/inet.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>

#define PORT 8080
#define BACKLOG 10
#define MAX_BUF 1024

void handle_request(int client_sock) {
    char buffer[MAX_BUF];
    int read_size = 0;

    // Read the incoming request
    memset(buffer, 0, sizeof(buffer));
    read_size = (int)read(client_sock, buffer, sizeof(buffer) - 1);

    if (read_size < 0) {
        perror("Failed to read request");
        close(client_sock);
        return;
    }

    // Only proceed if there is a valid request
    if (read_size > 0) {
        // Prepare a simple "Hello, World!" HTML response
        const char* http_response =
            "HTTP/1.1 200 OK\r\n"
            "Content-Type: text/html; charset=UTF-8\r\n"
            "Connection: close\r\n\r\n"
            "<html><head><title>Hello, World!</title></head>"
            "<body><h1>Hello, World!</h1></body></html>\r\n";

        // Send the response to the client
        write(client_sock, http_response, strlen(http_response));
    }

    // Close the client socket
    close(client_sock);
}

int main() {
    int server_sock = 0;
    int client_sock = 0;
    struct sockaddr_in server_addr;
    struct sockaddr_in client_addr;
    socklen_t client_addr_len = sizeof(client_addr);

    // Create socket
    server_sock = socket(AF_INET, SOCK_STREAM, 0);
    if (server_sock < 0) {
        perror("Socket creation failed");
        exit(EXIT_FAILURE);
    }

    // Set up server address
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(PORT);

    // Bind the socket
    if (bind(server_sock, (struct sockaddr*)&server_addr, sizeof(server_addr)) == -1) {
        perror("Bind failed");
        close(server_sock);
        exit(EXIT_FAILURE);
    }

    // Start listening for connections
    if (listen(server_sock, BACKLOG) == -1) {
        perror("Listen failed");
        close(server_sock);
        exit(EXIT_FAILURE);
    }

    printf("Server listening on port %d...\n", PORT);

    // Main server loop
    while (1) {
        client_sock = accept(server_sock, (struct sockaddr*)&client_addr, &client_addr_len);
        // Accept incoming client connections
        if (client_sock < 0) {
            perror("Accept failed");
            continue;
        }

        // Handle the request
        handle_request(client_sock);
    }

    // Close server socket (although this line is never reached in the infinite loop)
    close(server_sock);

    return 0;
}
