#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <bsd/stdlib.h>
#include <bsd/sys/tree.h>

typedef struct Node {
    char *key;
    int value;
    RB_ENTRY(Node) entry; // Red-Black Tree Entry
} Node;

RB_HEAD(Tree, Node) rb_tree = RB_INITIALIZER(&rb_tree);

int nodecmp(Node *a, Node *b) {
    return strcmp(a->key, b->key);
}

RB_PROTOTYPE(Tree, Node, entry, nodecmp)
RB_GENERATE(Tree, Node, entry, nodecmp)

typedef struct {
    size_t size;
    size_t capacity;
    struct Tree *trees;
} HashTable;

unsigned int hash_function(const char *key) {
    // Simple hash function for demonstration
    unsigned int hash = 0;
    for (size_t i = 0; key[i] != '\0'; ++i) {
        hash = 31 * hash + key[i];
    }
    return hash;
}

Node *create_node(const char *key, int value) {
    Node *node = (Node *)malloc(sizeof(Node));
    if (node == NULL) {
        fprintf(stderr, "Memory allocation failed.\n");
        exit(EXIT_FAILURE);
    }
    node->key = strdup(key);
    node->value = value;
    return node;
}

void free_node(Node *node) {
    free(node->key);
    free(node);
}

void insert(HashTable *hashTable, const char *key, int value) {
    unsigned int index = hash_function(key) % hashTable->capacity;
    struct Tree *tree = &(hashTable->trees[index]);

    // Check if key already exists
    Node search_node = {.key = (char *)key};
    Node *existing_node = RB_FIND(Tree, tree, &search_node);
    if (existing_node != NULL) {
        // Update value
        existing_node->value = value;
        return;
    }

    Node *new_node = create_node(key, value);
    if (new_node == NULL) {
        fprintf(stderr, "Memory allocation failed.\n");
        exit(EXIT_FAILURE);
    }

    RB_INSERT(Tree, tree, new_node);
    hashTable->size++;
}

int get(HashTable *hashTable, const char *key) {
    unsigned int index = hash_function(key) % hashTable->capacity;
    struct Tree *tree = &(hashTable->trees[index]);

    Node search_node = {.key = (char *)key};
    Node *found_node = RB_FIND(Tree, tree, &search_node);

    if (found_node != NULL) {
        return found_node->value;
    }

    return -1; // Key not found
}

void delete(HashTable *hashTable, const char *key) {
    unsigned int index = hash_function(key) % hashTable->capacity;
    struct Tree *tree = &(hashTable->trees[index]);

    Node search_node = {.key = (char *)key};
    Node *found_node = RB_FIND(Tree, tree, &search_node);

    if (found_node != NULL) {
        RB_REMOVE(Tree, tree, found_node);
        free_node(found_node);
        hashTable->size--;
    }
}

void free_hash_table(HashTable *hashTable) {
    for (size_t i = 0; i < hashTable->capacity; ++i) {
        struct Tree *tree = &(hashTable->trees[i]);
        Node *node;
        while ((node = RB_MIN(Tree, tree)) != NULL) {
            RB_REMOVE(Tree, tree, node);
            free_node(node);
        }
    }
    free(hashTable->trees);
    free(hashTable);
}

HashTable *create_hash_table(size_t capacity) {
    HashTable *hashTable = (HashTable *)malloc(sizeof(HashTable));
    if (hashTable == NULL) {
        fprintf(stderr, "Memory allocation failed.\n");
        exit(EXIT_FAILURE);
    }
    hashTable->size = 0;
    hashTable->capacity = capacity;
    hashTable->trees = (struct Tree *)malloc(capacity * sizeof(struct Tree));
    if (hashTable->trees == NULL) {
        free(hashTable);
        fprintf(stderr, "Memory allocation failed.\n");
        exit(EXIT_FAILURE);
    }
    for (size_t i = 0; i < capacity; ++i) {
        RB_INIT(&(hashTable->trees[i]));
    }
    return hashTable;
}

int main() {
    HashTable *hashTable = create_hash_table(10);

    insert(hashTable, "apple", 5);
    insert(hashTable, "orange", 10);
    insert(hashTable, "banana", 7);

    printf("apple: %d\n", get(hashTable, "apple"));     // Output: apple: 5
    printf("orange: %d\n", get(hashTable, "orange"));   // Output: orange: 10
    printf("banana: %d\n", get(hashTable, "banana"));   // Output: banana: 7
    printf("grape: %d\n", get(hashTable, "grape"));     // Output: grape: -1 (Not found)

    delete(hashTable, "orange");

    printf("orange after deletion: %d\n", get(hashTable, "orange"));   // Output: orange after deletion: -1

    free_hash_table(hashTable);
    return 0;
}
