#include <stdio.h>
#include <stdlib.h>
#include <memory.h>
#include <ctype.h>

#define MAX 26
#define STRLEN 10

typedef struct TrieNode
{
    int count;
    struct TrieNode* next[MAX];
}TrieNode, *Trie;

TrieNode* CreateTrieNode()
{
    TrieNode* node = (TrieNode *)malloc(sizeof(TrieNode));
    node->count = 0;
    memset(node->next, 0, sizeof(node->next));
    return node;
}

void InsertTrie(Trie pRoot, char* s)
{
    Trie node = pRoot;
    char* p = s;
    int id;
    while(*p)
    {
        id = *p - 'a';
        if(node->next[id] == NULL)
            node->next[id] = CreateTrieNode();
        node = node->next[id];
        ++p;
        node->count += 1;
    }
}

int SearchTrie(Trie pRoot, char* s)
{
    Trie node = pRoot;
    char* p = s;
    int id;
    while(*p)
    {
        id = *p - 'a';
        node = node->next[id];
        ++p;
        if(node == NULL)
            return 0;
    }
    return node->count;
}

int main()
{
    Trie pRoot = CreateTrieNode();
    char str[STRLEN + 1];
    int n, m;

    scanf("%d", &n);
    while(n--)
    {
        scanf("%s", str);
        InsertTrie(pRoot, str);
    }

    scanf("%d", &m);
    while(m--)
    {
        scanf("%s", str);
        printf("%d\n", SearchTrie(pRoot, str));
    }
    return 0;
}